//! # Token
//!
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Token {
    A1,
    CloseParenthesis,
    Color,
    Date,
    EndModifier,
    Equals,
    Identifier,
    ModifierName,
    Number,
    OpenParenthesis,
    PositiveNumber,
    String,
    Slash,
    StartCellModifier,
    StartRowModifier,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Token::ModifierName.to_string(), "ModifierName");
    }
}
