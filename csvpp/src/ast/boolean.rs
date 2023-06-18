//! # Boolean
//!
//! Can either be TRUE or FALSE.
//!
use serde::{Serialize, Deserialize};
use std::any;
use std::fmt;
use std::str;

use crate::Error;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Boolean(pub bool);

impl super::Node for Boolean {
    fn as_any(&self) -> &dyn any::Any { self }

    fn node_eq(&self, other: &dyn any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self.0 { "TRUE" } else { "FALSE" })
    }
}

impl str::FromStr for Boolean {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_lower = input.to_lowercase();
        if input_lower == "true" {
            Ok(Boolean(true))
        } else if input_lower == "false" {
            Ok(Boolean(false))
        } else {
            Err(Error::CodeSyntaxError {
                message: "Error parsing boolean value".to_string(),
                bad_input: input.to_string(), 
                line_number: 0, // XXX
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use super::super::*;

    #[test]
    fn display() {
        assert_eq!("TRUE", Boolean(true).to_string());
        assert_eq!("FALSE", Boolean(false).to_string());
    }

    #[test]
    fn from_str_false() {
        assert_eq!(Boolean(false), Boolean::from_str("false").unwrap());
        assert_eq!(Boolean(false), Boolean::from_str("FALSE").unwrap());
    }

    #[test]
    fn from_str_true() {
        assert_eq!(Boolean(true), Boolean::from_str("true").unwrap());
        assert_eq!(Boolean(true), Boolean::from_str("TRUE").unwrap());
    }

    #[test]
    fn from_str_invalid() {
        assert!(Boolean::from_str("foo").is_err());
    }

    #[test]
    fn node_eq_false() {
        assert!(!&Boolean(true).node_eq(&Boolean(false)));
        assert!(!&Boolean(false).node_eq(&Boolean(true)))
    }

    #[test]
    fn node_eq_true() {
        assert!(&Boolean(true).node_eq(&Boolean(true)));
        assert!(&Boolean(false).node_eq(&Boolean(false)))
    }
}
