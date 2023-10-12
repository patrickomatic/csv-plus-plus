//! # BorderStyle
//!
use crate::error::ModifierParseError;
use crate::parser::modifier_lexer::TokenMatch;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BorderStyle {
    Dashed,
    Dotted,
    Double,
    Solid,
    SolidMedium,
    SolidThick,
}

impl TryFrom<TokenMatch> for BorderStyle {
    type Error = ModifierParseError;

    fn try_from(input: TokenMatch) -> Result<Self, Self::Error> {
        match input.str_match.to_lowercase().as_str() {
            "dash" | "dashed" => Ok(Self::Dashed),
            "dot" | "dotted" => Ok(Self::Dotted),
            "dbl" | "double" => Ok(Self::Double),
            "1" | "solid" => Ok(Self::Solid),
            "2" | "solid_medium" => Ok(Self::SolidMedium),
            "3" | "solid_thick" => Ok(Self::SolidThick),
            _ => Err(ModifierParseError::new(
                "borderstyle",
                input,
                Some(&[
                    "dashed (dash)",
                    "dotted (dot)",
                    "double (dbl)",
                    "solid (1)",
                    "solid_medium (2)",
                    "solid_thick (3)",
                ]),
            )),
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::Solid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from_dashed() {
        assert_eq!(
            BorderStyle::Dashed,
            BorderStyle::try_from(build_modifier_token_match("dash")).unwrap()
        );
        assert_eq!(
            BorderStyle::Dashed,
            BorderStyle::try_from(build_modifier_token_match("dashed")).unwrap()
        );
        assert_eq!(
            BorderStyle::Dashed,
            BorderStyle::try_from(build_modifier_token_match("DASHED")).unwrap()
        );
    }

    #[test]
    fn try_from_dotted() {
        assert_eq!(
            BorderStyle::Dotted,
            BorderStyle::try_from(build_modifier_token_match("dot")).unwrap()
        );
        assert_eq!(
            BorderStyle::Dotted,
            BorderStyle::try_from(build_modifier_token_match("dotted")).unwrap()
        );
        assert_eq!(
            BorderStyle::Dotted,
            BorderStyle::try_from(build_modifier_token_match("DOTTED")).unwrap()
        );
    }

    #[test]
    fn try_from_double() {
        assert_eq!(
            BorderStyle::Double,
            BorderStyle::try_from(build_modifier_token_match("dbl")).unwrap()
        );
        assert_eq!(
            BorderStyle::Double,
            BorderStyle::try_from(build_modifier_token_match("double")).unwrap()
        );
        assert_eq!(
            BorderStyle::Double,
            BorderStyle::try_from(build_modifier_token_match("DOUBLE")).unwrap()
        );
    }

    #[test]
    fn try_from_solid() {
        assert_eq!(
            BorderStyle::Solid,
            BorderStyle::try_from(build_modifier_token_match("1")).unwrap()
        );
        assert_eq!(
            BorderStyle::Solid,
            BorderStyle::try_from(build_modifier_token_match("solid")).unwrap()
        );
        assert_eq!(
            BorderStyle::Solid,
            BorderStyle::try_from(build_modifier_token_match("SOLID")).unwrap()
        );
    }

    #[test]
    fn try_from_solid_medium() {
        assert_eq!(
            BorderStyle::SolidMedium,
            BorderStyle::try_from(build_modifier_token_match("2")).unwrap()
        );
        assert_eq!(
            BorderStyle::SolidMedium,
            BorderStyle::try_from(build_modifier_token_match("solid_medium")).unwrap()
        );
        assert_eq!(
            BorderStyle::SolidMedium,
            BorderStyle::try_from(build_modifier_token_match("SOLID_MEDIUM")).unwrap()
        );
    }

    #[test]
    fn try_from_solid_thick() {
        assert_eq!(
            BorderStyle::SolidThick,
            BorderStyle::try_from(build_modifier_token_match("3")).unwrap()
        );
        assert_eq!(
            BorderStyle::SolidThick,
            BorderStyle::try_from(build_modifier_token_match("solid_thick")).unwrap()
        );
        assert_eq!(
            BorderStyle::SolidThick,
            BorderStyle::try_from(build_modifier_token_match("SOLID_THICK")).unwrap()
        );
    }

    #[test]
    fn try_from_invalid() {
        assert!(BorderStyle::try_from(build_modifier_token_match("foo")).is_err());
    }
}
