//! # Error
//!
//! Error handling structs.
use crate::Output;
use std::error;
use std::path;

mod bad_input;
mod cell_parse_error;
mod display;
mod eval_error;
mod parse_error;

pub use parse_error::ParseError;
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) use bad_input::BadInput;
pub(crate) use cell_parse_error::CellParseError;
pub(crate) use eval_error::EvalError;

pub(crate) type ParseResult<T> = std::result::Result<T, ParseError>;
pub(crate) type EvalResult<T> = std::result::Result<T, EvalError>;

/// The various kinds of errors that can occur during compilation and evaluation of a csv++
/// module.
#[derive(Debug)]
pub enum Error {
    /// A syntax error in a formula in a cell.
    CellSyntaxError {
        filename: path::PathBuf,
        position: a1_notation::Address,
        parse_error: Box<ParseError>,
    },

    /// A syntax error in the code section.
    CodeSyntaxError {
        filename: path::PathBuf,
        parse_error: Box<ParseError>,
    },

    /// An error encountered when evaluating the formulas in a cell.  For example if a builtin
    /// funciton is called with the wrong number of arguments.
    EvalError {
        message: String,
        filename: path::PathBuf,
        position: a1_notation::Address,
    },

    /// Google Sheets requires that the `gcloud` CLI tools are installed and configured.  If we
    /// think they're not, this message includes a lot of details about setting them up.  So
    /// it's very Google Sheets-specific.
    GoogleSetupError(String),

    /// An error while building the runtime or reading the source code.  These are typically not
    /// due to user error.
    InitError(String),

    /// An error encountered while serializing the compiled module to an object file.
    ObjectCodeError {
        filename: path::PathBuf,
        message: String,
    },

    /// An error ecountered reaading or doing an initial parse of the source code.
    SourceCodeError {
        filename: path::PathBuf,
        message: String,
    },

    /// An error encountered while writing to the target.
    TargetWriteError { message: String, output: Output },
}

impl error::Error for Error {}
