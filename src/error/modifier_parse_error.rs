//! # ModifierParseError
//!
//! An error that can be thrown when parsing the value of a modifier.
use super::BadInput;
use crate::parser::modifier_lexer::TokenMatch;
use crate::{CharOffset, LineNumber, ParseError};
use std::error;
use std::fmt;

#[derive(Debug)]
pub(crate) struct ModifierParseError {
    pub(crate) modifier_name: String,
    pub(crate) bad_input: TokenMatch,
    pub(crate) possible_values: Vec<String>,
}

impl ModifierParseError {
    pub(crate) fn new(
        modifier_name: &str,
        bad_input: TokenMatch,
        possible_values: &[&str],
    ) -> ModifierParseError {
        ModifierParseError {
            bad_input,
            modifier_name: modifier_name.to_string(),
            possible_values: possible_values
                .iter()
                .map(|pv| pv.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

impl From<ModifierParseError> for ParseError {
    fn from(e: ModifierParseError) -> Self {
        let modifier_name = e.modifier_name.clone();
        e.into_parse_error(format!(
            "received invalid value when parsing `{modifier_name}` modifier"
        ))
    }
}

impl fmt::Display for ModifierParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.bad_input.str_match)
    }
}

impl BadInput for ModifierParseError {
    fn line_number(&self) -> LineNumber {
        self.bad_input.line_number()
    }

    fn line_offset(&self) -> CharOffset {
        self.bad_input.line_offset()
    }

    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
        let possible_values = self.possible_values.clone();
        let source_code = self.bad_input.source_code.clone();
        source_code.parse_error_with_possible_values(self, message, possible_values)
    }
}

impl error::Error for ModifierParseError {}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO
}
