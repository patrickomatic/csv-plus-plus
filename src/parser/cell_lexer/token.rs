//! # Token
//!
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Token {
    A1,
    CloseParenthesis,
    Color,
    Date,
    EndOptions,
    Equals,
    Identifier,
    Number,
    OpenParenthesis,
    OptionName,
    PositiveNumber,
    String,
    Slash,
    StartCellOptions,
    StartRowOptions,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Token::OptionName.to_string(), "OptionName");
    }
}
