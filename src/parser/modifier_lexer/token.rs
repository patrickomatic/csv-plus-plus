//! # Token
//!
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    A1,
    CloseParenthesis,
    Color,
    Comma,
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
        writeln!(f, "{:?}", self)
    }
}
