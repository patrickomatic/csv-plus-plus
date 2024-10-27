//! # `UnknownToken`
//!
use crate::error::{BadInput, ParseError};
use crate::{compiler_error, ArcSourceCode, CharOffset};
use csvp::Field;
use std::fmt;

#[derive(Debug)]
pub(crate) struct UnknownToken {
    pub(crate) bad_input: String,
    pub(crate) field: Field,
    pub(crate) cell_offset: CharOffset,
    pub(crate) source_code: ArcSourceCode,
}

impl fmt::Display for UnknownToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unrecognized token `{}`", self.bad_input)
    }
}

impl BadInput for UnknownToken {
    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
        self.source_code.parse_error(&self, message)
    }

    fn position(&self) -> csvp::SourcePosition {
        self.field
            .position_for_offset(self.cell_offset)
            .unwrap_or_else(|| {
                compiler_error(format!(
                    "Error getting position of {} from {:?}.",
                    self.cell_offset, self.field,
                ))
            })
    }
}

impl From<UnknownToken> for ParseError {
    fn from(u: UnknownToken) -> Self {
        u.into_parse_error("Unrecognized token")
    }
}
