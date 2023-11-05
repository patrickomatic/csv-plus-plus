//! # EvalError
//!
use std::error;
use std::fmt;

#[derive(Debug)]
pub(crate) struct EvalError {
    pub(crate) position: a1_notation::Address,
    pub(crate) message: String,
}

impl EvalError {
    pub(crate) fn new<S: Into<String>>(position: a1_notation::Address, message: S) -> EvalError {
        EvalError {
            position,
            message: message.into(),
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl error::Error for EvalError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            EvalError::new(a1_notation::Address::new(0, 0), "foo").to_string(),
            "foo\n"
        );
    }
}
