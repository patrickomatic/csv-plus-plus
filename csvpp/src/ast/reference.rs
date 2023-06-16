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
use std::any;
use std::fmt;
use std::str;

use crate::{Error, Node};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Reference(pub String);

impl Reference {
    /// "A1 format" ([\w!:_]) is a superset of the things that can be functions or variables
    /// ([\w_]).  So we know for sure when we're dealing with (some) A1 references if they have a
    /// character like `!` or `:` that wouldn't be allowed in a function name.  
    ///
    /// For example:
    ///
    /// ```
    /// // we know that A1:B2 can only be a cell reference, not a variable or function
    /// let r1 = csvpp::Reference("A1:B2".to_string());
    /// assert!(r1.is_definitely_a1_format());
    ///
    /// // and the same with Sheet1!C3
    /// let r2 = csvpp::Reference("Sheet1!C3".to_string());
    /// assert!(r2.is_definitely_a1_format());
    ///
    /// // however "A1" could be a function/variable ref OR a cell reference
    /// let r3 = csvpp::Reference("A1".to_string());
    /// assert!(!r3.is_definitely_a1_format());
    /// ```
    ///
    pub fn is_definitely_a1_format(&self) -> bool {
        for c in self.0.chars() {
            if !c.is_alphanumeric() && c != '_' {
                return true
            }
        }
        false
    }
}

impl Node for Reference {
    fn id_ref(&self) -> Option<super::NodeId> {
        if self.is_definitely_a1_format() {
            None
        } else {
            Some(self.0.clone())
        }
    }

    fn eq(&self, other: &dyn any::Any) -> bool {
        if let Some(other_ref) = other.downcast_ref::<Reference>() {
            return self == other_ref
        }

        false
    }
}

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
    use super::super::*;

    #[test]
    fn display() {
        assert_eq!("foo", Reference("foo".to_string()).to_string());
    }

    #[test]
    fn from_str() {
        assert_eq!(Reference("bar".to_string()), Reference::from_str("bar").unwrap());
    }

    #[test]
    fn eq() {
        assert!(Node::eq(&Reference("foo".to_string()), &Reference("foo".to_string())))
    }

    #[test]
    fn eq_false() {
        assert!(!Node::eq(&Reference("foo".to_string()), &Reference("bar".to_string())));
        assert!(!Node::eq(&Reference("foo".to_string()), &Float(123.0)))
    }
}
