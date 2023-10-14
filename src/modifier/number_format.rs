//! # NumberFormat
use crate::error::ModifierParseError;
use crate::parser::modifier_lexer::TokenMatch;
use serde::{Deserialize, Serialize};

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

impl TryFrom<TokenMatch> for NumberFormat {
    type Error = ModifierParseError;

    fn try_from(input: TokenMatch) -> Result<Self, Self::Error> {
        match input.str_match.to_lowercase().as_str() {
            "c" | "currency" => Ok(Self::Currency),
            "d" | "date" => Ok(Self::Date),
            "dt" | "datetime" => Ok(Self::DateTime),
            "n" | "number" => Ok(Self::Number),
            "p" | "percent" => Ok(Self::Percent),
            "text" => Ok(Self::Text), // TODO: think of a shortcut!!!
            "t" | "time" => Ok(Self::Time),
            "s" | "scientific" => Ok(Self::Scientific),
            _ => Err(ModifierParseError::new(
                "numberformat",
                input,
                &[
                    "currency (c)",
                    "date (d)",
                    "datetime (dt)",
                    "number (n)",
                    "percent (p)",
                    "text",
                    "time (t)",
                    "scientific (s)",
                ],
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from_currency() {
        assert_eq!(
            NumberFormat::Currency,
            NumberFormat::try_from(build_modifier_token_match("c")).unwrap()
        );
        assert_eq!(
            NumberFormat::Currency,
            NumberFormat::try_from(build_modifier_token_match("currency")).unwrap()
        );
        assert_eq!(
            NumberFormat::Currency,
            NumberFormat::try_from(build_modifier_token_match("CURRENCY")).unwrap()
        );
    }

    #[test]
    fn try_from_date() {
        assert_eq!(
            NumberFormat::Date,
            NumberFormat::try_from(build_modifier_token_match("d")).unwrap()
        );
        assert_eq!(
            NumberFormat::Date,
            NumberFormat::try_from(build_modifier_token_match("date")).unwrap()
        );
        assert_eq!(
            NumberFormat::Date,
            NumberFormat::try_from(build_modifier_token_match("DATE")).unwrap()
        );
    }

    #[test]
    fn try_from_datetime() {
        assert_eq!(
            NumberFormat::DateTime,
            NumberFormat::try_from(build_modifier_token_match("dt")).unwrap()
        );
        assert_eq!(
            NumberFormat::DateTime,
            NumberFormat::try_from(build_modifier_token_match("datetime")).unwrap()
        );
        assert_eq!(
            NumberFormat::DateTime,
            NumberFormat::try_from(build_modifier_token_match("DATETIME")).unwrap()
        );
    }

    #[test]
    fn try_from_number() {
        assert_eq!(
            NumberFormat::Number,
            NumberFormat::try_from(build_modifier_token_match("n")).unwrap()
        );
        assert_eq!(
            NumberFormat::Number,
            NumberFormat::try_from(build_modifier_token_match("number")).unwrap()
        );
        assert_eq!(
            NumberFormat::Number,
            NumberFormat::try_from(build_modifier_token_match("NUMBER")).unwrap()
        );
    }

    #[test]
    fn try_from_percent() {
        assert_eq!(
            NumberFormat::Percent,
            NumberFormat::try_from(build_modifier_token_match("p")).unwrap()
        );
        assert_eq!(
            NumberFormat::Percent,
            NumberFormat::try_from(build_modifier_token_match("percent")).unwrap()
        );
        assert_eq!(
            NumberFormat::Percent,
            NumberFormat::try_from(build_modifier_token_match("PERCENT")).unwrap()
        );
    }

    #[test]
    fn try_from_text() {
        assert_eq!(
            NumberFormat::Text,
            NumberFormat::try_from(build_modifier_token_match("text")).unwrap()
        );
        assert_eq!(
            NumberFormat::Text,
            NumberFormat::try_from(build_modifier_token_match("TEXT")).unwrap()
        );
    }

    #[test]
    fn try_from_time() {
        assert_eq!(
            NumberFormat::Time,
            NumberFormat::try_from(build_modifier_token_match("t")).unwrap()
        );
        assert_eq!(
            NumberFormat::Time,
            NumberFormat::try_from(build_modifier_token_match("time")).unwrap()
        );
        assert_eq!(
            NumberFormat::Time,
            NumberFormat::try_from(build_modifier_token_match("TIME")).unwrap()
        );
    }

    #[test]
    fn try_from_scientific() {
        assert_eq!(
            NumberFormat::Scientific,
            NumberFormat::try_from(build_modifier_token_match("s")).unwrap()
        );
        assert_eq!(
            NumberFormat::Scientific,
            NumberFormat::try_from(build_modifier_token_match("scientific")).unwrap()
        );
        assert_eq!(
            NumberFormat::Scientific,
            NumberFormat::try_from(build_modifier_token_match("SCIENTIFIC")).unwrap()
        );
    }

    #[test]
    fn try_from_invalid() {
        assert!(NumberFormat::try_from(build_modifier_token_match("foo")).is_err());
    }
}
