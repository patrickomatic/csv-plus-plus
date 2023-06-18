//! # Integer
//!
//! A signed integer with a maximum value of 64 bits.
//!
use serde::{Deserialize, Serialize};
use std::any;
use std::fmt;
use std::str;

use crate::Error;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Integer(pub i64);

impl super::Node for Integer {
    fn as_any(&self) -> &dyn any::Any { self }

    fn node_eq(&self, other: &dyn any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |f| self == f)
        /*
        if let Some(other_int) = (&*other).downcast_ref::<Integer>() {
            dbg!(other_int);
            return *self == *other_int
        }

        false
        */
    }
}

impl str::FromStr for Integer {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.parse::<i64>() {
            Ok(i) => Ok(Integer(i)),
            Err(e) => Err(Error::CodeSyntaxError {
                message: format!("Error parsing integer value: {}", e),
                bad_input: input.to_string(), 
                line_number: 0, // XXX
            }),
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use super::super::*;

    #[test]
    fn display() {
        assert_eq!("123", Integer(123).to_string());
    }

    #[test]
    fn from_str() {
        assert_eq!(Integer(123), Integer::from_str("123").unwrap());
    }

    #[test]
    fn node_eq() {
        assert!(Node::node_eq(&Integer(123), &Integer(123)))
    }

    #[test]
    fn node_eq_false() {
        assert!(!Node::node_eq(&Integer(123), &Integer(456)));
        assert!(!Node::node_eq(&Integer(123), &Float(123.0)))
    }
}
