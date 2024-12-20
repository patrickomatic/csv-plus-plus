#![warn(clippy::pedantic)]
#![deny(warnings)]

mod ast;
mod cell;
mod cell_options;
mod cli_args;
mod compiler;
mod config;
mod date_time;
mod error;
mod logger;
mod module;
mod module_loader;
mod module_path;
mod output;
mod parser;
mod rgb;
mod row;
mod scope;
mod source_code;
mod spreadsheet;
mod target;

pub(crate) use cell::Cell;
pub use cli_args::CliArgs;
pub use compiler::Compiler;
pub(crate) use config::Config;
pub use date_time::DateTime;
pub(crate) use error::EvalResult;
pub use error::{Error, EvalError, ParseError, Result};
pub use module::Module;
pub(crate) use module_loader::ModuleLoader;
pub use module_path::ModulePath;
pub(crate) use output::Output;
pub use rgb::Rgb;
pub use row::Row;
pub use scope::Scope;
pub(crate) use source_code::ArcSourceCode;
pub use source_code::{CharOffset, LineNumber, SourceCode};
pub use spreadsheet::Spreadsheet;
pub(crate) use target::CompilationTarget;

use log::{error, info, warn};

// test_utils should only be included in tests, never referenced by release code (or built into the
// release)
#[cfg(test)]
mod test_utils;

pub(crate) fn compiler_error<S: std::fmt::Display>(message: S) -> ! {
    error!(
        "csv++ ran into an unexpected error while compiling.
Please run with `-vvvv` as a CLI flag and share a copy of the output and your source code at:
https://github.com/patrickomatic/csv-plus-plus/issues"
    );
    panic!("{message}")
}

pub(crate) fn deprecated_feature<S: std::fmt::Display>(message: S, to_fix: S) {
    warn!("Deprecation warning: {message}");
    info!("To fix: {to_fix}");
}
