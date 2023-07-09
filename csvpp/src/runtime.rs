//! # Runtime
//!
use clap::Parser;
use std::fmt;
use crate::{CliArgs, CompilationTarget, Init, Options, OutputTarget, Result, SourceCode, Template};
use crate::compiler::token_library::TokenLibrary;
use crate::ast::{BuiltinFunction, BuiltinFunctions, BuiltinVariable, BuiltinVariables};

// TODO: manually implement debug
// #[derive(Debug)]
pub struct Runtime {
    pub builtin_functions: BuiltinFunctions,
    pub builtin_variables: BuiltinVariables,
    pub options: Options,
    pub target: OutputTarget,
    pub source_code: SourceCode,
    pub template: Template,
    pub token_library: TokenLibrary,
}

impl Runtime {
    pub fn from_cli_args() -> Result<Self> {
        Self::new(CliArgs::parse())
    }

    pub fn new(cli_args: CliArgs) -> Result<Self> {
        let token_library = TokenLibrary::build()?;
        let init = Init::from_cli_args(cli_args, &token_library)?;

        Ok(Self {
            builtin_functions: BuiltinFunction::all(),
            builtin_variables: BuiltinVariable::all(),
            options: init.options,
            target: init.output,
            source_code: init.source_code,
            template: Template::default(),
            token_library: TokenLibrary::build()?,
        })
    }

    pub fn target<'a>(&'a self) -> Result<Box<dyn CompilationTarget + 'a>> {
        self.target.compilation_target(self)
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"
# csv++

## Called with options

{}

## Parsed template

{}
"#, 
            self.options,
            self.template,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn build_runtime() -> Result<Runtime> {
        let cli_args = CliArgs {
            input_filename: PathBuf::from("foo.csvpp"),
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

variables: {}
functions: {}
rows: 0
"#, runtime.to_string());
    }
}

