//! # Compiler
//!
use crate::ast::{BuiltinFunctions, BuiltinVariables};
use crate::parser::ast_lexer;
use crate::parser::cell_lexer;
use crate::{CliArgs, CompilationTarget, Error, Options, Output, Result, SourceCode};
use clap::Parser;
use colored::Colorize;
use std::fmt;
use std::sync;

mod display;
mod eval;
mod try_from;

#[derive(Debug)]
pub struct Compiler {
    pub(crate) builtin_functions: BuiltinFunctions,
    pub(crate) builtin_variables: BuiltinVariables,
    pub options: Options,
    pub output: Output,
    pub source_code: sync::Arc<SourceCode>,
    pub(crate) ast_token_library: ast_lexer::TokenLibrary,
    pub(crate) cell_token_library: cell_lexer::TokenLibrary,
}

impl Compiler {
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
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn info_verbose() {
        // TODO
    }

    #[test]
    fn info_not_verbose() {
        // TODO
    }

    #[test]
    fn warn() {
        // TODO
    }
}
