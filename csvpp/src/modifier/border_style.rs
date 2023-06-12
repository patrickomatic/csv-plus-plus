//!
use serde::{Serialize, Deserialize};
use std::str::FromStr;

use crate::Error;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BorderStyle {
    Dashed,
    Dotted,
    Double,
    Solid,
    SolidMedium,
    SolidThick,
}

impl FromStr for BorderStyle {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "dash" | "dashed"        => Ok(Self::Dashed),
            "dot"  | "dotted"        => Ok(Self::Dotted),
            "dbl"  | "double"        => Ok(Self::Double),
            "1"    | "solid"         => Ok(Self::Solid),
            "2"    | "solid_medium"  => Ok(Self::SolidMedium),
            "3"    | "solid_thick"   => Ok(Self::SolidThick),
            _ => Err(Error::InvalidModifier {
                message: "Invalid borderstyle= value".to_string(),
                bad_input: input.to_string(),
                possible_values: "dashed (dash) | dotted (dot) | double (dbl) \
                                    | solid (1) | solid_medium (2) | solid_thick (3)".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_dashed() {
        assert_eq!(BorderStyle::Dashed, BorderStyle::from_str("dash").unwrap());
        assert_eq!(BorderStyle::Dashed, BorderStyle::from_str("dashed").unwrap());
        assert_eq!(BorderStyle::Dashed, BorderStyle::from_str("DASHED").unwrap());
    }

    #[test]
    fn from_str_dotted() {
        assert_eq!(BorderStyle::Dotted, BorderStyle::from_str("dot").unwrap());
        assert_eq!(BorderStyle::Dotted, BorderStyle::from_str("dotted").unwrap());
        assert_eq!(BorderStyle::Dotted, BorderStyle::from_str("DOTTED").unwrap());
    }

    #[test]
    fn from_str_double() {
        assert_eq!(BorderStyle::Double, BorderStyle::from_str("dbl").unwrap());
        assert_eq!(BorderStyle::Double, BorderStyle::from_str("double").unwrap());
        assert_eq!(BorderStyle::Double, BorderStyle::from_str("DOUBLE").unwrap());
    }

    #[test]
    fn from_str_solid() {
        assert_eq!(BorderStyle::Solid, BorderStyle::from_str("1").unwrap());
        assert_eq!(BorderStyle::Solid, BorderStyle::from_str("solid").unwrap());
        assert_eq!(BorderStyle::Solid, BorderStyle::from_str("SOLID").unwrap());
    }

    #[test]
    fn from_str_solid_medium() {
        assert_eq!(BorderStyle::SolidMedium, BorderStyle::from_str("2").unwrap());
        assert_eq!(BorderStyle::SolidMedium, BorderStyle::from_str("solid_medium").unwrap());
        assert_eq!(BorderStyle::SolidMedium, BorderStyle::from_str("SOLID_MEDIUM").unwrap());
    }

    #[test]
    fn from_str_solid_thick() {
        assert_eq!(BorderStyle::SolidThick, BorderStyle::from_str("3").unwrap());
        assert_eq!(BorderStyle::SolidThick, BorderStyle::from_str("solid_thick").unwrap());
        assert_eq!(BorderStyle::SolidThick, BorderStyle::from_str("SOLID_THICK").unwrap());
    }

    #[test]
    fn from_str_invalid() {
        assert!(BorderStyle::from_str("foo").is_err());
    }
}
