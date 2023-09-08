//! # Template
//!
//! A `template` holds the final compiled state for a single csv++ source file, as well as managing
//! evaluation and scope resolution.
//!
use a1_notation::{A1, Address};
use serde::{Deserialize, Serialize};
use std::cell;
use std::collections;
use std::convert;
use std::fmt;
use std::path;
use std::str::FromStr;
use crate::{
    Error,
    InnerError,
    Result,
    Runtime,
    SourceCode,
    Spreadsheet,
    SpreadsheetCell,
};
use crate::ast::{
    Ast, 
    AstReferences, 
    BuiltinFunction,
    BuiltinVariable,
    Functions,
    Node,
    Variables,
};
use super::code_section_parser::{CodeSection, CodeSectionParser};

#[derive(Debug)]
pub struct Template<'a> {
    pub functions: Functions,
    pub spreadsheet: cell::RefCell<Spreadsheet>,
    pub variables: Variables,
    csv_line_number: usize,
    runtime: &'a Runtime,
}

/// A template stripped down to just it's serializable fields.  This is internal to this module and
/// should be converted as we read from or write to the object files.
#[derive(Deserialize, Serialize)]
struct TemplateAtRest {
    pub functions: Functions,
    pub spreadsheet: Spreadsheet,
    pub variables: Variables,
    csv_line_number: usize,
}

impl convert::From<&Template<'_>> for TemplateAtRest {
    fn from(template: &Template) -> Self {
        TemplateAtRest { 
            functions: template.functions.clone(),
            spreadsheet: template.spreadsheet.borrow().clone(),
            variables: template.variables.clone(), 
            csv_line_number: template.csv_line_number,
        }
    }
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
        let spreadsheet = Spreadsheet::parse(&runtime.source_code)?;

        let code_section = if let Some(code_section_source) = &runtime.source_code.code_section {
            Some(CodeSectionParser::parse(code_section_source, &runtime.source_code)?)
        } else {
            None
        };

        Self::new(spreadsheet, code_section, runtime).expand().eval()
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
            csv_line_number: runtime.source_code.length_of_code_section + 1,
            spreadsheet: cell::RefCell::new(spreadsheet),
            functions: code_section_fns,
            variables: code_section_vars
                .into_iter()
                .chain(spreadsheet_vars)
                .chain(cli_vars.clone())
                .collect(),
        }
    }

    /// For each row of the spreadsheet, if it has an [[expand=]] modifier then we need to actually
    /// expand it to that many rows.  
    ///
    /// This has to happen before eval()ing the cells because that process depends on them being in
    /// their final location.
    fn expand(self) -> Self {
        let mut new_spreadsheet = Spreadsheet::default();
        let s = self.spreadsheet.borrow_mut();
        let mut row_num = 0;

        for row in s.cells.iter() {
            if let Some(cell) = row.first() {
                if let Some(e) = &cell.expand() {
                    let expand_amount = e.expand_amount(row_num);
                    for _ in 0..expand_amount {
                        new_spreadsheet.cells.push(
                            row.iter()
                                .map(|c| c.clone_to_row(row_num.into()))
                                .collect());
                        row_num += 1;
                    }
                } else {
                    new_spreadsheet.cells.push(row.clone());
                    row_num += 1;
                }
            } else {
                new_spreadsheet.cells.push(row.clone());
                row_num += 1;
            }
        }

        Self {
            spreadsheet: cell::RefCell::new(new_spreadsheet),
            ..self
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
            csv_line_number: self.csv_line_number,
            functions: self.functions.clone(),
            runtime: self.runtime,
            spreadsheet: cell::RefCell::new(Spreadsheet { cells: evaled_rows }),
            variables: self.variables.clone(),
        })
    }

    /// The idea here is just to keep looping as long as we are making progress eval()ing.
    /// Progress being defined as `.extract_references()` returning the same result twice in a row
    fn eval_ast(&self, ast: &Ast, position: Address) -> Result<Ast> {
        let mut evaled_ast = *ast.clone();
        let mut last_round_refs = AstReferences::default();

        loop {
            let refs = evaled_ast.extract_references(self);
            if refs.is_empty() || refs == last_round_refs {
                break
            }
            last_round_refs = refs.clone();

            evaled_ast = evaled_ast
                .eval_variables(self.resolve_variables(refs.variables, position)?)
                .map_err(|e| self.inner_error_to_error(e, position))?
                .eval_functions(&refs.functions, |fn_id, args| {
                    if let Some(function) = self.functions.get(fn_id) {
                        Ok(function.clone())
                    } else if let Some(BuiltinFunction { eval, .. }) = self.runtime.builtin_functions.get(fn_id) {
                        Ok(Box::new(eval(position, &args)?))
                    } else {
                        Err(InnerError::bad_input(fn_id, "Could not find function"))
                    }
                })
                .map_err(|e| self.inner_error_to_error(e, position))?;
        }

        Ok(Box::new(evaled_ast))
    }

    fn eval_row(&self, row: &[SpreadsheetCell]) -> Result<Vec<SpreadsheetCell>> {
        let mut evaled_row = vec![];
        for cell in row.iter() {
            let evaled_ast = if let Some(ast) = &cell.ast {
                Some(self.eval_ast(ast, cell.position)?)
            } else {
                None
            };

            evaled_row.push(SpreadsheetCell {
                ast: evaled_ast,
                position: cell.position,
                modifier: cell.modifier.clone(),
                row_modifier: cell.row_modifier.clone(),
                value: cell.value.clone(),
            });
        }

        Ok(evaled_row)
    }

    fn inner_error_to_error(
        &self,
        inner_error: InnerError,
        position: Address,
    ) -> Error {
        let line_number = self.csv_line_number + position.row.y;
        Error::EvalError {
            message: inner_error.to_string(),
            position,
            line_number,
        }
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
    fn resolve_variables(
        &self,
        var_names: Vec<String>,
        position: Address,
    ) -> Result<collections::HashMap<String, Ast>> {
        let mut resolved_vars = collections::HashMap::new();
        for var_name in var_names {
            if let Some(val) = self.resolve_variable(&var_name, position)? {
                resolved_vars.insert(var_name, val);
            }
        }

        Ok(resolved_vars)
    }

    fn resolve_variable(&self, var_name: &str, position: Address) -> Result<Option<Ast>> {
        Ok(
            if let Some(value) = self.variables.get(var_name) {
                let value_from_var = match &**value {
                    Node::Variable { body, scope, .. } => {
                        if let Some(expand) = scope {
                            let pos: A1 = (*expand).into();
                            if pos.contains(&position.into()) {
                                // XXX store the address more directly, don't do this parsing every
                                // time we resolve...
                                let address = Address::from_str(&body.to_string()).unwrap();
                                address.with_y(position.row.y).into()
                            } else {
                                return Err(Error::InitError(
                                        format!("Variable `{var_name}` can only be referenced inside the `expand` where it was defined")))

                                /* XXX return a proper error
                                return Err(Error::VariableOutOfScope {
                                    message: format!("Variable `{var_name}` can only be referenced inside the `expand` where it was defined"),
                                    position: position.clone(),
                                    line_number: 100,
                                })
                                */
                            }
                        } else {
                            *body.clone()
                        }
                    },
                    n => n.clone(),
                };

                Some(Box::new(value_from_var))
            } else if let Some(BuiltinVariable { eval, .. }) = self.runtime.builtin_variables.get(var_name) {
                Some(Box::new(
                        eval(position).map_err(|e| 
                                               self.inner_error_to_error(e, position))?))
            } else {
                None
            }
        )
    }

    /* TODO: read and use object files for linking
    fn from_template_at_rest(&self) -> Self {
        todo!()
    }
    */

    pub fn write_object_file(&self, source_code: &SourceCode) -> Result<path::PathBuf> {
        let object_code_filename = source_code.object_code_filename();
        /* TODO spend some more time thinking about what would be a good representation
        // let mut s = flexbuffers::FlexbufferSerializer::new();

        let template_at_rest = TemplateAtRest::from(self);
        // let serializer = template_at_rest.serialize(&mut s).unwrap();
        let file = fs::File::create(&object_code_filename).unwrap();
        let writer = ciborium::into_writer(&template_at_rest, &file).unwrap();
        fs::write(&object_code_filename, writer).map_err(|e| {
            Error::ObjectWriteError { 
                filename: object_code_filename.clone(),
                message: format!("Error writing object file: {}", e),
            }
        })?;
        */

        Ok(object_code_filename)
    }
}

#[cfg(test)]
mod tests {
    use std::cell;
    use super::*;
    use crate::test_utils::TestFile;

    fn build_template(runtime: &Runtime) -> Template {
        Template {
            csv_line_number: 5,
            functions: collections::HashMap::new(),
            variables: collections::HashMap::new(),
            runtime,
            spreadsheet: cell::RefCell::new(Spreadsheet::default()),
        }
    }

    #[test]
    fn compile_empty() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let template = Template::compile(&runtime);

        assert!(template.is_ok());
    }

    #[test]
    fn compile_simple() {
        let test_file = TestFile::new("csv", "---\nfoo,bar,baz\n1,2,3");
        let runtime = test_file.into();
        let template = Template::compile(&runtime).unwrap();

        assert_eq!(template.spreadsheet.borrow().cells.len(), 2);
    }

    #[test]
    fn compile_with_expand_finite() {
        let test_file = TestFile::new("xlsx", "![[expand=10]]foo,bar,baz");
        let runtime = test_file.into();
        let template = Template::compile(&runtime).unwrap();

        assert_eq!(template.spreadsheet.borrow().cells.len(), 10);
    }
    
    #[test]
    fn compile_with_expand_infinite() {
        let test_file = TestFile::new("xlsx", "![[expand]]foo,bar,baz");
        let runtime = test_file.into();
        let template = Template::compile(&runtime).unwrap();

        assert_eq!(template.spreadsheet.borrow().cells.len(), 1000);
    }

    #[test]
    fn compile_with_expand_multiple() {
        let test_file = TestFile::new("xlsx", "![[e=10]]foo,bar,baz\n![[e]]1,2,3");
        let runtime = test_file.into();
        let template = Template::compile(&runtime).unwrap();

        assert_eq!(template.spreadsheet.borrow().cells.len(), 1000);
    }

    #[test]
    fn display() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let template = build_template(&runtime);

        assert_eq!(r#"variables: {}
functions: {}
rows: 0"#, template.to_string());
   }

    #[test]
    fn is_function_defined_true() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut template = build_template(&runtime);
        template.functions.insert("foo".to_string(), Box::new(42.into()));

        assert!(template.is_function_defined("foo"));
    }

    #[test]
    fn is_function_defined_builtin_true() {
        let test_file = TestFile::new("csv", "");
        let mut runtime: Runtime = test_file.into();
        runtime.builtin_functions.insert("foo".to_string(), BuiltinFunction {
            name: "foo".to_owned(),
            eval: Box::new(|_a1, _args| Ok(42.into()))
        });
        let template = build_template(&runtime);

        assert!(template.is_function_defined("foo"));
    }

    #[test]
    fn is_variable_defined_true() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut template = build_template(&runtime);
        template.variables.insert("foo".to_string(), Box::new(42.into()));

        assert!(template.is_variable_defined("foo"));
    }

    #[test]
    fn is_variable_defined_builtin_true() {
        let test_file = TestFile::new("csv", "");
        let mut runtime: Runtime = test_file.into();
        runtime.builtin_variables.insert("foo".to_string(), BuiltinVariable {
            name: "foo".to_owned(),
            eval: Box::new(|_a1| Ok(42.into()))
        });
        let template = build_template(&runtime);

        assert!(template.is_variable_defined("foo"));
    }

    #[test]
    fn new_with_code_section() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut functions = collections::HashMap::new();
        functions.insert("foo".to_string(), Box::new(1.into()));
        let mut variables = collections::HashMap::new();
        variables.insert("bar".to_string(), Box::new(2.into()));
        let code_section = CodeSection { functions, variables };
        let template = Template::new(
            Spreadsheet::default(),
            Some(code_section),
            &runtime);

        assert!(template.functions.contains_key("foo"));
        assert!(template.variables.contains_key("bar"));
    }
    
    #[test]
    fn new_without_code_section() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let template = Template::new(
            Spreadsheet::default(),
            None,
            &runtime);

        assert!(template.functions.is_empty());
        assert!(template.variables.is_empty());
    }
}
