//! # TokenMatch
//!
use super::Token;
use crate::error::{BadInput, ParseError, ParseResult};
use crate::parser::TokenInput;
use crate::{CharOffset, LineNumber, SourceCode};
use std::fmt;
use std::sync;

#[derive(Clone, Debug)]
pub(crate) struct TokenMatch {
    pub(crate) token: Token,
    // TODO: turn into a &'a str
    pub(crate) str_match: String,
    pub(crate) position: a1_notation::Address,
    pub(crate) cell_offset: CharOffset,
    pub(crate) source_code: sync::Arc<SourceCode>,
}

impl TokenMatch {
    // TODO: make an actual Into impl
    pub(crate) fn into_number(self) -> ParseResult<isize> {
        self.str_match
            .parse::<isize>()
            .map_err(|e| self.into_parse_error(&format!("Unable to parse date: {e}")))
    }
}

impl fmt::Display for TokenMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.str_match)
    }
}

impl BadInput for TokenMatch {
    fn into_parse_error(self, message: &str) -> ParseError {
        let source_code = self.source_code.clone();
        source_code.parse_error(self, message)
    }

    fn line_number(&self) -> LineNumber {
        self.source_code.csv_line_number(self.position)
    }

    fn line_offset(&self) -> CharOffset {
        self.source_code.line_offset_for_cell(self.position) + self.cell_offset
    }
}

impl TokenInput for TokenMatch {
    fn input(&self) -> &str {
        self.str_match.as_str()
    }
}
