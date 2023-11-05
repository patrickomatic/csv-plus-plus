//! # UnknownToken
//!
use crate::error::{BadInput, ParseError};
use crate::{CharOffset, LineNumber, SourceCode};
use std::fmt;
use std::sync;

#[derive(Debug)]
pub(crate) struct UnknownToken {
    pub(crate) bad_input: String,
    pub(crate) position: a1_notation::Address,
    pub(crate) cell_offset: CharOffset,
    pub(crate) source_code: sync::Arc<SourceCode>,
}

impl fmt::Display for UnknownToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unrecognized token `{}`", self.bad_input)
    }
}

impl BadInput for UnknownToken {
    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
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

impl From<UnknownToken> for ParseError {
    fn from(u: UnknownToken) -> Self {
        u.into_parse_error("Unrecognized token")
    }
}
