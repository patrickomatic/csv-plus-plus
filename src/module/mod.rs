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
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{
    compiler_error, ArcSourceCode, Error, ModuleLoader, ModulePath, Result, Row, Scope, SourceCode,
    Spreadsheet,
};
use log::{error, info, warn};
use std::cmp;
use std::fs;
use std::path;

mod display;
mod try_from;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Module {
    pub compiler_version: String,
    pub module_path: ModulePath,
    pub scope: Scope,
    pub spreadsheet: Spreadsheet,
    pub required_modules: Vec<ModulePath>,
    pub(crate) source_code: ArcSourceCode,
    pub(crate) is_dirty: bool,
}

impl Module {
    /// For each row of the spreadsheet, if it has a [[fill=]] then we need to actually fill it to
    /// that many rows.  
    ///
    /// This has to happen before eval()ing the cells because that process depends on them being in
    /// their final location.
    // TODO: make sure there is only one infinite fill
    // TODO: move this into spreadsheet?
    pub(crate) fn eval_fills(self) -> Self {
        let mut new_spreadsheet = Spreadsheet::default();
        let s = self.spreadsheet;
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
            spreadsheet: new_spreadsheet,
            ..self
        }
    }

    // TODO: do this in parallel (thread for each row (maybe cell? with a threadpool))
    pub(crate) fn eval_spreadsheet(self, external_vars: Variables) -> Result<Self> {
        let spreadsheet = self.spreadsheet;
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
            spreadsheet: Spreadsheet { rows: evaled_rows },
            ..self
        })
    }

    pub(crate) fn load_dependencies<P: Into<path::PathBuf>>(self, relative_to: P) -> Result<Self> {
        let module_loader = ModuleLoader::load_dependencies(&self, relative_to)?;
        let dependencies = module_loader.into_direct_dependencies()?;

        Ok(Self {
            scope: self.scope.merge(dependencies),
            ..self
        })
    }

    fn load_from_object_code_file<P: AsRef<path::Path>>(
        module_path: ModulePath,
        relative_to: &ModulePath,
        loader_root: P,
    ) -> Option<Self> {
        let mut filename = loader_root
            .as_ref()
            .join(module_path.clone().filename_relative_to(relative_to));
        filename.set_extension("csvpo");

        if !filename.exists() {
            return None;
        }

        let obj_file_reader = match fs::File::open(&filename) {
            Ok(r) => r,
            Err(e) => compiler_error(format!("Error opening object code: {e}")),
        };

        let Ok(loaded_module): std::result::Result<Self, serde_cbor::Error> =
            serde_cbor::from_reader(obj_file_reader)
        else {
            // if we fail to load the old object file just warn about it and move on.  for whatever
            // reason (written by an old version) it's not compatible with our current version
            warn!(
                "Error loading object code from {}.  Was it written with an old version of csv++?",
                filename.display()
            );
            return None;
        };

        Some(loaded_module)
    }

    pub(crate) fn load_from_source<P: Into<path::PathBuf>>(
        module_path: ModulePath,
        filename: P,
    ) -> Result<Self> {
        let source_code = ArcSourceCode::new(SourceCode::try_from(filename.into())?);

        if let Some(scope_source) = &source_code.code_section {
            let (scope, required_modules) =
                CodeSectionParser::parse(scope_source, source_code.clone())?;
            Ok(Module {
                compiler_version: env!("CARGO_PKG_VERSION").to_string(),
                is_dirty: false,
                module_path,
                required_modules,
                scope,
                source_code,
                spreadsheet: Spreadsheet::default(),
            })
        } else {
            Err(Error::ModuleLoadError(
                "This module does not have a code section (but you imported it)".to_string(),
            ))
        }
    }

    pub(crate) fn load_from_cache_or_source<P: AsRef<path::Path>>(
        module_path: ModulePath,
        relative_to: &ModulePath,
        loader_root: P,
    ) -> Result<Self> {
        if let Some(mut loaded_module) =
            Self::load_from_object_code_file(module_path.clone(), relative_to, &loader_root)
        {
            loaded_module.check_if_is_dirty()?;
            Ok(loaded_module)
        } else {
            let filename = loader_root
                .as_ref()
                .join(module_path.clone().filename_relative_to(relative_to));

            Self::load_from_source(module_path, filename)
        }
    }

    pub(crate) fn write_object_file(&self) -> Result<()> {
        /* TODO: bring back no_cache flag
        if !compiler.options.use_cache {
            info!("Not writing object file because --no-cache flag is set");
            return Ok(());
        }
        */

        let object_code_filename = self.source_code.object_code_filename();

        info!("Writing object file to {}", object_code_filename.display());

        let object_file = fs::File::create(&object_code_filename).map_err(|e| {
            error!("IO error: {e:?}");
            Error::SourceCodeError {
                filename: object_code_filename,
                message: format!("Error opening object code for writing: {e}"),
            }
        })?;

        match serde_cbor::to_writer(object_file, self) {
            Err(e) => {
                error!("CBOR write error: {e:?}");
                compiler_error(format!("Error serializing object code for writing: {e}"));
            }
            _ => Ok(()),
        }
    }

    pub fn check_if_is_dirty(&mut self) -> Result<()> {
        let object_code_filename = self.source_code.object_code_filename();

        let obj_file_modified = fs::metadata(&object_code_filename)
            .and_then(|s| s.modified())
            .map_err(|e| Error::SourceCodeError {
                filename: object_code_filename,
                message: format!("Unable to stat object code: {e}"),
            })?;

        let source_file_modified = fs::metadata(&self.source_code.filename)
            .and_then(|s| s.modified())
            .map_err(|e| Error::SourceCodeError {
                filename: self.source_code.filename.clone(),
                message: format!("Unable to stat source code: {e}"),
            })?;

        // is the object code more recent than the source? (i.e., no changes since it was last
        // written)
        if source_file_modified > obj_file_modified {
            self.is_dirty = true;
        }

        let current_version = env!("CARGO_PKG_VERSION");
        let this_version = match semver::Version::parse(current_version) {
            Ok(v) => v,
            Err(e) => compiler_error(format!(
                "Unable to parse compiler version `{current_version}`: {e}"
            )),
        };

        let loaded_version = match semver::Version::parse(&self.compiler_version) {
            Ok(v) => v,
            Err(e) => compiler_error(format!(
                "Unable to parse loaded module version `{}`: {e}",
                &self.compiler_version
            )),
        };

        // if the version is less than ours, don't use it and recompile instead.  otherwise we can
        // trust that it's ok to use
        self.is_dirty = match this_version.cmp(&loaded_version) {
            cmp::Ordering::Equal | cmp::Ordering::Greater => false,
            cmp::Ordering::Less => true,
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn eval_fills_finite() {
        let module = Module {
            spreadsheet: Spreadsheet {
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
            },
            ..build_module()
        }
        .eval_fills();
        let spreadsheet = module.spreadsheet;

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
            spreadsheet: Spreadsheet {
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
            },
            ..build_module()
        }
        .eval_fills();
        let spreadsheet = module.spreadsheet;

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
        let module = module.load_dependencies("").unwrap();

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
