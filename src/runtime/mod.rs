//! # Runtime
//!
use crate::ast::{BuiltinFunctions, BuiltinVariables};
use crate::{CliArgs, CompilationTarget, Options, Output, Result, SourceCode};
use clap::Parser;

mod display;
mod try_from;

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
        Self::try_from(&CliArgs::parse())
    }

    pub fn target<'a>(&'a self) -> Result<Box<dyn CompilationTarget + 'a>> {
        self.output.compilation_target(self)
    }
}
