//! # Token
//!
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Color,
    EndModifier,
    Equals,
    ModifierName,
    ModifierRightSide,
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
