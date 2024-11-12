//! # `BorderSide`
//!
//! Represents a side (or all) of a spreadsheet cell.
//!
use crate::error::CellParseError;
use crate::parser::cell_lexer::TokenMatch;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BorderSide {
    All,
    Top,
    Bottom,
    Left,
    Right,
}

impl TryFrom<TokenMatch> for BorderSide {
    type Error = CellParseError;

    fn try_from(input: TokenMatch) -> Result<Self, Self::Error> {
        match input.str_match.to_lowercase().as_str() {
            "a" | "all" => Ok(Self::All),
            "t" | "top" => Ok(Self::Top),
            "b" | "bottom" => Ok(Self::Bottom),
            "l" | "left" => Ok(Self::Left),
            "r" | "right" => Ok(Self::Right),
            _ => Err(CellParseError::new(
                "border",
                input,
                &["all (a)", "top (t)", "bottom (b)", "left (l)", "right (r)"],
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from_all() {
        assert_eq!(
            BorderSide::All,
            BorderSide::try_from(build_cell_token_match("a")).unwrap()
        );
        assert_eq!(
            BorderSide::All,
            BorderSide::try_from(build_cell_token_match("all")).unwrap()
        );
        assert_eq!(
            BorderSide::All,
            BorderSide::try_from(build_cell_token_match("ALL")).unwrap()
        );
    }

    #[test]
    fn try_from_top() {
        assert_eq!(
            BorderSide::Top,
            BorderSide::try_from(build_cell_token_match("t")).unwrap()
        );
        assert_eq!(
            BorderSide::Top,
            BorderSide::try_from(build_cell_token_match("top")).unwrap()
        );
        assert_eq!(
            BorderSide::Top,
            BorderSide::try_from(build_cell_token_match("TOP")).unwrap()
        );
    }

    #[test]
    fn try_from_right() {
        assert_eq!(
            BorderSide::Right,
            BorderSide::try_from(build_cell_token_match("r")).unwrap()
        );
        assert_eq!(
            BorderSide::Right,
            BorderSide::try_from(build_cell_token_match("right")).unwrap()
        );
        assert_eq!(
            BorderSide::Right,
            BorderSide::try_from(build_cell_token_match("RIGHT")).unwrap()
        );
    }

    #[test]
    fn try_from_bottom() {
        assert_eq!(
            BorderSide::Bottom,
            BorderSide::try_from(build_cell_token_match("b")).unwrap()
        );
        assert_eq!(
            BorderSide::Bottom,
            BorderSide::try_from(build_cell_token_match("bottom")).unwrap()
        );
        assert_eq!(
            BorderSide::Bottom,
            BorderSide::try_from(build_cell_token_match("BOTTOM")).unwrap()
        );
    }

    #[test]
    fn try_from_left() {
        assert_eq!(
            BorderSide::Left,
            BorderSide::try_from(build_cell_token_match("l")).unwrap()
        );
        assert_eq!(
            BorderSide::Left,
            BorderSide::try_from(build_cell_token_match("left")).unwrap()
        );
        assert_eq!(
            BorderSide::Left,
            BorderSide::try_from(build_cell_token_match("LEFT")).unwrap()
        );
    }

    #[test]
    fn try_from_invalid() {
        assert!(BorderSide::try_from(build_cell_token_match("middle")).is_err());
        assert!(BorderSide::try_from(build_cell_token_match("123")).is_err());
    }
}
