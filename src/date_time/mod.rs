//! # DateTime
//!
//! How we handle dates and times (or both together).  This is mostly a wrapper around `chrono` but
//! it's important that we handle timezones and parsing uniformly across the app.
use serde::{Deserialize, Serialize};

mod display;
mod try_from;

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

fn date_epoch() -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()
}

impl DateTime {
    /// Excel references Dates since January 1, 1900
    pub(crate) fn distance_from_epoch(&self) -> i64 {
        match self {
            Self::Date(d) => d.signed_duration_since(date_epoch()).num_days(),
            // TODO: nothing uses these... but this is kinda a footgun, we should implement it
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_from_epoch_date() {
        assert_eq!(
            DateTime::Date(chrono::NaiveDate::from_ymd_opt(2012, 10, 22).unwrap())
                .distance_from_epoch(),
            41202
        );
    }
}
