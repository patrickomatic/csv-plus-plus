//! # Template
//!
//! A `template` holds the final compiled state for a single csv++ source file.
//!
use flexbuffers;
use std::cell;
use std::collections;
use std::fmt;

use crate::{Result, Runtime, Spreadsheet, SpreadsheetCell};
use crate::ast::{Ast, Functions, Variables};
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

        template.eval(runtime)
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

            functions: code_section_fns,

            variables: code_section_vars
                .into_iter()
                .chain(spreadsheet_vars)
                .chain(cli_vars.clone())
                .collect(),
        }
    }

    // TODO:
    // * do this in parallel (thread for each cell)
    fn eval(&self, runtime: &Runtime) -> Result<Self> {
        let spreadsheet = self.spreadsheet.borrow();

        let mut evaled_rows = vec![];
        for row in spreadsheet.cells.iter() {
            evaled_rows.push(self.eval_row(&row, runtime)?);
        }

        Ok(Self {
            functions: self.functions.clone(),
            spreadsheet: cell::RefCell::new(Spreadsheet { cells: evaled_rows }),
            variables: self.variables.clone(),
        })
    }

    /// The idea here is just to keep looping as long as we are making progress eval()ing
    fn eval_ast(&self, ast: &Ast, runtime: &Runtime) -> Result<Ast> {
        let mut evaled_ast = ast.clone();
        let mut last_round_refs = vec![];

        loop {
            let refs = self.extract_references(&evaled_ast);
            if refs.is_empty() || refs == last_round_refs {
                break
            }

            last_round_refs = refs;

            // evaled_ast = self.eval_ast_
            // self.eval_ast_functions(

            // TODO
        }

        Ok(evaled_ast)
    }

    fn extract_references(&self, ast: &Ast) -> Vec<String> {
        let refs = vec![];
        // refs.push(
        refs
    }

    fn eval_row(&self, row: &[SpreadsheetCell], runtime: &Runtime) -> Result<Vec<SpreadsheetCell>> {
        let mut evaled_row = vec![];
        for cell in row.iter() {
            let evaled_ast = if let Some(ast) = &cell.ast {
                Some(self.eval_ast(ast, runtime)?)
            } else {
                None
            };

            evaled_row.push(SpreadsheetCell {
                ast: evaled_ast,
                index: cell.index.clone(),
                modifier: cell.modifier.clone(),
                value: cell.value.clone(),
            });
        }

        Ok(evaled_row)
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
