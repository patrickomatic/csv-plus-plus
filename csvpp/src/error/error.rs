//! Error handling functions
use std::error;
use std::fmt;
use std::path::PathBuf;
use crate::Output;

#[derive(Clone, Debug)]
pub enum Error {
    CodeSyntaxError {
        message: String,
        line_number: usize,
        position: usize,
    },
    CellSyntaxError {
        message: String,
        position: a1_notation::A1,
    },
    EvalError {
        message: String,
        line_number: usize,
        position: a1_notation::A1,
    },
    InitError(String),
    ModifierSyntaxError {
        position: a1_notation::A1,
        message: String,
    },
    ObjectWriteError {
        filename: PathBuf,
        message: String,
    },
    SourceCodeError {
        filename: PathBuf,
        message: String,
    },
    TargetWriteError {
        output: Output,
        message: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CellSyntaxError { message, position } =>
                write!(f, "Cell->{}: {}", position, message),
            Self::CodeSyntaxError { line_number, message, position } =>
                write!(f, "{}:{}: {}", line_number, position, message),
            Self::EvalError { message, line_number, position } =>
                write!(f, "{}: Cell->{}: {}", line_number, position, message),
            Self::InitError(message) => 
                write!(f, "Error initializing: {}", message),
            Self::ModifierSyntaxError { position, message } =>
                write!(f, "Cell->{}: {}", position, message),
            Self::ObjectWriteError { filename, message } =>
                write!(f, "Error writing object file [{}]: {}", filename.display(), message),
            Self::SourceCodeError { filename, message } => {
                writeln!(f, "Error reading source: {}", filename.display())?;
                write!(f, "{}", message)
            },
            Self::TargetWriteError { output, message } => 
                write!(f, "Error writing to {}: {}", output, message),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn display_cell_syntax_error() {
        let message = Error::CellSyntaxError {
            position: a1_notation::A1::builder().xy(1, 5).build().unwrap(),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->B6: foo", message.to_string());
    }
    */

    #[test]
    fn display_code_syntax_error() {
        let message = Error::CodeSyntaxError {
            position: 2,
            line_number: 1,
            message: "foo".to_string(),
        };

        assert_eq!("1:2: foo", message.to_string());
    }

    #[test]
    fn display_modifier_syntax_error() {
        let message = Error::ModifierSyntaxError {
            position: a1_notation::A1::builder().xy(0, 1).build().unwrap(),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->A2: foo", message.to_string());
    }
}
