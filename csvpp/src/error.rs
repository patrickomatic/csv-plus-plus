//! Error handling functions
use std::error;
use std::fmt;

use crate::Position;
use crate::options::OutputTarget;

#[derive(Clone, Debug)]
pub enum Error<'a> {
    // TODO we could have a codesyntax error in a cell
    CodeSyntaxError {
        bad_input: &'a str,
        line_number: usize,
        message: &'a str,
    },
    CellSyntaxError {
        index: Position,
        message: String,
    },
    InitError(String),
    InvalidModifier {
        message: &'a str,
        bad_input: &'a str,
        possible_values: &'a str,
    },
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

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: anything else to do when in verbose mode?
        match self {
            Error::CodeSyntaxError { bad_input, line_number, message } => {
                write!(f, "{}: {}", line_number, message);
                write!(f, "bad input: {}", bad_input)
            },
            Error::CellSyntaxError { index: Position(x, y), message } => 
                write!(f, "Cell->[{},{}]: {}", x, y, message),
            Error::InitError(message) => 
                write!(f, "Error initializing: {}", message),
            Error::InvalidModifier { message, bad_input, possible_values } => {
                write!(f, "{}", message);
                write!(f, "bad input: {}", bad_input);
                write!(f, "possible values: {}", possible_values)
            },
            Error::ModifierSyntaxError { bad_input, index: Position(x, y), message } => {
                write!(f, "Cell->[{},{}]: {}", x, y, message);
                write!(f, "bad input: {}", bad_input)
            },
            Error::TargetWriteError { target, message } => 
                write!(f, "Error writing to {}: {}", target, message),
        }
    }
}

impl error::Error for Error<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_syntax_error_display() {
        let message = Error::CodeSyntaxError {
            line_number: 1,
            message: "foo",
            bad_input: "bar",
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
