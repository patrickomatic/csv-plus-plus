//! # `DateTime`
//!
//! How we handle dates and times (or both together).  This is mostly a wrapper around `chrono` but
//! it's important that we handle timezones and parsing uniformly across the app.
mod display;
mod try_from;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DateTime {
    Date(chrono::NaiveDate),
    DateAndTime(chrono::NaiveDateTime),
    Time(chrono::NaiveTime),
}

fn date_epoch() -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()
}

impl DateTime {
    /// Kinda odd but Excel references Dates as days since January 1, 1900 (we don't support other
    /// use-cases (for now))
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
