//! # `EvalError`
use std::{error, fmt};

#[derive(Debug)]
pub struct EvalError {
    pub bad_input: String,
    pub message: String,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.bad_input)
    }
}

impl EvalError {
    pub(crate) fn new<B, M>(bad_input: B, message: M) -> Self
    where
        B: Into<String>,
        M: Into<String>,
    {
        Self {
            bad_input: bad_input.into(),
            message: message.into(),
        }
    }
}

impl error::Error for EvalError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_eval_error() {
        assert_eq!(
            EvalError::new("foo_bar(1, 2)", "The function expected one argument").to_string(),
            "The function expected one argument: foo_bar(1, 2)"
        );
    }
}
