//! # Template
//!
//! A `template` holds the final compiled state for a single csv++ source file, as well as managing
//! evaluation and scope resolution.
//!
// TODO: 
//
// * maybe rename this to Scope?
use a1_notation;
use flexbuffers;
use std::cell;
use std::collections;
use std::fmt;
use crate::{Error, Result, Runtime, Spreadsheet, SpreadsheetCell};
use crate::ast::{Ast, AstReferences, BuiltinFunction, BuiltinVariable, Functions, Variables};
use super::code_section_parser::{CodeSection, CodeSectionParser};

#[derive(Debug)]
pub struct Template<'a> {
    pub functions: Functions,
    pub spreadsheet: cell::RefCell<Spreadsheet>,
    pub variables: Variables,
    runtime: &'a Runtime,
}

impl fmt::Display for Template<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "variables: {:?}", &self.variables)?;
        writeln!(f, "functions: {:?}", &self.functions)?;
        write!(f, "rows: {}", self.spreadsheet.borrow().cells.len())
    }
}

impl<'a> Template<'a> {
    pub fn compile(runtime: &'a Runtime) -> Result<Self> {
        let spreadsheet = Spreadsheet::parse(runtime)?;

        let code_section = if let Some(code_section_source) = &runtime.source_code.code_section {
            Some(CodeSectionParser::parse(code_section_source, &runtime.token_library)?)
        } else {
            None
        };

        Self::new(spreadsheet, code_section, runtime).eval()
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
    pub fn new(spreadsheet: Spreadsheet, code_section: Option<CodeSection>, runtime: &'a Runtime) -> Self {
        let cli_vars = &runtime.options.key_values;
        let spreadsheet_vars = spreadsheet.variables();
        let (code_section_vars, code_section_fns) = if let Some(cs) = code_section {
            (cs.variables, cs.functions)
        } else {
            (collections::HashMap::new(), collections::HashMap::new())
        };

        Self {
            runtime,
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
    fn eval(&self) -> Result<Self> {
        let spreadsheet = self.spreadsheet.borrow();

        let mut evaled_rows = vec![];
        for row in spreadsheet.cells.iter() {
            evaled_rows.push(self.eval_row(row)?);
        }

        Ok(Self {
            functions: self.functions.clone(),
            runtime: self.runtime,
            spreadsheet: cell::RefCell::new(Spreadsheet { cells: evaled_rows }),
            variables: self.variables.clone(),
        })
    }

    /// The idea here is just to keep looping as long as we are making progress eval()ing.
    /// Progress being defined as `.extract_references()` returning the same result twice in a row
    fn eval_ast(&self, ast: &Ast, index: &a1_notation::A1) -> Result<Ast> {
        let mut evaled_ast = *ast.clone();
        let mut last_round_refs = AstReferences::default();

        loop {
            let refs = evaled_ast.extract_references(self);
            if refs.is_empty() || refs == last_round_refs {
                break
            }
            last_round_refs = refs.clone();

            evaled_ast = evaled_ast
                .eval_variables(self.resolve_variables(refs.variables, index)?)?
                .eval_functions(refs.functions, |fn_id, args| {
                    if let Some(function) = self.functions.get(fn_id) {
                        Ok(function.clone())
                    } else if let Some(BuiltinFunction { eval, .. }) = self.runtime.builtin_functions.get(fn_id) {
                        Ok(Box::new(eval(index, &args)?))
                    } else {
                        Err(Error::CodeSyntaxError {
                            bad_input: fn_id.to_string(),
                            line_number: 0, // XXX
                            message: "Could not find function".to_owned(),
                        })
                    }
                })?;
        }

        Ok(Box::new(evaled_ast))
    }

    fn eval_row(&self, row: &[SpreadsheetCell]) -> Result<Vec<SpreadsheetCell>> {
        let mut evaled_row = vec![];
        for cell in row.iter() {
            let evaled_ast = if let Some(ast) = &cell.ast {
                Some(self.eval_ast(ast, &cell.index)?)
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

    pub fn is_function_defined(&self, fn_name: &str) -> bool {
        self.functions.contains_key(fn_name)
            || self.runtime.builtin_functions.contains_key(fn_name) 
    }

    pub fn is_variable_defined(&self, var_name: &str) -> bool {
        self.variables.contains_key(var_name)
            || self.runtime.builtin_variables.contains_key(var_name) 
    }

    /// Variables can all be resolved in one go - we just loop them by name and resolve the ones
    /// that we can and leave the rest alone.
    fn resolve_variables(&self, var_names: Vec<String>, index: &a1_notation::A1) -> Result<collections::HashMap<String, Ast>> {
        let mut resolved_vars = collections::HashMap::new();

        for var_name in var_names {
            if let Some(val) = self.resolve_variable(&var_name, index)? {
                resolved_vars.insert(var_name, val);
            }
        }

        Ok(resolved_vars)
    }

    fn resolve_variable(&self, var_name: &str, index: &a1_notation::A1) -> Result<Option<Ast>> {
        Ok(
            if let Some(value) = self.variables.get(var_name) {
                Some(value.to_owned())
            } else if let Some(BuiltinVariable { eval, .. }) = self.runtime.builtin_variables.get(var_name) {
                Some(Box::new(eval(index)?))
            } else {
                None
            }
        )
    }

    // TODO hmm should this just move onto impl Runtime rather than taking a runtime
    pub fn write_object_code(runtime: &Runtime) {
        let _object_code_filename = runtime.source_code.object_code_filename();
        let mut _s = flexbuffers::FlexbufferSerializer::new();
        // self.serialize(&mut s).unwrap();
        // TODO: write `s` to a file
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn new() {
        // TODO
    }

    /*
    #[test]
    fn display() {
        let template = Template::default();
        assert_eq!(r#"variables: {}
functions: {}
rows: 0"#, template.to_string());
   k}
    */

    #[test]
    fn is_function_defined_true() {
        // TODO
    }

    #[test]
    fn is_variable_defined_true() {
        // TODO
    }

    #[test]
    fn resolve_function() {
        // TODO
    }

    #[test]
    fn resolve_variable() {
        // TODO
    }
}
