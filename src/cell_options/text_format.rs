//! # `TextFormat`
use crate::error::CellParseError;
use crate::parser::cell_lexer::TokenMatch;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TextFormat {
    Bold,
    Italic,
    Strikethrough,
    Underline,
}

impl TryFrom<TokenMatch> for TextFormat {
    type Error = CellParseError;

    fn try_from(input: TokenMatch) -> Result<Self, Self::Error> {
        match input.str_match.to_lowercase().as_str() {
            "b" | "bold" => Ok(Self::Bold),
            "i" | "italic" => Ok(Self::Italic),
            "s" | "strikethrough" => Ok(Self::Strikethrough),
            "u" | "underline" => Ok(Self::Underline),
            _ => Err(CellParseError::new(
                "text",
                input,
                &[
                    "bold (b)",
                    "italic (i)",
                    "strikethrough (s)",
                    "underline (u)",
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
    fn try_from_bold() {
        assert_eq!(
            TextFormat::Bold,
            TextFormat::try_from(build_cell_token_match("b")).unwrap()
        );
        assert_eq!(
            TextFormat::Bold,
            TextFormat::try_from(build_cell_token_match("bold")).unwrap()
        );
        assert_eq!(
            TextFormat::Bold,
            TextFormat::try_from(build_cell_token_match("BOLD")).unwrap()
        );
    }

    #[test]
    fn try_from_italic() {
        assert_eq!(
            TextFormat::Italic,
            TextFormat::try_from(build_cell_token_match("i")).unwrap()
        );
        assert_eq!(
            TextFormat::Italic,
            TextFormat::try_from(build_cell_token_match("italic")).unwrap()
        );
        assert_eq!(
            TextFormat::Italic,
            TextFormat::try_from(build_cell_token_match("ITALIC")).unwrap()
        );
    }

    #[test]
    fn try_from_underline() {
        assert_eq!(
            TextFormat::Underline,
            TextFormat::try_from(build_cell_token_match("u")).unwrap()
        );
        assert_eq!(
            TextFormat::Underline,
            TextFormat::try_from(build_cell_token_match("underline")).unwrap()
        );
        assert_eq!(
            TextFormat::Underline,
            TextFormat::try_from(build_cell_token_match("UNDERLINE")).unwrap()
        );
    }

    #[test]
    fn try_from_strikethrough() {
        assert_eq!(
            TextFormat::Strikethrough,
            TextFormat::try_from(build_cell_token_match("s")).unwrap()
        );
        assert_eq!(
            TextFormat::Strikethrough,
            TextFormat::try_from(build_cell_token_match("strikethrough")).unwrap()
        );
        assert_eq!(
            TextFormat::Strikethrough,
            TextFormat::try_from(build_cell_token_match("STRIKETHROUGH")).unwrap()
        );
    }
}
