use crate::ast::{Ast, AstReferences, BuiltinFunction, BuiltinVariable, Node};
use crate::error::{EvalError, EvalResult};
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{Cell, Compiler, Module, ModuleName, Result, Row, Spreadsheet};
use std::cell;
use std::collections;

impl Compiler {
    pub fn compile(&self) -> Result<Module> {
        Ok(if let Some(t) = Module::read_from_object_file(self)? {
            self.progress("Read module from object file (not compiling)");
            self.info(&t);
            t
        } else {
            self.progress("Compiling module from source code");

            let spreadsheet = Spreadsheet::parse(self)?;
            self.progress("Parsed spreadsheet");

            let code_section = if let Some(code_section_source) = &self.source_code.code_section {
                let cs = CodeSectionParser::parse(code_section_source, self)?;
                self.progress("Parsed code section");
                self.info(&cs);
                Some(cs)
            } else {
                None
            };

            let module_name: ModuleName = self.source_code.filename.clone().try_into()?;
            let compiled_module = self
                .eval(Module::new(spreadsheet, code_section, module_name))
                .map_err(|e| self.source_code.eval_error(&e.message, e.position))?;

            self.progress("Compiled module");
            self.info(&compiled_module);

            self.progress(format!(
                "Writing object file {}",
                self.source_code.object_code_filename().display()
            ));
            compiled_module.write_object_file(self)?;

            compiled_module
        })
    }

    fn eval(&self, module: Module) -> EvalResult<Module> {
        self.progress("Evaluating all cells");
        self.eval_cells(self.eval_fills(module))
    }

    /// For each row of the spreadsheet, if it has a [[fill=]] then we need to actually fill it to
    /// that many rows.  
    ///
    /// This has to happen before eval()ing the cells because that process depends on them being in
    /// their final location.
    // TODO: make sure there is only one infinite fill
    fn eval_fills(&self, module: Module) -> Module {
        let mut new_spreadsheet = Spreadsheet::default();
        let s = module.spreadsheet.borrow_mut();
        let mut row_num = 0;

        for row in s.rows.iter() {
            if let Some(e) = row.fill {
                for _ in 0..e.fill_amount(row_num) {
                    new_spreadsheet.rows.push(row.clone_to_row(row_num.into()));
                    row_num += 1;
                }
            } else {
                new_spreadsheet.rows.push(row.clone_to_row(row_num.into()));
                row_num += 1;
            }
        }

        Module {
            spreadsheet: cell::RefCell::new(new_spreadsheet),
            ..module
        }
    }

    // TODO:
    // * do this in parallel (thread for each cell)
    // * rather than calling `.spreadsheet.borrow()`, call `.spreadsheet.into_inner()` and consume
    //   it so we don't have to clone the `row` later
    fn eval_cells(&self, module: Module) -> EvalResult<Module> {
        let spreadsheet = module.spreadsheet.borrow();

        let mut evaled_rows = vec![];
        for (row_index, row) in spreadsheet.rows.iter().enumerate() {
            evaled_rows.push(self.eval_row(&module, row.clone(), row_index.into())?);
        }

        Ok(Module {
            spreadsheet: cell::RefCell::new(Spreadsheet { rows: evaled_rows }),
            ..module
        })
    }

    /// The idea here is just to keep looping as long as we are making progress eval()ing.
    /// Progress being defined as `.extract_references()` returning the same result twice in a row
    fn eval_ast(
        &self,
        module: &Module,
        ast: &Ast,
        position: a1_notation::Address,
    ) -> EvalResult<Ast> {
        let mut evaled_ast = *ast.clone();
        let mut last_round_refs = AstReferences::default();

        loop {
            let refs = evaled_ast.extract_references(self, module);
            if refs.is_empty() || refs == last_round_refs {
                break;
            }
            last_round_refs = refs.clone();

            evaled_ast = evaled_ast
                .eval_variables(self.resolve_variables(module, &refs.variables, position)?)?
                .eval_functions(&refs.functions, |fn_id, args| {
                    if let Some(function) = module.functions.get(fn_id) {
                        Ok(function.clone())
                    } else if let Some(BuiltinFunction { eval, .. }) =
                        self.builtin_functions.get(fn_id)
                    {
                        Ok(Box::new(eval(position, args)?))
                    } else {
                        Err(EvalError::new(position, "Undefined function: {fn_id}"))
                    }
                })?;
        }

        Ok(Box::new(evaled_ast))
    }

    fn eval_row(&self, module: &Module, row: Row, row_a1: a1_notation::Row) -> EvalResult<Row> {
        let mut cells = vec![];

        for (cell_index, cell) in row.cells.into_iter().enumerate() {
            let cell_a1 = a1_notation::Address::new(cell_index, row_a1.y);
            let evaled_ast = if let Some(ast) = &cell.ast {
                Some(self.eval_ast(module, ast, cell_a1)?)
            } else {
                None
            };

            cells.push(Cell {
                ast: evaled_ast,
                ..cell
            });
        }

        Ok(Row { cells, ..row })
    }

    pub fn is_function_defined(&self, module: &Module, fn_name: &str) -> bool {
        module.functions.contains_key(fn_name) || self.builtin_functions.contains_key(fn_name)
    }

    pub fn is_variable_defined(&self, module: &Module, var_name: &str) -> bool {
        module.variables.contains_key(var_name) || self.builtin_variables.contains_key(var_name)
    }

    /// Variables can all be resolved in one go - we just loop them by name and resolve the ones
    /// that we can and leave the rest alone.
    fn resolve_variables(
        &self,
        module: &Module,
        var_names: &[String],
        position: a1_notation::Address,
    ) -> EvalResult<collections::HashMap<String, Ast>> {
        let mut resolved_vars = collections::HashMap::new();
        for var_name in var_names {
            if let Some(val) = self.resolve_variable(module, var_name, position)? {
                resolved_vars.insert(var_name.to_string(), val);
            }
        }

        Ok(resolved_vars)
    }

    /// Find the value (`Option<Ast>`) for a given variable.  The search order goes (where the
    /// first one is the winner):
    ///
    /// * CLI-provided variables
    /// * User-defined (in the module source code)
    /// * Builtins
    /// * Otherwise `None`
    ///
    fn resolve_variable(
        &self,
        module: &Module,
        var_name: &str,
        position: a1_notation::Address,
    ) -> EvalResult<Option<Ast>> {
        Ok(if let Some(value) = self.options.key_values.get(var_name) {
            Some(value.clone())
        } else if let Some(value) = module.variables.get(var_name) {
            let value_from_var = match &**value {
                Node::Variable { value, .. } => value.clone().into_ast(position),
                n => Box::new(n.clone()),
            };

            Some(value_from_var)
        } else if let Some(BuiltinVariable { eval, .. }) = self.builtin_variables.get(var_name) {
            Some(Box::new(eval(position)?))
        } else {
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use std::cell;

    fn build_module() -> Module {
        Module {
            compiler_version: "v0.0.1".to_string(),
            functions: collections::HashMap::new(),
            module_name: ModuleName("main".to_string()),
            spreadsheet: cell::RefCell::new(Spreadsheet::default()),
            variables: collections::HashMap::new(),
        }
    }

    #[test]
    fn compile_empty() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile();

        assert!(module.is_ok());
    }

    #[test]
    fn compile_simple() {
        let test_file = &TestSourceCode::new("csv", "---\nfoo,bar,baz\n1,2,3");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile().unwrap();

        assert_eq!(module.spreadsheet.borrow().rows.len(), 2);
    }

    #[test]
    fn compile_with_fill_finite() {
        let test_file = &TestSourceCode::new("xlsx", "![[fill=10]]foo,bar,baz");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile().unwrap();

        assert_eq!(module.spreadsheet.borrow().rows.len(), 10);
    }

    #[test]
    fn compile_with_fill_infinite() {
        let test_file = &TestSourceCode::new("xlsx", "![[fill]]foo,bar,baz");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile().unwrap();

        assert_eq!(module.spreadsheet.borrow().rows.len(), 1000);
    }

    #[test]
    fn compile_with_fill_multiple() {
        let test_file = &TestSourceCode::new("xlsx", "![[f=10]]foo,bar,baz\n![[f]]1,2,3");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile().unwrap();

        assert_eq!(module.spreadsheet.borrow().rows.len(), 1000);
    }

    #[test]
    fn compile_with_fill_and_rows() {
        let test_file =
            &TestSourceCode::new("xlsx", "foo,bar,baz\n![[f=2]]foo,bar,baz\none,last,row\n");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile().unwrap();
        let spreadsheet = module.spreadsheet.borrow();

        assert_eq!(spreadsheet.rows.len(), 4);
    }

    #[test]
    fn is_function_defined_true() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler: Compiler = test_file.into();
        let mut module = build_module();
        module
            .functions
            .insert("foo".to_string(), Box::new(42.into()));

        assert!(compiler.is_function_defined(&module, "foo"));
    }

    #[test]
    fn is_function_defined_builtin_true() {
        let test_file = &TestSourceCode::new("csv", "");
        let mut compiler: Compiler = test_file.into();
        compiler.builtin_functions.insert(
            "foo".to_string(),
            BuiltinFunction {
                name: "foo".to_owned(),
                eval: Box::new(|_a1, _args| Ok(42.into())),
            },
        );
        let module = build_module();

        assert!(compiler.is_function_defined(&module, "foo"));
    }

    #[test]
    fn is_variable_defined_true() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler: Compiler = test_file.into();
        let mut module = build_module();
        module
            .variables
            .insert("foo".to_string(), Box::new(42.into()));

        assert!(compiler.is_variable_defined(&module, "foo"));
    }

    #[test]
    fn is_variable_defined_builtin_true() {
        let test_file = &TestSourceCode::new("csv", "");
        let mut compiler: Compiler = test_file.into();
        compiler.builtin_variables.insert(
            "foo".to_string(),
            BuiltinVariable {
                name: "foo".to_owned(),
                eval: Box::new(|_a1| Ok(42.into())),
            },
        );
        let module = build_module();

        assert!(compiler.is_variable_defined(&module, "foo"));
    }
}
