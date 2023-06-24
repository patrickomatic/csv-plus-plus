//! # DateTime
//!
//! A general type for handling all dates - whether it's a date-nnly, date+time or time.
//!
use chrono::serde::ts_milliseconds;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str;

use crate::Error;

// XXX this doesn't even implement Node
// TODO: make the acceptable formats more flexible
const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S %Z";

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct DateTime(
    #[serde(with = "ts_milliseconds")]
    pub chrono::DateTime<chrono::Utc>
);

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for DateTime {
    type Err = crate::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match chrono::NaiveDateTime::parse_from_str(input, DATE_FORMAT) {
            Ok(d) => Ok(Self(chrono::DateTime::from_utc(d, chrono::Utc))),
            Err(e) => Err(Error::CodeSyntaxError {
                message: format!("Unable to parse date: {}", e),
                bad_input: input.to_string(),
                line_number: 0, // XXX
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn display() {
        let date_time = chrono::DateTime::from_utc(
            chrono::NaiveDate::from_ymd_opt(2022, 10, 12).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            chrono::Utc,
        );
        let date = DateTime(date_time);

        assert_eq!("2022-10-12 00:00:00 UTC", date.to_string());
    }

    #[test]
    fn from_str() {
        let date_time = chrono::DateTime::from_utc(
            chrono::NaiveDate::from_ymd_opt(2022, 10, 12).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            chrono::Utc,
        );

        assert_eq!(DateTime(date_time), DateTime::from_str("2022-10-12 00:00:00 UTC").unwrap());
    }

    #[test]
    fn from_str_invalid() {
        assert!(DateTime::from_str("foo").is_err());
    }
}
