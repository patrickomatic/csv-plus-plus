use crate::error::{BadInput, ParseError};
use crate::{CharOffset, LineNumber, SourceCode};
use std::fmt;

#[derive(Debug)]
pub(crate) struct UnknownToken<'a> {
    pub(crate) bad_input: String,
    pub(crate) line_number: LineNumber,
    pub(crate) line_offset: CharOffset,
    pub(crate) source_code: &'a SourceCode,
}

impl fmt::Display for UnknownToken<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut shortened_bad_input = self.bad_input.clone();
        shortened_bad_input.truncate(50);
        write!(f, "{shortened_bad_input}")
    }
}

impl BadInput for UnknownToken<'_> {
    fn line_number(&self) -> LineNumber {
        self.line_number
    }

    fn line_offset(&self) -> CharOffset {
        self.line_offset
    }

    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
        self.source_code.parse_error(self, message)
    }
}

impl From<UnknownToken<'_>> for ParseError {
    fn from(u: UnknownToken) -> Self {
        u.into_parse_error("Error parsing input - invalid token")
    }
}
