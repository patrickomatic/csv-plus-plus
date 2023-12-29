//! # Compiler
//!
use crate::{
    CliArgs, CompilationTarget, Error, Module, ModuleLoader, ModulePath, Options, Output, Result,
};
use clap::Parser;
use log::{debug, info};
use std::path;

mod display;
mod try_from;

#[derive(Debug)]
pub struct Compiler {
    pub options: Options,
    pub output: Output,
    pub(crate) input_filename: path::PathBuf,
}

impl Compiler {
    /// Given the current `self.options` load the main module, all of it's dependencies and
    /// evaluate it and get ready for output.  This function just compiles but does not output.
    ///
    /// # Errors
    ///
    /// * `Error` for anything that can go wrong during compilation.  This can be a huge range,
    /// anything is really game at this point.
    pub fn compile(&self) -> Result<Module> {
        debug!("Loading module from file {}", self.input_filename.display());

        let loader_root = self
            .input_filename
            .parent()
            .unwrap_or_else(|| path::Path::new(""))
            .to_path_buf();

        let Some(main_filename) = self.input_filename.file_name() else {
            return Err(Error::InitError(format!(
                "Unable to extract filename for: {}",
                self.input_filename.display()
            )));
        };

        let main_module_path: ModulePath =
            path::Path::new(main_filename).to_path_buf().try_into()?;

        let main_module =
            Module::load_from_cache_from_filename(main_module_path, self.input_filename.clone())?;

        debug!("Loading dependencies for {}", &main_module.module_path);
        let mut main_module =
            ModuleLoader::load_main(main_module, loader_root, self.options.use_cache)?;

        if main_module.needs_eval {
            debug!("Compiling module");
            main_module = self.eval(main_module)?;

            info!("Compiled main: {main_module}");

            if self.options.use_cache {
                debug!(
                    "Writing object file {}",
                    main_module.source_code.object_code_filename().display()
                );
                main_module.write_object_file()?;
            }
        } else {
            debug!("Cached main is up to date, skipping compilation");
        }

        Ok(main_module)
    }

    /// # Errors
    ///
    /// * `Error::InitError` - if the combination of CLI args are invalid.
    pub fn from_cli_args() -> Result<Self> {
        Self::try_from(&CliArgs::parse())
    }

    /// # Errors
    ///
    /// * `Error`
    pub fn target<'a>(&'a self) -> Result<Box<dyn CompilationTarget + 'a>> {
        self.output.compilation_target(self)
    }

    pub(crate) fn output_error<M: Into<String>>(&self, message: M) -> Error {
        self.output.clone().into_error(message)
    }

    fn eval(&self, module: Module) -> Result<Module> {
        debug!("Evaluating module");

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

        assert_eq!(module.spreadsheet.rows.len(), 2);
    }

    #[test]
    fn compile_with_fill_finite() {
        let test_file = &TestSourceCode::new("xlsx", "![[fill=10]]foo,bar,baz");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile().unwrap();

        assert_eq!(module.spreadsheet.rows.len(), 10);
    }

    #[test]
    fn compile_with_fill_infinite() {
        let test_file = &TestSourceCode::new("xlsx", "![[fill]]foo,bar,baz");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile().unwrap();

        assert_eq!(module.spreadsheet.rows.len(), 1000);
    }

    #[test]
    fn compile_with_fill_multiple() {
        let test_file = &TestSourceCode::new("xlsx", "![[f=10]]foo,bar,baz\n![[f]]1,2,3");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile().unwrap();

        assert_eq!(module.spreadsheet.rows.len(), 1000);
    }

    #[test]
    fn compile_with_fill_and_rows() {
        let test_file =
            &TestSourceCode::new("xlsx", "foo,bar,baz\n![[f=2]]foo,bar,baz\none,last,row\n");
        let compiler: Compiler = test_file.into();
        let module = compiler.compile().unwrap();

        assert_eq!(module.spreadsheet.rows.len(), 4);
    }
}
