//! # Error
//!
//! Error handling structs.
use crate::{ModulePath, Output};
use std::{collections, error, path};

mod bad_input;
mod cell_parse_error;
mod display;
mod eval_error;
mod from;
mod parse_error;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) use bad_input::BadInput;
pub(crate) use cell_parse_error::CellParseError;

#[allow(clippy::module_name_repetitions)]
pub use eval_error::EvalError;
pub(crate) type EvalResult<T> = std::result::Result<T, EvalError>;

#[allow(clippy::module_name_repetitions)]
pub use parse_error::ParseError;
pub(crate) type ParseResult<T> = std::result::Result<T, ParseError>;

/// The various kinds of errors that can occur during compilation and evaluation of a csv++
/// module.
#[derive(Debug)]
pub enum Error {
    /// A syntax error in a formula in a cell.
    CellSyntaxError {
        filename: path::PathBuf,
        address: a1::Address,
        parse_error: Box<ParseError>,
    },

    /// A syntax error in the code section.
    CodeSyntaxError {
        filename: path::PathBuf,
        parse_error: Box<ParseError>,
    },

    /// There was an error parsing the raw CSV
    CsvParseError {
        filename: path::PathBuf,
        parse_error: Box<ParseError>,
    },

    /// An error encountered when evaluating the formulas in a cell.
    EvalError {
        eval_error: Box<EvalError>,
        filename: path::PathBuf,
        address: Option<a1::Address>,
    },

    /// Google Sheets requires that the `gcloud` CLI tools are installed and configured.  If we
    /// think they're not, this message includes a lot of details about setting them up.  So
    /// it's very Google Sheets-specific.
    GoogleSetupError(String),

    /// An error while building the compiler or reading the source code.  These are typically not
    /// due to user error.
    InitError(String),

    ModuleLoadError(String),

    ModuleLoadErrors(collections::HashMap<ModulePath, Error>),

    /// An error encountered reading or doing an initial parse of the source code.
    SourceCodeError {
        filename: path::PathBuf,
        message: String,
    },

    /// An error encountered while writing to the target.
    TargetWriteError {
        message: String,
        output: Output,
    },
}

impl error::Error for Error {}
