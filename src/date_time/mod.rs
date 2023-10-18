//! # DateTime
//!
//! How we handle dates and times (or both together).  This is mostly a wrapper around `chrono` but
//! it's important that we handle timezones and parsing uniformly across the app.
use crate::error::{BadInput, ParseResult};
use crate::parser::TokenInput;
use crate::SourceCode;
use serde::{Deserialize, Serialize};

mod display;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DateTime {
    // Just a date.  No timezone is used.
    Date(chrono::NaiveDate),

    // A date and a time.  A timezone will attempt to be parsed but if not supplied it will assume
    // the local TZ
    DateAndTime(chrono::DateTime<chrono::FixedOffset>),

    // Just a time, no timezone (since it doesn't make sense in the context of just a time)
    Time(chrono::NaiveTime),
}

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

impl DateTime {
    pub(crate) fn from_token_input(
        input: impl BadInput + TokenInput,
        source_code: &SourceCode,
    ) -> ParseResult<Self> {
        for format in DATE_TIME_WITH_TZ {
            if let Ok(d) = chrono::DateTime::parse_from_str(input.input(), format) {
                return Ok(Self::DateAndTime(d));
            }
        }

        for format in DATE_TIME {
            if let Ok(d) = chrono::NaiveDateTime::parse_from_str(input.input(), format) {
                // TODO: deal with the unwrap and present a better error
                return Ok(Self::DateAndTime(
                    d.and_local_timezone(chrono::Local).unwrap().into(),
                ));
            }
        }

        for format in DATE {
            if let Ok(d) = chrono::NaiveDate::parse_from_str(input.input(), format) {
                return Ok(Self::Date(d));
            }
        }

        for format in TIME {
            if let Ok(t) = chrono::NaiveTime::parse_from_str(input.input(), format) {
                return Ok(Self::Time(t));
            }
        }

        Err(source_code.parse_error(input, "Unable to parse date"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_lexer;
    use crate::test_utils::*;

    fn build_input(s: &str) -> ast_lexer::TokenMatch {
        build_ast_token_match(s)
    }

    #[test]
    fn date() {
        assert_eq!(
            DateTime::from_token_input(build_input("10/22/2012"), &build_source_code()).unwrap(),
            DateTime::Date(chrono::NaiveDate::from_ymd_opt(2012, 10, 22).unwrap()),
        );

        assert_eq!(
            DateTime::from_token_input(build_input("2012-10-22"), &build_source_code()).unwrap(),
            DateTime::Date(chrono::NaiveDate::from_ymd_opt(2012, 10, 22).unwrap()),
        );
    }

    #[test]
    fn date_and_time() {
        assert_eq!(
            DateTime::from_token_input(build_input("10/22/2012 1:00"), &build_source_code())
                .unwrap(),
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
            DateTime::from_token_input(build_input("10/22/2012 1:00 0800"), &build_source_code())
                .unwrap(),
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
            DateTime::from_token_input(build_input("1:00"), &build_source_code()).unwrap(),
            DateTime::Time(chrono::NaiveTime::from_hms_opt(1, 0, 0).unwrap()),
        );
    }
}
