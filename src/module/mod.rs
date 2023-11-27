//! # Module
//!
//! A `module` holds the final compiled state for a single csv++ source file, as well as managing
//! evaluation and scope resolution.
//!
// TODO:
// * we need more unit tests around the various eval phases
//      - fills
//      - row vs cell variable definitions
// * eval cells in parallel (rayon)
// * make sure there is only one infinite fill in the docs (ones can follow it, but they have to
//      be finite and subtract from it
use crate::ast::{Functions, Variables};
use crate::parser::code_section_parser::CodeSection;
use crate::{Compiler, Result, Spreadsheet};
use std::cell;
use std::cmp;
use std::collections;
use std::fs;
use std::path;

mod display;
mod module_name;

pub use module_name::ModuleName;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Module {
    pub functions: Functions,
    pub module_name: ModuleName,
    pub spreadsheet: cell::RefCell<Spreadsheet>,
    pub variables: Variables,
    pub compiler_version: String,
}

impl Module {
    /// Given a parsed code section and spreadsheet section, this function will assemble all of the
    /// available functions and variables.  There are some nuances here because there are a lot of
    /// sources of functions and variables and they're allowed to override each other.
    ///
    /// ## Function Precedence
    ///
    /// Functions are just comprised of what is builtin and what the user puts in the code section.
    /// The code section functions can override builtins so the precedence is (with the lowest
    /// number being the one that is used):
    ///
    /// 1. Functions in the code section
    /// 2. Builtin functions
    ///
    /// ## Variable Precedence
    ///
    /// There are a lot more sources of variables - here is their order of precedence:
    ///
    /// 1. Variables from the -k/--key-values CLI flag
    /// 2. Variables defined in cells
    /// 3. Variables defined in the code section
    /// 4. Builtin variables
    ///
    pub fn new(
        spreadsheet: Spreadsheet,
        code_section: Option<CodeSection>,
        module_name: ModuleName,
    ) -> Self {
        let spreadsheet_vars = spreadsheet.variables();
        let (code_section_vars, code_section_fns) = if let Some(cs) = code_section {
            (cs.variables, cs.functions)
        } else {
            (collections::HashMap::new(), collections::HashMap::new())
        };

        Self {
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            functions: code_section_fns,
            module_name,
            spreadsheet: cell::RefCell::new(spreadsheet),
            variables: code_section_vars
                .into_iter()
                .chain(spreadsheet_vars)
                // .chain(cli_vars.clone())
                .collect(),
        }
    }

    pub(crate) fn write_object_file(&self, compiler: &Compiler) -> Result<path::PathBuf> {
        compiler.progress("Writing object file");

        let object_code_filename = compiler.source_code.object_code_filename();

        let object_file = fs::File::create(&object_code_filename).map_err(|e| {
            compiler.error(format!("IO error: {e:?}"));
            compiler
                .source_code
                .object_code_error(format!("Error opening object code for writing: {e}"))
        })?;

        serde_cbor::to_writer(object_file, self).map_err(|e| {
            compiler.error(format!("CBOR write error: {e:?}"));
            compiler
                .source_code
                .object_code_error(format!("Error serializing object code for writing: {e}"))
        })?;

        Ok(object_code_filename)
    }

    pub(crate) fn read_from_object_file(compiler: &Compiler) -> Result<Option<Self>> {
        let sc = &compiler.source_code;
        let obj_file = sc.object_code_filename();

        // does the object code file even exist?
        if !obj_file.exists() {
            return Ok(None);
        }

        let obj_file_modified = fs::metadata(&obj_file)
            .and_then(|s| s.modified())
            .map_err(|e| sc.object_code_error(format!("Unable to stat object code: {e}")))?;
        let source_file_modified = fs::metadata(&compiler.source_code.filename)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_code_section() {
        let mut functions = collections::HashMap::new();
        functions.insert("foo".to_string(), Box::new(1.into()));
        let mut variables = collections::HashMap::new();
        variables.insert("bar".to_string(), Box::new(2.into()));
        let code_section = CodeSection {
            functions,
            variables,
            ..Default::default()
        };
        let module = Module::new(
            Spreadsheet::default(),
            Some(code_section),
            ModuleName::new("foo"),
        );

        assert!(module.functions.contains_key("foo"));
        assert!(module.variables.contains_key("bar"));
    }

    #[test]
    fn new_without_code_section() {
        let module = Module::new(Spreadsheet::default(), None, ModuleName::new("foo"));

        assert!(module.functions.is_empty());
        assert!(module.variables.is_empty());
    }
}
