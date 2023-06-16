//! # Float
//!
//! A float value.
//!
use serde::{Deserialize, Serialize};
use std::any;
use std::fmt;
use std::str;

use crate::Error;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Float(pub f64);

impl super::Node for Float {
    fn eq(&self, other: &dyn any::Any) -> bool {
        if let Some(other_float) = other.downcast_ref::<Float>() {
            return self == other_float
        }

        false
    }
}

impl str::FromStr for Float {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.parse::<f64>() {
            Ok(i) => Ok(Float(i)),
            Err(e) => Err(Error::CodeSyntaxError {
                message: format!("Error parsing float value: {}", e),
                bad_input: input.to_string(), 
                line_number: 0, // XXX
            }),
        }
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn display() {
        assert_eq!("123.45", Float(123.45).to_string());
    }

    #[test]
    fn from_str() {
        assert_eq!(Float(123.45), Float::from_str("123.45").unwrap());
    }

    #[test]
    fn from_str_invalid() {
        assert!(Float::from_str("foo").is_err());
    }
}
