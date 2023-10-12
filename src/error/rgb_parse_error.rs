//! # RgbParseError
//!
//! An error that can be thrown when parsing an RGB value.  This is very similar but slightly
//! different from `ModifierParseError` in that it's used in a lot more contexts and not
//! necessarily specific to parsing a single modifier.
use super::BadInput;
use crate::parser::modifier_lexer::TokenMatch;
use crate::{CharOffset, LineNumber, ParseError, SourceCode};
use std::error;
use std::fmt;

#[derive(Debug)]
pub(crate) struct RgbParseError {
    pub(crate) bad_input: TokenMatch,
    pub(crate) message: String,
}

impl RgbParseError {
    pub(crate) fn new(bad_input: TokenMatch, message: &str) -> RgbParseError {
        RgbParseError {
            bad_input,
            message: message.to_string(),
        }
    }

    pub(crate) fn into_parse_error(
        self,
        modifier_name: &str,
        source_code: &SourceCode,
    ) -> ParseError {
        let message = &self.message.clone();
        source_code.parse_error(
            self,
            &format!("Error parsing RGB value for `{modifier_name}` modifier: {message}"),
        )
    }
}

impl fmt::Display for RgbParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.bad_input.str_match)
    }
}

impl BadInput for RgbParseError {
    fn line_number(&self) -> LineNumber {
        self.bad_input.line_number
    }

    fn line_offset(&self) -> CharOffset {
        self.bad_input.line_offset
    }
}

impl error::Error for RgbParseError {}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO
}
