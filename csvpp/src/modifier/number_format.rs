//! # NumberFormat
use serde::{Serialize, Deserialize};
use std::str::FromStr;

use crate::Error;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum NumberFormat {
    Currency,
    Date,
    DateTime,
    Number,
    Percent,
    Text,
    Time,
    Scientific,
}

impl FromStr for NumberFormat {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "c"  | "currency"      => Ok(Self::Currency),
            "d"  | "date"          => Ok(Self::Date),
            "dt" | "datetime"      => Ok(Self::DateTime),
            "n"  | "number"        => Ok(Self::Number),
            "p"  | "percent"       => Ok(Self::Percent),
            "text"                 => Ok(Self::Text),  // TODO: think of a shortcut!!!
            "t"  | "time"          => Ok(Self::Time),
            "s"  | "scientific"    => Ok(Self::Scientific),
            _ => Err(Error::InvalidModifier { 
                message: "Invalid numberformat= value".to_string(),
                bad_input: input.to_string(),
                possible_values: "currency (c) | date (d) | datetime (dt) | number (n) | percent (p) \
                                    | text | time (t) | scientific (s)".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_currency() {
        assert_eq!(NumberFormat::Currency, NumberFormat::from_str("c").unwrap());
        assert_eq!(NumberFormat::Currency, NumberFormat::from_str("currency").unwrap());
        assert_eq!(NumberFormat::Currency, NumberFormat::from_str("CURRENCY").unwrap());
    }

    #[test]
    fn from_str_date() {
        assert_eq!(NumberFormat::Date, NumberFormat::from_str("d").unwrap());
        assert_eq!(NumberFormat::Date, NumberFormat::from_str("date").unwrap());
        assert_eq!(NumberFormat::Date, NumberFormat::from_str("DATE").unwrap());
    }

    #[test]
    fn from_str_datetime() {
        assert_eq!(NumberFormat::DateTime, NumberFormat::from_str("dt").unwrap());
        assert_eq!(NumberFormat::DateTime, NumberFormat::from_str("datetime").unwrap());
        assert_eq!(NumberFormat::DateTime, NumberFormat::from_str("DATETIME").unwrap());
    }

    #[test]
    fn from_str_number() {
        assert_eq!(NumberFormat::Number, NumberFormat::from_str("n").unwrap());
        assert_eq!(NumberFormat::Number, NumberFormat::from_str("number").unwrap());
        assert_eq!(NumberFormat::Number, NumberFormat::from_str("NUMBER").unwrap());
    }

    #[test]
    fn from_str_percent() {
        assert_eq!(NumberFormat::Percent, NumberFormat::from_str("p").unwrap());
        assert_eq!(NumberFormat::Percent, NumberFormat::from_str("percent").unwrap());
        assert_eq!(NumberFormat::Percent, NumberFormat::from_str("PERCENT").unwrap());
    }

    #[test]
    fn from_str_text() {
        assert_eq!(NumberFormat::Text, NumberFormat::from_str("text").unwrap());
        assert_eq!(NumberFormat::Text, NumberFormat::from_str("TEXT").unwrap());
    }

    #[test]
    fn from_str_time() {
        assert_eq!(NumberFormat::Time, NumberFormat::from_str("t").unwrap());
        assert_eq!(NumberFormat::Time, NumberFormat::from_str("time").unwrap());
        assert_eq!(NumberFormat::Time, NumberFormat::from_str("TIME").unwrap());
    }

    #[test]
    fn from_str_scientific() {
        assert_eq!(NumberFormat::Scientific, NumberFormat::from_str("s").unwrap());
        assert_eq!(NumberFormat::Scientific, NumberFormat::from_str("scientific").unwrap());
        assert_eq!(NumberFormat::Scientific, NumberFormat::from_str("SCIENTIFIC").unwrap());
    }

    #[test]
    fn from_str_invalid() {
        assert!(NumberFormat::from_str("foo").is_err());
    }
}
