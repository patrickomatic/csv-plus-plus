//! # DataValidation
//!
//! * [Google
//! Sheets](https://developers.google.com/apps-script/reference/spreadsheet/data-validation-criteria)
//! * [Umya
//! Spreadsheet](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/structs/enum.DataValidationValues.html)
//!
use crate::ast::Ast;
use crate::error::{ModifierParseError, ModifierParseResult};
use crate::parser::modifier_lexer::TokenMatch;
use serde::{Deserialize, Serialize};

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DataValidation {
    // [[f=b / var=foo / dv=custom('=fomular')
    // [[f=b / var=foo / dv=date_after('3022-02-02')
    // [[f=b / var=foo / dv=date_between('3022-02-02','2033-02-02')
    // [[f=b / var=foo / dv=text_contains('foo bar')
    // [[f=b / var=foo / dv=number_between(1, 100)
    // [[f=b / var=foo / dv=value_in_list(1, 2, 3, 4)
    Custom(String),
    DateAfter(DateTime),
    DateBefore(DateTime),
    DateBetween(DateTime, DateTime),
    DateEqualTo(DateTime),
    DateIsValid,
    DateNotBetween(DateTime, DateTime),
    DateOnOrAfter(DateTime),
    DateOnOrBefore(DateTime),
    NumberBetween(isize, isize),
    NumberEqualTo(isize),
    NumberGreaterThan(isize),
    NumberGreaterThanOrEqualTo(isize),
    NumberLessThanOrEqualTo(isize),
    NumberLessThan(isize),
    NumberNotBetween(isize, isize),
    NumberNotEqualTo(isize),
    TextContains(String),
    TextDoesNotContain(String),
    TextEqualTo(String),
    TextIsValidEmail,
    TextIsValidUrl,
    ValueInList(Vec<Ast>),
    ValueInRange,
}

/*
impl TryFrom<TokenMatch> for DataValidation {
    type Error = ModifierParseError;

    fn try_from(input: TokenMatch) -> Result<Self, Self::Error> {
        match input.str_match.to_lowercase().as_str() {
            /*
            "custom" => Ok(Self::Custom),
            "date_after" => Ok(Self::DateAfter),
            "date_before" => Ok(Self::DateBefore),
            "date_between" => Ok(Self::DateBetween),
            "date_equal" => Ok(Self::DateEqualTo),
            "date" => Ok(Self::DateIsValid),
            "date_not_between" => Ok(Self::DateNotBetween),
            */
            _ => Err(ModifierParseError::new("validate", input, &[])),
        }
    }
}
*/

/*
impl DataValidationParser {
    fn parse(lexer: &ModifierLexer) -> ModifierParseResult<DataValidation> {
        match
    }
}
*/
