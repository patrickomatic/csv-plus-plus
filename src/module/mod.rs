//! # Module
//!
//! A `module` holds the final compiled state for a single csv++ source file
//!
// TODO:
// * we need more unit tests around the various eval phases
//      - fills
//      - row vs cell variable definitions
// * eval cells in parallel (rayon)
// * make sure there is only one infinite fill in the docs (ones can follow it, but they have to
//      be finite and subtract from it
use crate::ast::Variables;
use crate::{ArcSourceCode, Compiler, ModuleLoader, ModulePath, Result, Row, Scope, Spreadsheet};
use log::{debug, error, info};
use std::cell;
use std::fs;

mod display;
mod try_from;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Module {
    pub module_path: ModulePath,
    pub scope: Scope,
    pub spreadsheet: cell::RefCell<Spreadsheet>,
    pub compiler_version: String,
    pub(crate) source_code: ArcSourceCode,
}

impl Module {
    pub(crate) fn new(
        source_code: ArcSourceCode,
        module_path: ModulePath,
        scope: Scope,
        spreadsheet: Spreadsheet,
    ) -> Self {
        Self {
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            scope,
            module_path,
            spreadsheet: cell::RefCell::new(spreadsheet),
            source_code,
        }
    }

    /// For each row of the spreadsheet, if it has a [[fill=]] then we need to actually fill it to
    /// that many rows.  
    ///
    /// This has to happen before eval()ing the cells because that process depends on them being in
    /// their final location.
    // TODO: make sure there is only one infinite fill
    // TODO: move this into spreadsheet?
    pub(crate) fn eval_fills(self) -> Self {
        let mut new_spreadsheet = Spreadsheet::default();
        let s = self.spreadsheet.into_inner();
        let mut row_num = 0;

        for row in s.rows.into_iter() {
            if let Some(f) = row.fill {
                let new_fill = f.clone_to_row(row_num);
                for _ in 0..new_fill.fill_amount(row_num) {
                    new_spreadsheet.rows.push(Row {
                        fill: Some(new_fill),
                        ..row.clone()
                    });
                    row_num += 1;
                }
            } else {
                new_spreadsheet.rows.push(row);
                row_num += 1;
            }
        }

        Self {
            spreadsheet: cell::RefCell::new(new_spreadsheet),
            ..self
        }
    }

    // TODO: do this in parallel (thread for each row (maybe cell? with a threadpool))
    pub(crate) fn eval_spreadsheet(self, external_vars: Variables) -> Result<Self> {
        let spreadsheet = self.spreadsheet.into_inner();
        let scope = self
            .scope
            .merge_variables(spreadsheet.variables())
            .merge_variables(external_vars);

        let mut evaled_rows = vec![];
        for (row_index, row) in spreadsheet.rows.into_iter().enumerate() {
            evaled_rows.push(row.eval(self.source_code.clone(), &scope, row_index.into())?);
        }

        Ok(Self {
            scope,
            spreadsheet: cell::RefCell::new(Spreadsheet { rows: evaled_rows }),
            ..self
        })
    }

    pub(crate) fn load_dependencies(self) -> Result<Self> {
        let module_loader = ModuleLoader::load_main(&self.module_path, &self.scope)?;
        let dependencies = module_loader.into_direct_dependencies()?;

        Ok(Self {
            scope: self.scope.merge(dependencies),
            ..self
        })
    }

    pub(crate) fn write_object_file(&self, compiler: &Compiler) -> Result<()> {
        if !compiler.options.use_cache {
            info!("Not writing object file because --no-cache flag is set");
            return Ok(());
        }

        let object_code_filename = self.source_code.object_code_filename();

        debug!("Writing object file");

        let object_file = fs::File::create(object_code_filename).map_err(|e| {
            error!("IO error: {e:?}");
            self.source_code
                .object_code_error(format!("Error opening object code for writing: {e}"))
        })?;

        serde_cbor::to_writer(object_file, self).map_err(|e| {
            error!("CBOR write error: {e:?}");
            self.source_code
                .object_code_error(format!("Error serializing object code for writing: {e}"))
        })?;

        Ok(())
    }

    /* TODO: bring back object codes (but in a way that works with the module loader)
    pub(crate) fn read_from_object_file(compiler: &Compiler) -> Result<Option<Self>> {
        if !compiler.options.use_cache {
            compiler.info("Not reading object file");
            return Ok(None);
        }

        let sc = &compiler.source_code;
        let obj_file = sc.object_code_filename();

        // does the object code file even exist?
        if !obj_file.exists() {
            compiler.info("Attempted to read object file but it does not exist");
            return Ok(None);
        }

        let obj_file_modified = fs::metadata(&obj_file)
            .and_then(|s| s.modified())
            .map_err(|e| sc.object_code_error(format!("Unable to stat object code: {e}")))?;
        let source_file_modified = fs::metadata(&sc.filename)
            .and_then(|s| s.modified())
            .map_err(|e| sc.object_code_error(format!("Unable to stat source code: {e}")))?;

        // is the object code more recent than the source? (i.e., no changes since it was last
        // written)
        if source_file_modified > obj_file_modified {
            return Ok(None);
        }

        let obj_file_reader = fs::File::open(&obj_file)
            .map_err(|e| sc.object_code_error(format!("Error opening object code: {e}")))?;

        let Ok(loaded_module): std::result::Result<Self, serde_cbor::Error> =
            serde_cbor::from_reader(obj_file_reader)
        else {
            // if we fail to load the old object file just warn about it and move on.  for whatever
            // reason (written by an old version) it's not compatible with our current version
            compiler.warn(format!(
                "Error loading object code from {}.  Was it written with an old version of csv++?",
                obj_file.display()
            ));
            return Ok(None);
        };

        let current_version = env!("CARGO_PKG_VERSION").to_string();
        let this_version = semver::Version::parse(&current_version).map_err(|e| {
            sc.object_code_error(format!("Unable to parse version `{current_version}`: {e}"))
        })?;
        let loaded_version =
            semver::Version::parse(&loaded_module.compiler_version).map_err(|e| {
                sc.object_code_error(format!(
                    "Unable to parse loaded module version `{}`: {e}",
                    &loaded_module.compiler_version
                ))
            })?;

        // if the version is less than ours, don't use it and recompile instead.  otherwise we can
        // trust that it's ok to use
        Ok(match this_version.cmp(&loaded_version) {
            cmp::Ordering::Equal | cmp::Ordering::Greater => Some(loaded_module),
            cmp::Ordering::Less => None,
        })
    }
        */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::test_utils::*;
    use crate::*;
    use std::cell;

    #[test]
    fn eval_fills_finite() {
        let module = Module {
            spreadsheet: cell::RefCell::new(Spreadsheet {
                rows: vec![
                    Row {
                        fill: Some(Fill::new(0, Some(10))),
                        ..Default::default()
                    },
                    Row {
                        fill: Some(Fill::new(10, Some(30))),
                        ..Default::default()
                    },
                ],
            }),
            ..build_module()
        }
        .eval_fills();
        let spreadsheet = module.spreadsheet.borrow();

        assert_eq!(spreadsheet.rows.len(), 40);
        // 0-9 should be Fill { amount: 10, start_row: 0 }
        assert_eq!(spreadsheet.rows[0].fill.unwrap().start_row, 0.into());
        assert_eq!(spreadsheet.rows[9].fill.unwrap().start_row, 0.into());
        // and 10-39 should be Fill { amount: 30, start_row: 10 }
        assert_eq!(spreadsheet.rows[10].fill.unwrap().start_row, 10.into());
        assert_eq!(spreadsheet.rows[39].fill.unwrap().start_row, 10.into());
    }

    #[test]
    fn eval_fills_infinite() {
        let module = Module {
            spreadsheet: cell::RefCell::new(Spreadsheet {
                rows: vec![
                    Row {
                        fill: Some(Fill::new(0, Some(10))),
                        ..Default::default()
                    },
                    Row {
                        fill: Some(Fill::new(10, None)),
                        ..Default::default()
                    },
                ],
            }),
            ..build_module()
        }
        .eval_fills();
        let spreadsheet = module.spreadsheet.borrow();

        assert_eq!(spreadsheet.rows.len(), 1000);
        // 0-9 should be Fill { amount: 10, start_row: 0 }
        assert_eq!(spreadsheet.rows[0].fill.unwrap().start_row, 0.into());
        assert_eq!(spreadsheet.rows[9].fill.unwrap().start_row, 0.into());
        // and 10-999 should be Fill { amount: None, start_row: 10 }
        assert_eq!(spreadsheet.rows[10].fill.unwrap().start_row, 10.into());
        assert_eq!(spreadsheet.rows[999].fill.unwrap().start_row, 10.into());
    }

    #[test]
    fn load_dependencies_with_scope() {
        let mut module = build_module();
        module
            .scope
            .functions
            .insert("foo".to_string(), Ast::new(1.into()));
        module
            .scope
            .variables
            .insert("bar".to_string(), Ast::new(2.into()));
        let module = module.load_dependencies().unwrap();

        assert!(module.scope.functions.contains_key("foo"));
        assert!(module.scope.variables.contains_key("bar"));
    }

    #[test]
    fn load_depdencies_without_scope() {
        let module = build_module();

        assert!(module.scope.functions.is_empty());
        assert!(module.scope.variables.is_empty());
    }
}
