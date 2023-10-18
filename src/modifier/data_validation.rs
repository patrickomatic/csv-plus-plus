//! # DataValidation
//!
//! * [Google
//! Sheets](https://developers.google.com/apps-script/reference/spreadsheet/data-validation-criteria)
//! * [Umya
//! Spreadsheet](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/structs/enum.DataValidationValues.html)
//!
use crate::ast::Ast;
use crate::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DataValidation {
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
