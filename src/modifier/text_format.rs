//! # TextFormat
use crate::ParseError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub enum TextFormat {
    Bold,
    Italic,
    Strikethrough,
    Underline,
}

impl FromStr for TextFormat {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "b" | "bold" => Ok(Self::Bold),
            "i" | "italic" => Ok(Self::Italic),
            "s" | "strikethrough" => Ok(Self::Strikethrough),
            "u" | "underline" => Ok(Self::Underline),
            _ => Err(ParseError::bad_input_with_possibilities(
                input,
                "Invalid format= value",
                "bold (b) | italic (i) | strikethrough (s) | underline (u)",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_bold() {
        assert_eq!(TextFormat::Bold, TextFormat::from_str("b").unwrap());
        assert_eq!(TextFormat::Bold, TextFormat::from_str("bold").unwrap());
        assert_eq!(TextFormat::Bold, TextFormat::from_str("BOLD").unwrap());
    }

    #[test]
    fn from_str_italic() {
        assert_eq!(TextFormat::Italic, TextFormat::from_str("i").unwrap());
        assert_eq!(TextFormat::Italic, TextFormat::from_str("italic").unwrap());
        assert_eq!(TextFormat::Italic, TextFormat::from_str("ITALIC").unwrap());
    }

    #[test]
    fn from_str_underline() {
        assert_eq!(TextFormat::Underline, TextFormat::from_str("u").unwrap());
        assert_eq!(
            TextFormat::Underline,
            TextFormat::from_str("underline").unwrap()
        );
        assert_eq!(
            TextFormat::Underline,
            TextFormat::from_str("UNDERLINE").unwrap()
        );
    }

    #[test]
    fn from_str_strikethrough() {
        assert_eq!(
            TextFormat::Strikethrough,
            TextFormat::from_str("s").unwrap()
        );
        assert_eq!(
            TextFormat::Strikethrough,
            TextFormat::from_str("strikethrough").unwrap()
        );
        assert_eq!(
            TextFormat::Strikethrough,
            TextFormat::from_str("STRIKETHROUGH").unwrap()
        );
    }
}
