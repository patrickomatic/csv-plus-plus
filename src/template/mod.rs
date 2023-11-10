//! # Template
//!
//! A `template` holds the final compiled state for a single csv++ source file, as well as managing
//! evaluation and scope resolution.
//!
// TODO:
// * we need more unit tests around the various eval phases
//      - fills
//      - row vs cell variable definitions
// * eval cells in parallel (rayon)
// * make sure there is only one infinite fill in the docs (ones can follow it, but they have to
//      be finite and subtract from it
use crate::ast::{
    Ast, AstReferences, BuiltinFunction, BuiltinVariable, Functions, Node, VariableValue, Variables,
};
use crate::error::{EvalError, EvalResult};
use crate::parser::code_section_parser::{CodeSection, CodeSectionParser};
use crate::{Cell, Result, Row, Runtime, Spreadsheet};
use a1_notation::{Address, A1};
use std::cell;
use std::collections;

mod display;
mod template_at_rest;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Template {
    pub functions: Functions,
    pub module: String,
    pub spreadsheet: cell::RefCell<Spreadsheet>,
    pub variables: Variables,
    pub compiler_version: String,
}

impl Template {
    pub fn compile(runtime: &Runtime) -> Result<Self> {
        let spreadsheet = Spreadsheet::parse(runtime)?;

        let code_section = if let Some(code_section_source) = &runtime.source_code.code_section {
            Some(CodeSectionParser::parse(code_section_source, runtime)?)
        } else {
            None
        };

        let compiled_template = Self::new(spreadsheet, code_section, runtime)
            .eval(runtime)
            .map_err(|e| runtime.source_code.eval_error(&e.message, e.position))?;

        runtime.info(&compiled_template);

        Ok(compiled_template)
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
    pub fn new(
        spreadsheet: Spreadsheet,
        code_section: Option<CodeSection>,
        runtime: &Runtime,
    ) -> Self {
        // TODO: need to lift variable resultion (and therefore runtime.options.key_values out)
        let cli_vars = &runtime.options.key_values;
        let spreadsheet_vars = spreadsheet.variables();
        let (code_section_vars, code_section_fns) = if let Some(cs) = code_section {
            (cs.variables, cs.functions)
        } else {
            (collections::HashMap::new(), collections::HashMap::new())
        };

        Self {
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            functions: code_section_fns,
            module: runtime.source_code.module.clone(),
            spreadsheet: cell::RefCell::new(spreadsheet),
            variables: code_section_vars
                .into_iter()
                .chain(spreadsheet_vars)
                .chain(cli_vars.clone())
                .collect(),
        }
    }

    fn eval(self, runtime: &Runtime) -> EvalResult<Self> {
        runtime.progress("Evaluating all cells");
        self.eval_fills().eval_cells(runtime)
    }

    /// For each row of the spreadsheet, if it has an [[fill=]] modifier then we need to actually
    /// fill it to that many rows.  
    ///
    /// This has to happen before eval()ing the cells because that process depends on them being in
    /// their final location.
    // TODO: make sure there is only one infinite fill
    fn eval_fills(self) -> Self {
        let mut new_spreadsheet = Spreadsheet::default();
        let s = self.spreadsheet.borrow_mut();
        let mut row_num = 0;

        for row in s.rows.iter() {
            if let Some(e) = row.modifier.fill {
                for _ in 0..e.fill_amount(row_num) {
                    new_spreadsheet.rows.push(row.clone_to_row(row_num.into()));
                    row_num += 1;
                }
            } else {
                new_spreadsheet.rows.push(row.clone_to_row(row_num.into()));
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
    fn eval_cells(&self, runtime: &Runtime) -> EvalResult<Self> {
        let spreadsheet = self.spreadsheet.borrow();

        let mut evaled_rows = vec![];
        for (row_index, row) in spreadsheet.rows.iter().enumerate() {
            evaled_rows.push(self.eval_row(runtime, row, row_index.into())?);
        }

        Ok(Self {
            compiler_version: self.compiler_version.clone(),
            functions: self.functions.clone(),
            module: self.module.clone(),
            spreadsheet: cell::RefCell::new(Spreadsheet { rows: evaled_rows }),
            variables: self.variables.clone(),
        })
    }

    /// The idea here is just to keep looping as long as we are making progress eval()ing.
    /// Progress being defined as `.extract_references()` returning the same result twice in a row
    fn eval_ast(&self, runtime: &Runtime, ast: &Ast, position: Address) -> EvalResult<Ast> {
        let mut evaled_ast = *ast.clone();
        let mut last_round_refs = AstReferences::default();

        loop {
            let refs = evaled_ast.extract_references(runtime, self);
            if refs.is_empty() || refs == last_round_refs {
                break;
            }
            last_round_refs = refs.clone();

            evaled_ast = evaled_ast
                .eval_variables(self.resolve_variables(runtime, &refs.variables, position)?)?
                .eval_functions(&refs.functions, |fn_id, args| {
                    if let Some(function) = self.functions.get(fn_id) {
                        Ok(function.clone())
                    } else if let Some(BuiltinFunction { eval, .. }) =
                        runtime.builtin_functions.get(fn_id)
                    {
                        Ok(Box::new(eval(position, args)?))
                    } else {
                        Err(EvalError::new(position, "Undefined function: {fn_id}"))
                    }
                })?;
        }

        Ok(Box::new(evaled_ast))
    }

    fn eval_row(&self, runtime: &Runtime, row: &Row, row_a1: a1_notation::Row) -> EvalResult<Row> {
        let mut cells = vec![];

        for (cell_index, cell) in row.cells.iter().enumerate() {
            let cell_a1 = a1_notation::Address::new(cell_index, row_a1.y);
            let evaled_ast = if let Some(ast) = &cell.ast {
                Some(self.eval_ast(runtime, ast, cell_a1)?)
            } else {
                None
            };

            cells.push(Cell {
                ast: evaled_ast,
                modifier: cell.modifier.clone(),
                value: cell.value.clone(),
            });
        }

        Ok(Row {
            cells,
            modifier: row.modifier.clone(),
        })
    }

    pub fn is_function_defined(&self, runtime: &Runtime, fn_name: &str) -> bool {
        self.functions.contains_key(fn_name) || runtime.builtin_functions.contains_key(fn_name)
    }

    pub fn is_variable_defined(&self, runtime: &Runtime, var_name: &str) -> bool {
        self.variables.contains_key(var_name) || runtime.builtin_variables.contains_key(var_name)
    }

    /// Variables can all be resolved in one go - we just loop them by name and resolve the ones
    /// that we can and leave the rest alone.
    fn resolve_variables(
        &self,
        runtime: &Runtime,
        var_names: &[String],
        position: Address,
    ) -> EvalResult<collections::HashMap<String, Ast>> {
        let mut resolved_vars = collections::HashMap::new();
        for var_name in var_names {
            if let Some(val) = self.resolve_variable(runtime, var_name, position)? {
                resolved_vars.insert(var_name.to_string(), val);
            }
        }

        Ok(resolved_vars)
    }

    fn resolve_variable(
        &self,
        runtime: &Runtime,
        var_name: &str,
        position: Address,
    ) -> EvalResult<Option<Ast>> {
        Ok(if let Some(value) = self.variables.get(var_name) {
            let value_from_var = match &**value {
                Node::Variable { value, .. } => {
                    match value {
                        // absolute value, just turn it into a Ast
                        VariableValue::Absolute(address) => (*address).into(),

                        // already an AST, just clone it
                        VariableValue::Ast(ast) => *ast.clone(),

                        // it's relative to an fill - so if it's referenced inside the
                        // fill, it's the value at that location.  If it's outside the fill
                        // it's the range that it represents
                        VariableValue::ColumnRelative { scope, column } => {
                            let scope_a1: A1 = (*scope).into();
                            if scope_a1.contains(&position.into()) {
                                position.with_x(column.x).into()
                            } else {
                                let row_range: A1 = (*scope).into();
                                row_range.with_x(column.x).into()
                            }
                        }

                        VariableValue::Row(row) => {
                            let a1: a1_notation::A1 = (*row).into();
                            a1.into()
                        }

                        VariableValue::RowRelative { scope, .. } => {
                            let scope_a1: A1 = (*scope).into();
                            if scope_a1.contains(&position.into()) {
                                // we're within the scope (fill) so it's the row we're on
                                let row_a1: A1 = position.row.into();
                                row_a1.into()
                            } else {
                                // we're outside the scope (fill), so it represents the entire
                                // range contained by it (the scope)
                                let row_range: A1 = (*scope).into();
                                row_range.into()
                            }
                        }
                    }
                }
                n => n.clone(),
            };

            Some(Box::new(value_from_var))
        } else if let Some(BuiltinVariable { eval, .. }) = runtime.builtin_variables.get(var_name) {
            Some(Box::new(eval(position)?))
        } else {
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestFile;
    use std::cell;

    fn build_template() -> Template {
        Template {
            compiler_version: "v0.0.1".to_string(),
            functions: collections::HashMap::new(),
            module: "main".to_string(),
            spreadsheet: cell::RefCell::new(Spreadsheet::default()),
            variables: collections::HashMap::new(),
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

        assert_eq!(template.spreadsheet.borrow().rows.len(), 2);
    }

    #[test]
    fn compile_with_fill_finite() {
        let test_file = TestFile::new("xlsx", "![[fill=10]]foo,bar,baz");
        let runtime = test_file.into();
        let template = Template::compile(&runtime).unwrap();

        assert_eq!(template.spreadsheet.borrow().rows.len(), 10);
    }

    #[test]
    fn compile_with_fill_infinite() {
        let test_file = TestFile::new("xlsx", "![[fill]]foo,bar,baz");
        let runtime = test_file.into();
        let template = Template::compile(&runtime).unwrap();

        assert_eq!(template.spreadsheet.borrow().rows.len(), 1000);
    }

    #[test]
    fn compile_with_fill_multiple() {
        let test_file = TestFile::new("xlsx", "![[f=10]]foo,bar,baz\n![[f]]1,2,3");
        let runtime = test_file.into();
        let template = Template::compile(&runtime).unwrap();

        assert_eq!(template.spreadsheet.borrow().rows.len(), 1000);
    }

    #[test]
    fn compile_with_fill_and_rows() {
        let test_file = TestFile::new("xlsx", "foo,bar,baz\n![[f=2]]foo,bar,baz\none,last,row\n");
        let runtime = test_file.into();
        let template = Template::compile(&runtime).unwrap();
        let spreadsheet = template.spreadsheet.borrow();

        assert_eq!(spreadsheet.rows.len(), 4);
    }

    #[test]
    fn is_function_defined_true() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut template = build_template();
        template
            .functions
            .insert("foo".to_string(), Box::new(42.into()));

        assert!(template.is_function_defined(&runtime, "foo"));
    }

    #[test]
    fn is_function_defined_builtin_true() {
        let test_file = TestFile::new("csv", "");
        let mut runtime: Runtime = test_file.into();
        runtime.builtin_functions.insert(
            "foo".to_string(),
            BuiltinFunction {
                name: "foo".to_owned(),
                eval: Box::new(|_a1, _args| Ok(42.into())),
            },
        );
        let template = build_template();

        assert!(template.is_function_defined(&runtime, "foo"));
    }

    #[test]
    fn is_variable_defined_true() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut template = build_template();
        template
            .variables
            .insert("foo".to_string(), Box::new(42.into()));

        assert!(template.is_variable_defined(&runtime, "foo"));
    }

    #[test]
    fn is_variable_defined_builtin_true() {
        let test_file = TestFile::new("csv", "");
        let mut runtime: Runtime = test_file.into();
        runtime.builtin_variables.insert(
            "foo".to_string(),
            BuiltinVariable {
                name: "foo".to_owned(),
                eval: Box::new(|_a1| Ok(42.into())),
            },
        );
        let template = build_template();

        assert!(template.is_variable_defined(&runtime, "foo"));
    }

    #[test]
    fn new_with_code_section() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut functions = collections::HashMap::new();
        functions.insert("foo".to_string(), Box::new(1.into()));
        let mut variables = collections::HashMap::new();
        variables.insert("bar".to_string(), Box::new(2.into()));
        let code_section = CodeSection {
            functions,
            variables,
        };
        let template = Template::new(Spreadsheet::default(), Some(code_section), &runtime);

        assert!(template.functions.contains_key("foo"));
        assert!(template.variables.contains_key("bar"));
    }

    #[test]
    fn new_without_code_section() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let template = Template::new(Spreadsheet::default(), None, &runtime);

        assert!(template.functions.is_empty());
        assert!(template.variables.is_empty());
    }
}
