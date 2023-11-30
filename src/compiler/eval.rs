use crate::ast::{Ast, AstReferences, Functions, Node, Variables};
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
            let compiled_module = self.eval(Module::new(spreadsheet, code_section, module_name));

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

    fn eval(&self, module: Module) -> Module {
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
    fn eval_cells(&self, module: Module) -> Module {
        let spreadsheet = module.spreadsheet.into_inner();

        let mut evaled_rows = vec![];
        for (row_index, row) in spreadsheet.rows.into_iter().enumerate() {
            evaled_rows.push(self.eval_row(
                &module.functions,
                &module.variables,
                row,
                row_index.into(),
            ));
        }

        Module {
            spreadsheet: cell::RefCell::new(Spreadsheet { rows: evaled_rows }),
            ..module
        }
    }

    /// The idea here is just to keep looping as long as we are making progress eval()ing.
    /// Progress being defined as `.extract_references()` returning the same result twice in a row
    fn eval_ast(
        &self,
        module_fns: &Functions,
        module_vars: &Variables,
        ast: &Ast,
        position: a1_notation::Address,
    ) -> Ast {
        let mut evaled_ast = ast.clone();
        let mut last_round_refs = AstReferences::default();

        loop {
            let refs = evaled_ast.extract_references(self, module_fns, module_vars);
            if refs.is_empty() || refs == last_round_refs {
                break;
            }
            last_round_refs = refs.clone();

            evaled_ast = Box::new(
                evaled_ast
                    .eval_variables(self.resolve_variables(module_vars, &refs.variables, position))
                    .eval_functions(&refs.functions, module_fns),
            );
        }

        evaled_ast
    }

    fn eval_row(
        &self,
        module_fns: &Functions,
        module_vars: &Variables,
        row: Row,
        row_a1: a1_notation::Row,
    ) -> Row {
        let mut cells = vec![];
        for (cell_index, cell) in row.cells.into_iter().enumerate() {
            let cell_a1 = a1_notation::Address::new(cell_index, row_a1.y);
            cells.push(Cell {
                ast: cell
                    .ast
                    .as_ref()
                    .map(|ast| self.eval_ast(module_fns, module_vars, ast, cell_a1)),
                ..cell
            });
        }

        Row { cells, ..row }
    }

    pub fn is_function_defined(&self, module_fns: &Functions, fn_name: &str) -> bool {
        module_fns.contains_key(fn_name)
    }

    pub fn is_variable_defined(&self, module_vars: &Variables, var_name: &str) -> bool {
        module_vars.contains_key(var_name)
    }

    /// Variables can all be resolved in one go - we just loop them by name and resolve the ones
    /// that we can and leave the rest alone.
    fn resolve_variables(
        &self,
        module_vars: &Variables,
        var_names: &[String],
        position: a1_notation::Address,
    ) -> collections::HashMap<String, Ast> {
        let mut resolved_vars = collections::HashMap::new();
        for var_name in var_names {
            if let Some(val) = self.resolve_variable(module_vars, var_name, position) {
                resolved_vars.insert(var_name.to_string(), val);
            }
        }

        resolved_vars
    }

    /// Find the value (`Option<Ast>`) for a given variable.  The search order goes (where the
    /// first one is the winner):
    ///
    /// * CLI-provided variables
    /// * User-defined (in the module source code)
    /// * Otherwise `None`
    ///
    fn resolve_variable(
        &self,
        module_vars: &Variables,
        var_name: &str,
        position: a1_notation::Address,
    ) -> Option<Ast> {
        if let Some(value) = self.options.key_values.get(var_name) {
            Some(value.clone())
        } else if let Some(value) = module_vars.get(var_name) {
            let value_from_var = match &**value {
                Node::Variable { value, .. } => value.clone().into_ast(position),
                n => Box::new(n.clone()),
            };

            Some(value_from_var)
        } else {
            None
        }
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

        assert!(compiler.is_function_defined(&module.functions, "foo"));
    }

    #[test]
    fn is_variable_defined_true() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler: Compiler = test_file.into();
        let mut module = build_module();
        module
            .variables
            .insert("foo".to_string(), Box::new(42.into()));

        assert!(compiler.is_variable_defined(&module.variables, "foo"));
    }
}
