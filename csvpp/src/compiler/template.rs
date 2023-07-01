//! # Template
//!
//! A `template` holds the final compiled state for a single csv++ source file.
//!
use flexbuffers;
use std::cell;
use std::collections;
use std::fmt;

use crate::{Result, Runtime, Spreadsheet};
use crate::ast::{BuiltinFunction, BuiltinVariable, Functions, Variables};
use super::code_section_parser::{CodeSection, CodeSectionParser};

#[derive(Debug)]
pub struct Template {
    pub functions: Functions,
    pub spreadsheet: cell::RefCell<Spreadsheet>,
    pub variables: Variables,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "variables: {:?}", &self.variables)?;
        writeln!(f, "functions: {:?}", &self.functions)?;
        write!(f, "rows: {}", self.spreadsheet.borrow().cells.len())
    }
}

impl Default for Template {
    fn default() -> Self {
        Self {
            functions: collections::HashMap::new(),
            spreadsheet: cell::RefCell::new(Spreadsheet::default()),
            variables: collections::HashMap::new(),
        }
    }
}

impl Template {
    pub fn compile(runtime: &Runtime) -> Result<Self> {
        let spreadsheet = Spreadsheet::parse(runtime)?;

        let code_section = if let Some(code_section_source) = &runtime.source_code.code_section {
            Some(CodeSectionParser::parse(code_section_source, &runtime.token_library)?)
        } else {
            None
        };

        let template = Self::new(spreadsheet, code_section, runtime);

        template.resolve_cell_variables(runtime)
    }

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
    fn new(spreadsheet: Spreadsheet, code_section: Option<CodeSection>, runtime: &Runtime) -> Self {
        let cli_vars = &runtime.options.key_values;
        let spreadsheet_vars = spreadsheet.variables();
        let (code_section_vars, code_section_fns) = if let Some(cs) = code_section {
            (cs.variables, cs.functions)
        } else {
            (collections::HashMap::new(), collections::HashMap::new())
        };

        Self {
            spreadsheet: cell::RefCell::new(spreadsheet),

            functions: BuiltinFunction::all().into_iter()
                .chain(code_section_fns)
                .collect(),

            variables: BuiltinVariable::all().into_iter()
                .chain(code_section_vars)
                .chain(spreadsheet_vars)
                .chain(cli_vars.clone())
                .collect(),
        }
    }

    fn resolve_cell_variables(self, _runtime: &Runtime) -> Result<Self> {
        // TODO
        Ok(self)
    }

    // TODO hmm should this just move onto impl Runtime rather than taking a runtime
    pub fn write_object_code(runtime: &Runtime) {
        let _object_code_filename = runtime.source_code.object_code_filename();
        let mut _s = flexbuffers::FlexbufferSerializer::new();
        // runtime.template.serialize(&mut s).unwrap();
        // TODO: write `s` to a file
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        // TODO
    }

    #[test]
    fn display() {
        let template = Template::default();
        assert_eq!(r#"variables: {}
functions: {}
rows: 0"#, template.to_string());
    }
}
