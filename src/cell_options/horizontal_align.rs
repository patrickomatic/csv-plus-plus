//! # `HorizontalAlign`
use crate::error::CellParseError;
use crate::parser::cell_lexer::TokenMatch;

#[derive(Copy, Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum HorizontalAlign {
    #[default]
    Center,
    Left,
    Right,
}

impl TryFrom<TokenMatch> for HorizontalAlign {
    type Error = CellParseError;

    fn try_from(input: TokenMatch) -> Result<Self, Self::Error> {
        match input.str_match.to_lowercase().as_str() {
            "c" | "center" => Ok(Self::Center),
            "l" | "left" => Ok(Self::Left),
            "r" | "right" => Ok(Self::Right),
            _ => Err(CellParseError::new(
                "halign",
                input,
                &["center (c)", "left (l)", "right (r)"],
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from_right() {
        assert_eq!(
            HorizontalAlign::Right,
            HorizontalAlign::try_from(build_cell_token_match("r")).unwrap()
        );
        assert_eq!(
            HorizontalAlign::Right,
            HorizontalAlign::try_from(build_cell_token_match("right")).unwrap()
        );
        assert_eq!(
            HorizontalAlign::Right,
            HorizontalAlign::try_from(build_cell_token_match("RIGHT")).unwrap()
        );
    }

    #[test]
    fn try_from_center() {
        assert_eq!(
            HorizontalAlign::Center,
            HorizontalAlign::try_from(build_cell_token_match("c")).unwrap()
        );
        assert_eq!(
            HorizontalAlign::Center,
            HorizontalAlign::try_from(build_cell_token_match("center")).unwrap()
        );
        assert_eq!(
            HorizontalAlign::Center,
            HorizontalAlign::try_from(build_cell_token_match("CENTER")).unwrap()
        );
    }

    #[test]
    fn try_from_left() {
        assert_eq!(
            HorizontalAlign::Left,
            HorizontalAlign::try_from(build_cell_token_match("l")).unwrap()
        );
        assert_eq!(
            HorizontalAlign::Left,
            HorizontalAlign::try_from(build_cell_token_match("left")).unwrap()
        );
        assert_eq!(
            HorizontalAlign::Left,
            HorizontalAlign::try_from(build_cell_token_match("LEFT")).unwrap()
        );
    }

    #[test]
    fn try_from_invalid() {
        assert!(HorizontalAlign::try_from(build_cell_token_match("foo")).is_err());
    }
}
