//! # `VerticalAlign`
//!
use crate::error::CellParseError;
use crate::parser::cell_lexer::TokenMatch;

#[derive(Copy, Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum VerticalAlign {
    Bottom,
    #[default]
    Center,
    Top,
}

impl TryFrom<TokenMatch> for VerticalAlign {
    type Error = CellParseError;

    fn try_from(input: TokenMatch) -> Result<Self, Self::Error> {
        match input.str_match.to_lowercase().as_str() {
            "b" | "bottom" => Ok(Self::Bottom),
            "c" | "center" => Ok(Self::Center),
            "t" | "top" => Ok(Self::Top),
            _ => Err(CellParseError::new(
                "valign",
                input,
                &["bottom (b)", "center (c)", "top (t)"],
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from_top() {
        assert_eq!(
            VerticalAlign::Top,
            VerticalAlign::try_from(build_cell_token_match("t")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Top,
            VerticalAlign::try_from(build_cell_token_match("top")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Top,
            VerticalAlign::try_from(build_cell_token_match("TOP")).unwrap()
        );
    }

    #[test]
    fn try_from_center() {
        assert_eq!(
            VerticalAlign::Center,
            VerticalAlign::try_from(build_cell_token_match("c")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Center,
            VerticalAlign::try_from(build_cell_token_match("center")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Center,
            VerticalAlign::try_from(build_cell_token_match("CENTER")).unwrap()
        );
    }

    #[test]
    fn try_from_bottom() {
        assert_eq!(
            VerticalAlign::Bottom,
            VerticalAlign::try_from(build_cell_token_match("b")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Bottom,
            VerticalAlign::try_from(build_cell_token_match("bottom")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Bottom,
            VerticalAlign::try_from(build_cell_token_match("BOTTOM")).unwrap()
        );
    }
}
