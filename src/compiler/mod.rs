//! # Compiler
//!
use crate::{CliArgs, CompilationTarget, Error, Module, Options, Output, Result};
use clap::Parser;
use colored::Colorize;
use std::fmt;
use std::path;

mod display;
mod try_from;

#[derive(Debug)]
pub struct Compiler {
    pub options: Options,
    pub output: Output,
    pub(crate) input_filename: path::PathBuf,
}

// TODO: use an actual logger
impl Compiler {
    pub fn compile(&self) -> Result<Module> {
        /* TODO bring back object file usage
        Ok(if let Some(t) = Module::read_from_object_file(self)? {
            self.progress("Read module from object file (not compiling)");
            self.info(&t);
            t
        } else {
        */
        self.progress("Compiling module from source code");

        let main_module =
            self.eval(Module::try_from(self.input_filename.clone())?.load_dependencies()?)?;

        self.progress("Compiled module");
        self.info(&main_module);

        self.progress(format!(
            "Writing object file {}",
            main_module.source_code.object_code_filename().display()
        ));
        main_module.write_object_file(self)?;

        Ok(main_module)
        // })
    }

    pub fn from_cli_args() -> Result<Self> {
        Self::try_from(&CliArgs::parse())
    }

    pub fn target<'a>(&'a self) -> Result<Box<dyn CompilationTarget + 'a>> {
        self.output.compilation_target(self)
    }

    pub(crate) fn error<M: Into<String>>(&self, message: M) {
        eprintln!("{}", message.into().red());
    }

    pub(crate) fn info<M: fmt::Display>(&self, message: M) {
        if self.options.verbose {
            eprintln!("{message}");
        }
    }

    pub(crate) fn output_error<M: Into<String>>(&self, message: M) -> Error {
        self.output.clone().into_error(message)
    }

    pub(crate) fn progress<M: fmt::Display>(&self, message: M) {
        if self.options.verbose {
            eprintln!("{}", message.to_string().green());
        }
    }

    pub(crate) fn warn<M: Into<String>>(&self, message: M) {
        eprintln!("{}", message.into().yellow());
    }

    fn eval(&self, module: Module) -> Result<Module> {
        self.progress("Evaluating module");

        module
            .eval_fills()
            .eval_spreadsheet(self.options.key_values.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

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
}
