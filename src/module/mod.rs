//! # Module
//!
//! A `module` holds the final compiled state for a single csv++ source file
//!
// TODO:
// * we need more unit tests around the various eval phases
//      - fills
//      - row vs cell variable definitions
// * make sure there is only one infinite fill in the docs (ones can follow it, but they have to
//      be finite and subtract from it
use crate::ast::Variables;
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{
    compiler_error, ArcSourceCode, Error, ModulePath, Result, Scope, SourceCode, Spreadsheet,
};
use log::{debug, error, info, warn};
use rayon::prelude::*;
use std::{cmp, fs, path};

mod display;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Module {
    pub compiler_version: String,
    pub module_path: ModulePath,
    pub scope: Scope,
    pub spreadsheet: Spreadsheet,
    pub required_modules: Vec<ModulePath>,
    pub(crate) source_code: ArcSourceCode,
    pub(crate) is_dirty: bool,
    pub(crate) needs_eval: bool,
}

impl Module {
    pub(crate) fn eval_fills(self) -> Result<Self> {
        Ok(Self {
            spreadsheet: self
                .spreadsheet
                .eval_fills()
                .map_err(|e| self.source_code.eval_error(e, None))?,
            ..self
        })
    }

    pub(crate) fn eval_spreadsheet(self, external_vars: Variables) -> Result<Self> {
        let spreadsheet = self.spreadsheet;
        let scope = self
            .scope
            .merge_variables(spreadsheet.variables())
            .merge_variables(external_vars);

        let rows = spreadsheet
            .rows
            .into_par_iter()
            .enumerate()
            .map(|(row_index, row)| row.eval(&self.source_code, &scope, row_index.into()))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            scope,
            spreadsheet: Spreadsheet {
                rows,
                ..spreadsheet
            },
            ..self
        })
    }

    fn load_from_object_code<P: AsRef<path::Path>>(
        module_path: ModulePath,
        relative_to: &ModulePath,
        loader_root: P,
    ) -> Option<Self> {
        let filename = loader_root
            .as_ref()
            .join(module_path.filename_relative_to(relative_to));
        Self::load_from_object_code_from_filename(filename)
    }

    fn load_from_object_code_from_filename(mut filename: path::PathBuf) -> Option<Self> {
        filename.set_extension("csvpo");

        if !filename.exists() {
            return None;
        }

        let obj_file_reader = match fs::File::open(&filename) {
            Ok(r) => r,
            Err(e) => compiler_error(format!("Error opening object code: {e}")),
        };

        let Ok(loaded_module) = bincode::deserialize_from(obj_file_reader) else {
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

    // TODO: I would love to cut down on all these load_from_* functions
    pub(crate) fn load_from_source_relative<P: Into<path::PathBuf>>(
        module_path: ModulePath,
        relative_to: &ModulePath,
        loader_root: P,
    ) -> Result<Self> {
        Self::load_from_source_from_filename(
            module_path.clone(),
            loader_root
                .into()
                .join(module_path.filename_relative_to(relative_to)),
        )
    }

    pub(crate) fn load_from_source_from_filename(
        module_path: ModulePath,
        filename: path::PathBuf,
    ) -> Result<Self> {
        info!("Loading {module_path} from source: {}", filename.display());

        let source_code = ArcSourceCode::new(SourceCode::try_from(filename)?);

        let (scope, required_modules) = if let Some(scope_source) = &source_code.code_section {
            CodeSectionParser::parse(scope_source, source_code.clone())?
        } else {
            (Scope::default(), Vec::default())
        };

        let spreadsheet = Spreadsheet::parse(&source_code)?;

        Ok(Module {
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            is_dirty: false,
            needs_eval: true,
            module_path,
            required_modules,
            scope,
            source_code,
            spreadsheet,
        })
    }

    pub(crate) fn load_from_cache_from_filename(
        module_path: ModulePath,
        filename: path::PathBuf,
    ) -> Result<Self> {
        if let Some(mut loaded_module) = Self::load_from_object_code_from_filename(filename.clone())
        {
            loaded_module.check_if_is_dirty()?;

            // if we have a clean cache read we can just return that, otherwise we'll need to
            // reload below
            if !loaded_module.is_dirty {
                return Ok(loaded_module);
            }
        }

        Self::load_from_source_from_filename(module_path, filename)
    }

    pub(crate) fn load_from_cache<P: AsRef<path::Path>>(
        module_path: ModulePath,
        relative_to: &ModulePath,
        loader_root: P,
    ) -> Result<Self> {
        if let Some(mut loaded_module) =
            Self::load_from_object_code(module_path.clone(), relative_to, &loader_root)
        {
            loaded_module.check_if_is_dirty()?;
            Ok(loaded_module)
        } else {
            Self::load_from_source_relative(
                module_path,
                relative_to,
                loader_root.as_ref().to_path_buf().clone(),
            )
        }
    }

    pub(crate) fn write_object_file(&mut self) -> Result<()> {
        let object_code_filename = self.source_code.object_code_filename();

        debug!("Serializing {}", self.module_path);
        let encoded = bincode::serialize(&self).map_err(|e| Error::SourceCodeError {
            filename: object_code_filename.clone(),
            message: format!("Error serializing object code: {e}"),
        })?;

        info!("Writing object file to {}", object_code_filename.display());

        fs::write(&object_code_filename, encoded).map_err(|e| {
            error!("IO error: {e:?}");
            Error::SourceCodeError {
                filename: object_code_filename,
                message: format!("Error opening object code for writing: {e}"),
            }
        })?;

        Ok(())
    }

    /// In the case that `self` was deserialized from an object file (`.csvpo`), we need to see if
    /// the original source code could possibly have any updates that we don't have.  In other
    /// words does the source code have changes since this was compiled?
    ///
    /// # Errors
    ///
    /// * `Error::SourceCodeError` - if unable to stat the source code file
    /// * `Error::SourceCodeError` - if unable to stat the cached object code file
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
            self.needs_eval = true;
            return Ok(());
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
    // TODO
}
