//! # Runtime
//!
use clap::Parser;
use std::fmt;
use crate::{CliArgs, CompilationTarget, Options, Output, Result, SourceCode};
use crate::ast::{BuiltinFunction, BuiltinFunctions, BuiltinVariable, BuiltinVariables};

#[derive(Debug)]
pub struct Runtime {
    pub builtin_functions: BuiltinFunctions,
    pub builtin_variables: BuiltinVariables,
    pub options: Options,
    pub output: Output,
    pub source_code: SourceCode,
}

impl Runtime {
    pub fn from_cli_args() -> Result<Self> {
        Self::new(CliArgs::parse())
    }

    pub fn new(cli_args: CliArgs) -> Result<Self> {
        Ok(Self {
            builtin_functions: BuiltinFunction::all(),
            builtin_variables: BuiltinVariable::all(),
            options: Options::from_cli_args(&cli_args)?,
            output: Output::from_cli_args(&cli_args)?,
            source_code: SourceCode::open(cli_args.input_filename)?,
        })
    }

    pub fn target<'a>(&'a self) -> Result<Box<dyn CompilationTarget + 'a>> {
        self.output.compilation_target(self)
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "
# csv++

## Called with options

{}

## Parsed template

XXX

", 
            self.options,
            // TODO self.template,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path;
    use super::*;

    fn build_runtime() -> Result<Runtime> {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        Runtime::new(cli_args)
    }

    #[test]
    fn new() {
        let runtime = build_runtime();
        assert!(runtime.is_ok());
    }

    /*
    #[test]
    fn display() {
        let runtime = build_runtime().unwrap();
        assert_eq!(r#"
# csv++

## Called with options

backup: false
google_account_credentials: none
key_values: {}
offset: (0, 0)
overwrite_values: true
verbose: false

## Parsed template

"#, runtime.to_string());
    }
    */
}

