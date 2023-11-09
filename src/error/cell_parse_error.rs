//! # CellParseError
//!
//! An error that can be thrown when parsing a cell.
use super::BadInput;
use crate::parser::cell_lexer::TokenMatch;
use crate::{CharOffset, LineNumber, ParseError};
use std::error;
use std::fmt;

#[derive(Debug)]
pub(crate) struct CellParseError {
    pub(crate) option_name: String,
    pub(crate) bad_input: TokenMatch,
    pub(crate) possible_values: Vec<String>,
}

impl CellParseError {
    pub(crate) fn new<S: Into<String>>(
        option_name: S,
        bad_input: TokenMatch,
        possible_values: &[&str],
    ) -> CellParseError {
        CellParseError {
            bad_input,
            option_name: option_name.into(),
            possible_values: possible_values
                .iter()
                .map(|pv| pv.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

impl From<CellParseError> for ParseError {
    fn from(e: CellParseError) -> Self {
        let option_name = e.option_name.clone();
        e.into_parse_error(format!(
            "received invalid value when parsing `{option_name}` option"
        ))
    }
}

impl fmt::Display for CellParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.bad_input.str_match)
    }
}

impl BadInput for CellParseError {
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

impl error::Error for CellParseError {}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO
}
