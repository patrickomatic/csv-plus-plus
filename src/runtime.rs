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
        writeln!(f, "# CLI Options")?;
        writeln!(f, "{}", self.options)?;
        writeln!(f, "\n# Parsed Source Code")?;
        writeln!(f, "{}", self.source_code)?;
        writeln!(f, "\n# Output Target")?;
        writeln!(f, "{}", self.output)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::TestFile;
    use super::*;

    #[test]
    fn new() {
        let test_file = TestFile::new("csv", "foo,bar,baz");
        let cli_args = CliArgs {
            input_filename: test_file.input_file.clone(),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        let runtime = Runtime::new(cli_args);

        assert!(runtime.is_ok());
    }

    #[test]
    fn display() {
        let test_file = TestFile::new("csv", "foo,bar,baz");
        let cli_args = CliArgs {
            input_filename: test_file.input_file.clone(),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        let runtime_str = Runtime::new(cli_args).unwrap().to_string();

        assert!(runtime_str.contains("CLI Options"));
    }
}

