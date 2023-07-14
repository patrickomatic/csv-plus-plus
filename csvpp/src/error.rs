//! Error handling functions
use std::error;
use std::fmt;
use std::path::PathBuf;
use crate::OutputTarget;

#[derive(Clone, Debug)]
pub enum Error {
    A1ParseError {
        bad_input: String,
        message: String,
    },
    // TODO we could have a codesyntax error in a cell
    CodeSyntaxError {
        bad_input: String,
        line_number: usize,
        message: String,
    },
    CellSyntaxError {
        index: a1_notation::A1,
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
        index: a1_notation::A1,
        message: String,
    },
    ObjectWriteError {
        filename: PathBuf,
        message: String,
    },
    RgbSyntaxError {
        bad_input: String,
        message: String,
    },
    SourceCodeError {
        filename: PathBuf,
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
            Error::A1ParseError { bad_input, message } => {
                writeln!(f, "Error parsing A1 value: {}", message)?;
                write!(f, "bad input: {}", bad_input)
            },
            Error::CodeSyntaxError { bad_input, line_number, message } => {
                writeln!(f, "{}: {}", line_number, message)?;
                write!(f, "bad input: {}", bad_input)
            },
            Error::CellSyntaxError { index, message } => 
                write!(f, "Cell->{}: {}", index, message),
            Error::InitError(message) => 
                write!(f, "Error initializing: {}", message),
            Error::InvalidModifier { message, bad_input, possible_values } => {
                writeln!(f, "{}", message)?;
                writeln!(f, "bad input: {}", bad_input)?;
                write!(f, "possible values: {}", possible_values)
            },
            Error::ModifierSyntaxError { bad_input, index, message } => {
                writeln!(f, "Cell->{}: {}", index, message)?;
                write!(f, "bad input: {}", bad_input)
            },
            Error::ObjectWriteError { filename, message } =>
                writeln!(f, "Error writing object file [{}]: {}", filename.display(), message),
            Error::RgbSyntaxError { bad_input, message } => {
                writeln!(f, "Error parsing RGB value: {}", message)?;
                write!(f, "bad input: {}", bad_input)
            },
            Error::SourceCodeError { filename, message } => {
                writeln!(f, "Error reading source: {}", filename.display())?;
                write!(f, "{}", message)
            },
            Error::TargetWriteError { target, message } => 
                write!(f, "Error writing to {}: {}", target, message),
        }
    }
}

impl From<a1_notation::Error> for Error {
    fn from(err: a1_notation::Error) -> Self {
        match err {
            a1_notation::Error::A1BuilderError(m) =>
                Error::InitError(format!("Error building parsing A1 format: {}", m)),
            a1_notation::Error::A1ParseError { bad_input, message } =>
                Error::A1ParseError { bad_input, message },
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
            index: a1_notation::A1::builder().xy(1, 5).build().unwrap(),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->F2: foo", message.to_string());
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
            index: a1_notation::A1::builder().xy(0, 1).build().unwrap(),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->B1: foo\nbad input: bad_input", message.to_string());
    }
}
