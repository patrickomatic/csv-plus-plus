//! Error handling functions
use std::error::Error;
use std::fmt;

use crate::Position;

#[derive(Clone, Debug)]
pub enum CsvppError<'a> {
    // TODO we could have a codesyntax error in a cell
    CodeSyntaxError {
        line_number: usize,
        message: &'a str,
    },
    CellSyntaxError {
        index: Position,
        message: &'a str,
    },
    ModifierSyntaxError {
        bad_input: String,
        index: Position,
        message: String,
    },
}

impl<'a> fmt::Display for CsvppError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CsvppError::CodeSyntaxError { line_number, message } => 
                write!(f, "{}: {}", line_number, message),
            CsvppError::CellSyntaxError { index: Position(x, y), message } => 
                write!(f, "Cell->[{},{}]: {}", x, y, message),
            CsvppError::ModifierSyntaxError { bad_input, index: Position(x, y), message } => 
                // TODO: more specific error message
                write!(f, "Cell->[{},{}]: {}. bad_input = {}", x, y, message, bad_input),
        }
    }
}

impl<'a> Error for CsvppError<'a> {}
