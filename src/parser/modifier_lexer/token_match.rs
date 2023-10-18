//! # TokenMatch
//!
use super::Token;
use crate::error::{BadInput, ParseResult};
use crate::parser::TokenInput;
use crate::{CharOffset, LineNumber, SourceCode};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TokenMatch {
    pub(crate) token: Token,
    pub(crate) str_match: String,
    pub(crate) line_offset: CharOffset,
    pub(crate) line_number: LineNumber,
}

impl TokenMatch {
    pub(crate) fn into_number(self, source_code: &SourceCode) -> ParseResult<isize> {
        self.str_match
            .parse::<isize>()
            .map_err(|e| source_code.parse_error(self, &format!("Unable to parse date: {e}")))
    }
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

impl TokenInput for TokenMatch {
    fn input(&self) -> &str {
        self.str_match.as_str()
    }
}
