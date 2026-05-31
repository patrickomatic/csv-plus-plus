//! # `CellParseError`
//!
//! An error that can be thrown when parsing a cell.
use super::BadInput;
use crate::parser::cell_lexer::TokenMatch;
use crate::ParseError;
use std::{error, fmt};

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
                .map(|pv| (*pv).to_string())
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
    fn position(&self) -> csvp::SourcePosition {
        self.bad_input.position()
    }

    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
        self.bad_input.source_code.parse_error_with_possible_values(
            &self,
            message,
            self.possible_values.clone(),
        )
    }
}

impl error::Error for CellParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn new_stores_fields() {
        let err = CellParseError::new("my_option", build_cell_token_match("bad_val"), &["a", "b"]);
        assert_eq!(err.option_name, "my_option");
        assert_eq!(err.bad_input.str_match, "bad_val");
        assert_eq!(err.possible_values, vec!["a", "b"]);
    }

    #[test]
    fn display() {
        let err = CellParseError::new("align", build_cell_token_match("bad"), &[]);
        assert_eq!(format!("{err}"), "`bad`");
    }

    #[test]
    fn from_cell_parse_error_to_parse_error() {
        let err = CellParseError::new("align", build_cell_token_match("bad"), &["left", "right"]);
        let parse_error = ParseError::from(err);
        assert!(parse_error.message.contains("align"));
        assert_eq!(
            parse_error.possible_values.unwrap(),
            vec!["left".to_string(), "right".to_string()]
        );
    }
}
