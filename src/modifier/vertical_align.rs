//! # VerticalAlign
//!
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use crate::InnerError;

/// The possible values for aligning a cell vertically.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VerticalAlign {
    Bottom,
    Center,
    Top,
}

impl FromStr for VerticalAlign {
    type Err = InnerError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "b" | "bottom"    => Ok(Self::Bottom),
            "c" | "center"    => Ok(Self::Center),
            "t" | "top"       => Ok(Self::Top),
            _ => Err(InnerError::bad_input_with_possibilities(
                input, 
                "Invalid valign= value",
                "bottom (b) | center (c) | top (t)",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_top() {
        assert_eq!(VerticalAlign::Top, VerticalAlign::from_str("t").unwrap());
        assert_eq!(VerticalAlign::Top, VerticalAlign::from_str("top").unwrap());
        assert_eq!(VerticalAlign::Top, VerticalAlign::from_str("TOP").unwrap());
    }

    #[test]
    fn from_str_center() {
        assert_eq!(VerticalAlign::Center, VerticalAlign::from_str("c").unwrap());
        assert_eq!(VerticalAlign::Center, VerticalAlign::from_str("center").unwrap());
        assert_eq!(VerticalAlign::Center, VerticalAlign::from_str("CENTER").unwrap());
    }

    #[test]
    fn from_str_bottom() {
        assert_eq!(VerticalAlign::Bottom, VerticalAlign::from_str("b").unwrap());
        assert_eq!(VerticalAlign::Bottom, VerticalAlign::from_str("bottom").unwrap());
        assert_eq!(VerticalAlign::Bottom, VerticalAlign::from_str("BOTTOM").unwrap());
    }
}
