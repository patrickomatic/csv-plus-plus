//! # Lexer Token Library
//!
//! A set of shared tokens that the lexer can use to break the input into a stream of tokens.
//! There are more simplistic approaches such as going character-by-character or splitting on word
//! boundaries/spacing but neither of those will work very well for us when handling complex types
//! like double-quotes strings.
use super::Token;
use crate::parser::TokenMatcher;
use std::sync;

type Matcher = TokenMatcher<Token>;

#[derive(Debug)]
pub(crate) struct TokenLibrary {
    // pub(crate) a1_reference: Matcher,
    pub(crate) close_parenthesis: Matcher,
    pub(crate) date: Matcher,
    pub(crate) identifier: Matcher,
    pub(crate) number: Matcher,
    pub(crate) single_quoted_string: Matcher,
}

// TODO: re-use the regexes with the AST one
impl TokenLibrary {
    // once this lands I can get get rid of the unwraps and return a real error
    // https://github.com/rust-lang/rust/issues/109737
    // pub(crate) fn library() -> Result<&'static Self, Error> {
    pub(crate) fn library() -> &'static Self {
        static TOKEN_LIBRARY: sync::OnceLock<TokenLibrary> = sync::OnceLock::new();

        TOKEN_LIBRARY.get_or_init(|| Self {
            // a1_reference: TokenMatcher::new(r"[$!\w:]+", Token::A1)?,
            close_parenthesis: TokenMatcher::new(r"\)", Token::CloseParenthesis).unwrap(),
            date: TokenMatcher::new(
                r"(?:(?:\d\d\d\d\-\d\d\-\d\d)|(?:\d{1,2}\/\d{1,2}\/\d{2,4}))",
                Token::Date,
            )
            .unwrap(),
            identifier: TokenMatcher::new(r"\w+", Token::Identifier).unwrap(),
            number: TokenMatcher::new(r"-?\d+(\.\d+)?", Token::Number).unwrap(),
            single_quoted_string: TokenMatcher::new(
                r"'(?:[^'\\]|\\(?:['\\/bfnrt]|u[0-9a-fA-F]{4}))*'",
                Token::String,
            )
            .unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_library() -> &'static TokenLibrary {
        TokenLibrary::library()
    }

    #[test]
    fn build_date() {
        assert!(token_library().date.1.is_match("11/12/2024"));
        assert!(token_library().date.1.is_match("1/2/24"));
        assert!(token_library().date.1.is_match("11/2/24"));

        assert!(token_library().date.1.is_match("2024-11-12"));

        assert!(!token_library().date.1.is_match("1/2"));
        assert!(!token_library().date.1.is_match("123"));
    }

    #[test]
    fn build_identifier() {
        assert!(token_library().identifier.1.is_match("foo"));

        assert!(!token_library().identifier.1.is_match("<foo>"));
    }

    #[test]
    fn build_number() {
        assert!(token_library().number.1.is_match("123"));
        assert!(token_library().number.1.is_match("-123"));
        assert!(token_library().number.1.is_match("-123.0123"));

        assert!(!token_library().number.1.is_match("abc"));
    }

    #[test]
    fn build_single_quoted_string() {
        assert!(token_library().single_quoted_string.1.is_match("'foo'"));

        assert!(!token_library().single_quoted_string.1.is_match("foo"));
    }
}
