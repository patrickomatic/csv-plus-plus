//! Error handling functions
use crate::Output;
use std::error;
use std::path::PathBuf;

mod display;
mod inner_error;

pub use inner_error::InnerError;

pub type Result<T> = std::result::Result<T, Error>;
pub type InnerResult<T> = std::result::Result<T, InnerError>;

#[derive(Clone, Debug)]
pub enum Error {
    /// A syntax error in a formula in a cell.
    CellSyntaxError {
        line_number: usize,
        position: a1_notation::Address,
        inner_error: Box<InnerError>,
    },

    /// A syntax error in the code section.
    CodeSyntaxError {
        highlighted_lines: Vec<String>,
        line_number: usize,
        message: String,
        position: usize,
    },

    /// An error encountered when evaluating the formulas in a cell.  For example if a builtin
    /// funciton is called with the wrong number of arguments.
    EvalError {
        line_number: usize,
        message: String,
        position: a1_notation::Address,
    },

    /// An error while building the runtime or reading the source code.  These are typically not
    /// due to user error.
    InitError(String),

    /// A syntax error encountered while parsing the modifiers of a cell.
    ModifierSyntaxError {
        inner_error: Box<InnerError>,
        position: a1_notation::Address,
        line_number: usize,
    },

    /// An error encountered while serializing the compiled template to an object file.
    ObjectWriteError { filename: PathBuf, message: String },

    /// An error ecountered reaading or doing an initial parse of the source code.
    SourceCodeError { filename: PathBuf, message: String },

    /// An error encountered while writing to the target.
    TargetWriteError { message: String, output: Output },
}

impl error::Error for Error {}
