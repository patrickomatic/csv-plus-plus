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
    InvalidModifier {
        message: String,
        bad_input: String,
        possible_values: String,
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: anything else to do when in verbose mode?
        match self {
            Error::CodeSyntaxError { bad_input, line_number, message } => {
                writeln!(f, "{}: {}", line_number, message)?;
                write!(f, "bad input: {}", bad_input)
            },
            Error::CellSyntaxError { index: Position(x, y), message } => 
                write!(f, "Cell->[{},{}]: {}", x, y, message),
            Error::InitError(message) => 
                write!(f, "Error initializing: {}", message),
            Error::InvalidModifier { message, bad_input, possible_values } => {
                writeln!(f, "{}", message)?;
                writeln!(f, "bad input: {}", bad_input)?;
                write!(f, "possible values: {}", possible_values)
            },
            Error::ModifierSyntaxError { bad_input, index: Position(x, y), message } => {
                writeln!(f, "Cell->[{},{}]: {}", x, y, message)?;
                write!(f, "bad input: {}", bad_input)
            },
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
    fn display_cell_syntax_error() {
        let message = Error::CellSyntaxError {
            index: Position(1, 5),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->[1,5]: foo", message.to_string());
    }

    #[test]
    fn display_code_syntax_error() {
        let message = Error::CodeSyntaxError {
            line_number: 1,
            message: "foo".to_string(),
            bad_input: "bar".to_string(),
        };

        assert_eq!("1: foo\nbad input: bar", message.to_string());
    }

    #[test]
    fn display_modifier_syntax_error() {
        let message = Error::ModifierSyntaxError {
            bad_input: "bad_input".to_string(),
            index: Position(0, 1),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->[0,1]: foo\nbad input: bad_input", message.to_string());
    }
}
