use super::Token;
use crate::error::BadInput;
use crate::parser::TokenInput;
use crate::{CharOffset, LineNumber};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TokenMatch<'a> {
    pub(crate) token: Token,
    pub(crate) str_match: &'a str,
    pub(crate) line_number: LineNumber,
    pub(crate) line_offset: CharOffset,
}

impl BadInput for TokenMatch<'_> {
    fn line_number(&self) -> LineNumber {
        self.line_number
    }

    fn line_offset(&self) -> CharOffset {
        self.line_offset
    }
}

impl TokenInput for TokenMatch<'_> {
    fn input(&self) -> &str {
        self.str_match
    }
}

impl fmt::Display for TokenMatch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.str_match)
    }
}
