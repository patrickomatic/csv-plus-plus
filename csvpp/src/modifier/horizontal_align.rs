//!
use serde::{Serialize, Deserialize};
use std::str::FromStr;

use crate::Error;

/// The possible values for aligning a cell horizontally.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum HorizontalAlign {
    Center,
    Left,
    Right,
}

impl FromStr for HorizontalAlign {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "c" | "center"  => Ok(Self::Center),
            "l" | "left"    => Ok(Self::Left),
            "r" | "right"   => Ok(Self::Right),
            _ => Err(Error::InvalidModifier { 
                message: "Invalid halign= value".to_string(),
                bad_input: input.to_string(), 
                possible_values: "center (c) | left (l) | right (r)".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_right() {
        assert_eq!(HorizontalAlign::Right, HorizontalAlign::from_str("r").unwrap());
        assert_eq!(HorizontalAlign::Right, HorizontalAlign::from_str("right").unwrap());
        assert_eq!(HorizontalAlign::Right, HorizontalAlign::from_str("RIGHT").unwrap());
    }

    #[test]
    fn from_str_center() {
        assert_eq!(HorizontalAlign::Center, HorizontalAlign::from_str("c").unwrap());
        assert_eq!(HorizontalAlign::Center, HorizontalAlign::from_str("center").unwrap());
        assert_eq!(HorizontalAlign::Center, HorizontalAlign::from_str("CENTER").unwrap());
    }

    #[test]
    fn from_str_left() {
        assert_eq!(HorizontalAlign::Left, HorizontalAlign::from_str("l").unwrap());
        assert_eq!(HorizontalAlign::Left, HorizontalAlign::from_str("left").unwrap());
        assert_eq!(HorizontalAlign::Left, HorizontalAlign::from_str("LEFT").unwrap());
    }

    #[test]
    fn from_str_invalid() {
        assert!(HorizontalAlign::from_str("foo").is_err());
    }
}
