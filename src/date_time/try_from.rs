//! # `DateTime::TryFrom`
//!
//! Uniform date-handling
//!
use super::DateTime;
use crate::error::{BadInput, ParseError, ParseResult};
use crate::parser::{ast_lexer, cell_lexer, TokenInput};

// I wish chrono had some kind of smart string parser but it seems like it's up to me to handle all
// the nuances of different types and supported patterns
const DATE_TIME_WITH_TZ: &[&str] = &[
    "%Y-%m-%d %H:%M:%S %Z",
    "%m/%d/%Y %H:%M:%S %Z",
    "%Y-%m-%d %H:%M %Z",
    "%m/%d/%Y %H:%M %Z",
];

const DATE_TIME: &[&str] = &[
    "%Y-%m-%d %H:%M:%S",
    "%m/%d/%Y %H:%M:%S",
    "%Y-%m-%d %H:%M",
    "%m/%d/%Y %H:%M",
];

const DATE: &[&str] = &["%y-%m-%d", "%m/%d/%y", "%Y-%m-%d", "%m/%d/%Y"];

const TIME: &[&str] = &["%H:%M:%S.%Z", "%H:%M:%S", "%H:%M"];

fn token_into(input: impl BadInput + TokenInput) -> ParseResult<DateTime> {
    for format in DATE_TIME_WITH_TZ {
        if let Ok(d) = chrono::DateTime::parse_from_str(input.input(), format) {
            return Ok(DateTime::DateAndTime(d));
        }
    }

    for format in DATE_TIME {
        if let Ok(d) = chrono::NaiveDateTime::parse_from_str(input.input(), format) {
            // TODO: deal with the unwrap and present a better error
            return Ok(DateTime::DateAndTime(
                d.and_local_timezone(chrono::Local).unwrap().into(),
            ));
        }
    }

    for format in DATE {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(input.input(), format) {
            return Ok(DateTime::Date(d));
        }
    }

    for format in TIME {
        if let Ok(t) = chrono::NaiveTime::parse_from_str(input.input(), format) {
            return Ok(DateTime::Time(t));
        }
    }

    Err(input.into_parse_error("Unable to parse date"))
}

impl TryFrom<ast_lexer::TokenMatch<'_>> for DateTime {
    type Error = ParseError;
    fn try_from(input: ast_lexer::TokenMatch) -> ParseResult<Self> {
        token_into(input)
    }
}

impl TryFrom<cell_lexer::TokenMatch> for DateTime {
    type Error = ParseError;
    fn try_from(input: cell_lexer::TokenMatch) -> ParseResult<Self> {
        token_into(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_lexer;
    use crate::test_utils::*;
    use crate::*;

    fn build_input<'a>(s: &'a str, source_code: ArcSourceCode) -> ast_lexer::TokenMatch<'a> {
        build_ast_token_match(s, source_code)
    }

    #[test]
    fn date() {
        let source_code = build_source_code();

        assert_eq!(
            DateTime::try_from(build_input("10/22/2012", source_code.clone())).unwrap(),
            DateTime::Date(chrono::NaiveDate::from_ymd_opt(2012, 10, 22).unwrap()),
        );

        assert_eq!(
            DateTime::try_from(build_input("2012-10-22", source_code.clone())).unwrap(),
            DateTime::Date(chrono::NaiveDate::from_ymd_opt(2012, 10, 22).unwrap()),
        );
    }

    #[test]
    fn date_and_time() {
        assert_eq!(
            DateTime::try_from(build_input("10/22/2012 1:00", build_source_code())).unwrap(),
            DateTime::DateAndTime(
                chrono::NaiveDate::from_ymd_opt(2012, 10, 22)
                    .unwrap()
                    .and_hms_opt(1, 0, 0)
                    .unwrap()
                    .and_local_timezone(chrono::Local)
                    .unwrap()
                    .into()
            ),
        );
    }

    #[ignore]
    #[test]
    fn date_and_time_and_timezone() {
        assert_eq!(
            DateTime::try_from(build_input("10/22/2012 1:00 0800", build_source_code())).unwrap(),
            DateTime::DateAndTime(
                chrono::NaiveDate::from_ymd_opt(2012, 10, 22)
                    .unwrap()
                    .and_hms_opt(1, 0, 0)
                    .unwrap()
                    .and_local_timezone(chrono::FixedOffset::west_opt(-8 * 3600).unwrap())
                    .unwrap()
            ),
        );
    }

    #[test]
    fn time() {
        assert_eq!(
            DateTime::try_from(build_input("1:00", build_source_code())).unwrap(),
            DateTime::Time(chrono::NaiveTime::from_hms_opt(1, 0, 0).unwrap()),
        );
    }
}
