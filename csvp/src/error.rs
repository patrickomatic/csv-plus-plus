use super::SourcePosition;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseError {
        bad_input: String,
        message: String,
        position: SourcePosition,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError {
                bad_input,
                message,
                position,
            } => {
                writeln!(f, "Error parsing {bad_input} at {position}: {message}")
            }
        }
    }
}

impl error::Error for Error {}
