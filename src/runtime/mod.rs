//! # Runtime
//!
use crate::ast::{BuiltinFunctions, BuiltinVariables};
use crate::parser::ast_lexer::TokenLibrary;
use crate::{CliArgs, CompilationTarget, Options, Output, Result, SourceCode};
use clap::Parser;
use std::sync;

mod display;
mod try_from;

#[derive(Debug)]
pub struct Runtime {
    pub(crate) builtin_functions: BuiltinFunctions,
    pub(crate) builtin_variables: BuiltinVariables,
    pub options: Options,
    pub output: Output,
    pub source_code: sync::Arc<SourceCode>,
    pub(crate) token_library: TokenLibrary,
}

impl Runtime {
    pub fn from_cli_args() -> Result<Self> {
        Self::try_from(&CliArgs::parse())
    }

    pub fn target<'a>(&'a self) -> Result<Box<dyn CompilationTarget + 'a>> {
        self.output.compilation_target(self)
    }
}
