//! # Reference
//!
//! A reference is a bit of a catch-all value - a reference can be an identifier like a variable or 
//! function name, but it can also be a cell reference literal. A cell reference literal is
//! something like A1, A1:B2 or even Sheet1!A1:A2.  
//!
//! This leaves us in an awkward place where we want to have special boutique logic for handling
//! to and from A1 format but at the same time a Reference could also just be a function call - unless
//! we have additional knowledge there's no way to know if A3 is a variable reference/function call
//! or an A1-format literal.
//!
// TODO need to be able to init from either A1 format or a cell index
//
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str;

use crate::Error;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Reference(String);

impl super::Node for Reference {}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for Reference {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Reference(input.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn display() {
        assert_eq!("foo", Reference("foo".to_string()).to_string());
    }

    #[test]
    fn from_str() {
        assert_eq!(Reference("bar".to_string()), Reference::from_str("bar").unwrap());
    }
}
