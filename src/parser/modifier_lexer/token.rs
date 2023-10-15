//! # Token
//!
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    CloseParenthesis,
    Color,
    Comma,
    Date,
    EndModifier,
    Equals,
    ModifierName,
    ModifierRightSide,
    OpenParenthesis,
    PositiveNumber,
    String,
    Slash,
    StartCellModifier,
    StartRowModifier,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self)
    }
}
