//! Error handling functions
use std::error;
use std::fmt;

use crate::Position;
use crate::options::OutputTarget;

#[derive(Clone, Debug)]
pub enum Error {
    // TODO we could have a codesyntax error in a cell
    CodeSyntaxError {
        bad_input: String,
        line_number: usize,
        message: String,
    },
    CellSyntaxError {
        index: Position,
        message: String,
    },
    InitError(String),
    ModifierSyntaxError {
        bad_input: String,
        index: Position,
        message: String,
    },
    TargetWriteError {
        target: OutputTarget,
        message: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CodeSyntaxError { bad_input: _bad_input, line_number, message } => 
                // TODO use bad_input
                write!(f, "{}: {}", line_number, message),
            Error::CellSyntaxError { index: Position(x, y), message } => 
                write!(f, "Cell->[{},{}]: {}", x, y, message),
            Error::InitError(message) => 
                // TODO some kind of better error message when in verbose mode
                write!(f, "Error starting: {}", message),
            Error::ModifierSyntaxError { bad_input, index: Position(x, y), message } => 
                // TODO: more specific error message
                write!(f, "Cell->[{},{}]: {}. bad_input = {}", x, y, message, bad_input),
            Error::TargetWriteError { target, message } => 
                write!(f, "Error writing to {}: {}", target, message),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_syntax_error_display() {
        let message = Error::CodeSyntaxError {
            line_number: 1,
            message: "foo".to_string(),
            bad_input: "bar".to_string(),
        };

        assert_eq!("1: foo", message.to_string());
    }

    #[test]
    fn cell_syntax_error_display() {
        let message = Error::CellSyntaxError {
            index: Position(1, 5),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->[1,5]: foo", message.to_string());
    }

    #[test]
    fn modifier_syntax_error_display() {
        let message = Error::ModifierSyntaxError {
            bad_input: "bad input".to_string(),
            index: Position(0, 1),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->[0,1]: foo. bad_input = bad input", message.to_string());
    }
}
 
