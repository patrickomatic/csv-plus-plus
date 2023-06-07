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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_syntax_error_display() {
        let message = CsvppError::CodeSyntaxError {
            line_number: 1,
            message: "foo",
        };

        assert_eq!("1: foo", message.to_string());
    }

    #[test]
    fn cell_syntax_error_display() {
        let message = CsvppError::CellSyntaxError {
            index: Position(1, 5),
            message: "foo",
        };

        assert_eq!("Cell->[1,5]: foo", message.to_string());
    }

    #[test]
    fn modifier_syntax_error_display() {
        let message = CsvppError::ModifierSyntaxError {
            bad_input: "bad input".to_string(),
            index: Position(0, 1),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->[0,1]: foo. bad_input = bad input", message.to_string());
    }
}
 
