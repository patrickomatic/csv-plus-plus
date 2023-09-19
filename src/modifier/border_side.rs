//! # BorderSide
//!
//! Represents a side (or all) of a spreadsheet cell.
//!
use crate::InnerError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BorderSide {
    All,
    Top,
    Bottom,
    Left,
    Right,
}

impl FromStr for BorderSide {
    type Err = InnerError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "a" | "all" => Ok(Self::All),
            "t" | "top" => Ok(Self::Top),
            "b" | "bottom" => Ok(Self::Bottom),
            "l" | "left" => Ok(Self::Left),
            "r" | "right" => Ok(Self::Right),
            _ => Err(InnerError::bad_input_with_possibilities(
                input,
                "Invalid border= value",
                "all (a) | top (t) | bottom (b) | left (l) | right (r)",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all() {
        assert_eq!(BorderSide::All, BorderSide::from_str("a").unwrap());
        assert_eq!(BorderSide::All, BorderSide::from_str("all").unwrap());
        assert_eq!(BorderSide::All, BorderSide::from_str("ALL").unwrap());
    }

    #[test]
    fn from_str_top() {
        assert_eq!(BorderSide::Top, BorderSide::from_str("t").unwrap());
        assert_eq!(BorderSide::Top, BorderSide::from_str("top").unwrap());
        assert_eq!(BorderSide::Top, BorderSide::from_str("TOP").unwrap());
    }

    #[test]
    fn from_str_right() {
        assert_eq!(BorderSide::Right, BorderSide::from_str("r").unwrap());
        assert_eq!(BorderSide::Right, BorderSide::from_str("right").unwrap());
        assert_eq!(BorderSide::Right, BorderSide::from_str("RIGHT").unwrap());
    }

    #[test]
    fn from_str_bottom() {
        assert_eq!(BorderSide::Bottom, BorderSide::from_str("b").unwrap());
        assert_eq!(BorderSide::Bottom, BorderSide::from_str("bottom").unwrap());
        assert_eq!(BorderSide::Bottom, BorderSide::from_str("BOTTOM").unwrap());
    }

    #[test]
    fn from_str_left() {
        assert_eq!(BorderSide::Left, BorderSide::from_str("l").unwrap());
        assert_eq!(BorderSide::Left, BorderSide::from_str("left").unwrap());
        assert_eq!(BorderSide::Left, BorderSide::from_str("LEFT").unwrap());
    }

    #[test]
    fn from_str_invalid() {
        assert!(BorderSide::from_str("middle").is_err());
        assert!(BorderSide::from_str("123").is_err());
    }
}
