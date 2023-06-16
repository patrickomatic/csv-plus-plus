//! # Text
//!
//! A string of text - it starts with a `"` and ends with a `"`.  
//!
use serde::{Deserialize, Serialize};
use std::any;
use std::fmt;
use std::str;

use crate::Error;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Text(pub String);

impl super::Node for Text {
    fn eq(&self, other: &dyn any::Any) -> bool {
        if let Some(other_text) = other.downcast_ref::<Text>() {
            return self == other_text
        }

        false
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
        assert_eq!(Text("foo".to_string()), Text::from_str("foo").unwrap());
    }
}
