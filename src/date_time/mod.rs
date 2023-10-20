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
