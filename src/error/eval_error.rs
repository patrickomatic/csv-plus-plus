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
    pub(crate) fn new(position: a1_notation::Address, message: &str) -> EvalError {
        EvalError {
            position,
            message: message.to_string(),
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
    // use super::*;
    // TODO
}
