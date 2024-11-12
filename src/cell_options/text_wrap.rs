//! # `TextWrap`
use crate::error::CellParseError;
use crate::parser::cell_lexer::TokenMatch;

#[derive(Copy, Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TextWrap {
    #[default]
    Wrap,
    Overflow,
    Clip,
}

impl TryFrom<TokenMatch> for TextWrap {
    type Error = CellParseError;

    fn try_from(input: TokenMatch) -> Result<Self, Self::Error> {
        match input.str_match.to_lowercase().as_str() {
            "c" | "clip" => Ok(Self::Clip),
            "o" | "overflow" => Ok(Self::Overflow),
            "w" | "wrap" => Ok(Self::Wrap),
            _ => Err(CellParseError::new(
                "wrap",
                input,
                &["clip (c)", "overflow (o)", "wrap (w)"],
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from_clip() {
        assert_eq!(
            TextWrap::Clip,
            TextWrap::try_from(build_cell_token_match("c")).unwrap()
        );
        assert_eq!(
            TextWrap::Clip,
            TextWrap::try_from(build_cell_token_match("clip")).unwrap()
        );
        assert_eq!(
            TextWrap::Clip,
            TextWrap::try_from(build_cell_token_match("CLIP")).unwrap()
        );
    }

    #[test]
    fn try_from_invalid() {
        assert!(TextWrap::try_from(build_cell_token_match("foo")).is_err());
    }
}
