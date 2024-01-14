use super::DateTime;
use std::fmt;

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DateAndTime(dt) => write!(f, "{dt}"),
            Self::Time(t) => write!(f, "{t}"),
            Self::Date(d) => write!(f, "{d}"),
            Self::NaiveDateAndTime(dt) => write!(f, "{dt}"),
        }
    }
}
