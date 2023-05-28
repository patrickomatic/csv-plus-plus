//! Error handling functions
use std::error::Error;
use std::fmt;
use std::result;

#[derive(Clone, Debug)]
struct SyntaxError(String);

type Result<T> = result::Result<T, SyntaxError>;

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax error at X:X[X,X]: {}", self.0)
    }
}

impl Error for SyntaxError {}
