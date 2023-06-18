//! # Text
//!
//! A string of text - it starts with a `"` and ends with a `"`.  
//!
use serde::{Deserialize, Serialize};
use std::any;
use std::fmt;
use std::str;

use crate::Error;
use super::Node;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Text(pub String);

impl Text {
    pub fn new(text: &str) -> Self {
        Text(text.to_string())
    }
}

impl Node for Text {
    fn as_any(&self) -> &dyn any::Any { self }

    fn node_eq(&self, other: &dyn any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |o| self == o)
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

impl str::FromStr for Text {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Text(input.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn display() {
        assert_eq!("\"foo\"", Text("foo".to_string()).to_string());
    }

    #[test]
    fn from_str() {
        assert_eq!(Text::new("foo"), Text::from_str("foo").unwrap());
    }

    #[test]
    fn node_eq_true() {
        assert!(Node::node_eq(&Text::new("foo"), &Text::new("foo")))
    }

    #[test]
    fn node_eq_false() {
        assert!(!Node::node_eq(&Text::new("foo"), &Text::new("bar")))
    }
}
