//! # `DateTime::TryFrom`
//!
//! Uniform date-handling
//!
use super::DateTime;
use crate::error::{BadInput, ParseError, ParseResult};
use crate::parser::{ast_lexer, cell_lexer, TokenInput};

const DATE_TIME: &[&str] = &[
    "%Y-%m-%d %H:%M:%S",
    "%m/%d/%Y %H:%M:%S",
    "%Y-%m-%d %H:%M",
    "%m/%d/%Y %H:%M",
];

const DATE: &[&str] = &["%y-%m-%d", "%m/%d/%y", "%Y-%m-%d", "%m/%d/%Y"];

const TIME: &[&str] = &["%H:%M:%S.%Z", "%H:%M:%S", "%H:%M"];

macro_rules! try_formats {
    ($input:ident, $formats:ident, $target:expr, $chrono_target:path) => {{
        for format in $formats {
            if let Ok(res) = $chrono_target($input.input(), format) {
                return Ok($target(res));
            }
        }
    }};
}

fn token_into(input: impl BadInput + TokenInput) -> ParseResult<DateTime> {
    try_formats!(
        input,
        DATE_TIME,
        DateTime::DateAndTime,
        chrono::NaiveDateTime::parse_from_str
    );
    try_formats!(
        input,
        DATE,
        DateTime::Date,
        chrono::NaiveDate::parse_from_str
    );
    try_formats!(
        input,
        TIME,
        DateTime::Time,
        chrono::NaiveTime::parse_from_str
    );

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

    fn build_input(s: &str, source_code: ArcSourceCode) -> ast_lexer::TokenMatch {
        build_ast_token_match(s, source_code)
    }

    #[test]
    fn naive_date() {
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
            ),
        );
    }

    #[test]
    fn naive_time() {
        assert_eq!(
            DateTime::try_from(build_input("1:00", build_source_code())).unwrap(),
            DateTime::Time(chrono::NaiveTime::from_hms_opt(1, 0, 0).unwrap()),
        );
    }
}
