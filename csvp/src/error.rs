use super::SourcePosition;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseError {
        message: String,
        position: SourcePosition,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError { message, position } => {
                writeln!(f, "Error parsing at {position}: {message}")
            }
        }
    }
}

impl error::Error for Error {}
