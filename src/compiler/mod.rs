//! # Compiler
//!
use crate::{ArcSourceCode, CliArgs, CompilationTarget, Error, Options, Output, Result};
use clap::Parser;
use colored::Colorize;
use std::fmt;

mod display;
mod eval;
mod try_from;

#[derive(Debug)]
pub struct Compiler {
    pub options: Options,
    pub output: Output,
    pub(crate) source_code: ArcSourceCode,
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
