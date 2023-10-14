//! # TokenMatch
//!
use super::Token;
use crate::error::BadInput;
use crate::{CharOffset, LineNumber};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TokenMatch {
    pub(crate) token: Token,
    pub(crate) str_match: String,
    pub(crate) line_offset: CharOffset,
    pub(crate) line_number: LineNumber,
}

impl fmt::Display for TokenMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.str_match)
    }
}

impl BadInput for TokenMatch {
    fn line_number(&self) -> LineNumber {
        self.line_number
    }

    fn line_offset(&self) -> CharOffset {
        self.line_offset
    }
}
