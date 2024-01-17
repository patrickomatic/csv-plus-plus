//! # Lexer Token Library
//!
//! A set of shared tokens that the lexer can use to break the input into a stream of tokens.
//! There are more simplistic approaches such as going character-by-character or splitting on word
//! boundaries/spacing but neither of those will work very well for us when handling complex types
//! like double-quotes strings.
use super::{Token, TokenMatcher};
use std::sync;

pub(crate) const CODE_SECTION_SEPARATOR: &str = "---";

type Matcher = TokenMatcher<Token>;

#[derive(Debug)]
pub(crate) struct TokenLibrary {
    pub(crate) boolean_true: Matcher,
    pub(crate) boolean_false: Matcher,
    pub(crate) code_section_eof: Matcher,
    pub(crate) comma: Matcher,
    pub(crate) comment: Matcher,
    pub(crate) close_paren: Matcher,
    pub(crate) double_quoted_string: Matcher,
    pub(crate) infix_operator: Matcher,
    pub(crate) integer: Matcher,
    pub(crate) float: Matcher,
    pub(crate) fn_def: Matcher,
    pub(crate) newline: Matcher,
    pub(crate) open_paren: Matcher,
    pub(crate) reference: Matcher,
    pub(crate) use_module: Matcher,
    pub(crate) var_assign: Matcher,
}

impl TokenLibrary {
    // once this lands I can get get rid of the unwraps and return a real error
    // https://github.com/rust-lang/rust/issues/109737
    // pub(crate) fn library() -> Result<&'static Self, Error> {
    pub(crate) fn library() -> &'static Self {
        static TOKEN_LIBRARY: sync::OnceLock<TokenLibrary> = sync::OnceLock::new();

        TOKEN_LIBRARY.get_or_init(|| Self {
            boolean_true: TokenMatcher::new(r"true", Token::Boolean),
            boolean_false: TokenMatcher::new(r"false", Token::Boolean),
            comma: TokenMatcher::new(r",", Token::Comma),
            comment: TokenMatcher::new(r"(?m)#.*", Token::Comment),
            code_section_eof: TokenMatcher::new(r"---", Token::CodeSectionEof),
            close_paren: TokenMatcher::new(r"\)", Token::CloseParen),
            // TODO: change to single_quoted_string
            double_quoted_string: TokenMatcher::new(
                r#""(?:[^"\\]|\\(?:["\\/bfnrt]|u[0-9a-fA-F]{4}))*""#,
                Token::DoubleQuotedString,
            ),
            infix_operator: TokenMatcher::new(
                r"(\^|\+|-|\*|/|&|=|<|>|<=|>=|<>)",
                Token::InfixOperator,
            ),
            integer: TokenMatcher::new(r"-?\d+", Token::Integer),
            float: TokenMatcher::new(r"-?\d+\.\d*", Token::Float),
            fn_def: TokenMatcher::new(r"fn\s+", Token::FunctionDefinition),
            newline: TokenMatcher::new(r"\n", Token::Newline),
            open_paren: TokenMatcher::new(r"\(", Token::OpenParen),
            reference: TokenMatcher::new(r"[$!\w:]+[$!\w:.]?", Token::Reference),
            use_module: TokenMatcher::new(r"use", Token::UseModule),
            var_assign: TokenMatcher::new(r":=", Token::VarAssign),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::test_utils::*;

    fn token_library() -> &'static TokenLibrary {
        TokenLibrary::library()
    }

    #[test]
    fn library_boolean() {
        assert!(token_library().boolean_true.1.is_match("true"));
        assert!(token_library().boolean_false.1.is_match("false"));

        assert!(!token_library().boolean_true.1.is_match("false"));
        assert!(!token_library().boolean_false.1.is_match("true"));
    }

    #[test]
    fn library_code_section_eof() {
        assert!(token_library().code_section_eof.1.is_match("---"));
    }

    #[test]
    fn library_comment() {
        assert!(token_library().comment.1.is_match("# this is a comment"));
    }

    #[test]
    fn library_double_quoted_string() {
        assert!(token_library()
            .double_quoted_string
            .1
            .is_match("\"this is a string\""));
        assert!(token_library()
            .double_quoted_string
            .1
            .is_match("\"with an \\\" escaped quote\""));

        assert!(!token_library()
            .double_quoted_string
            .1
            .is_match("\"missing end quote"));
        assert!(!token_library().double_quoted_string.1.is_match("foo"));
    }

    #[test]
    fn library_integer() {
        assert!(token_library().integer.1.is_match("555"));
        assert!(token_library().integer.1.is_match("-555"));

        assert!(!token_library().integer.1.is_match("foo"));
    }

    #[test]
    fn library_float() {
        assert!(token_library().float.1.is_match("555.55"));
        assert!(token_library().float.1.is_match("-555.55"));

        assert!(!token_library().float.1.is_match("555"));
    }

    #[test]
    fn library_reference() {
        assert!(token_library().reference.1.is_match("foo"));
        assert!(token_library().reference.1.is_match("A1:B2"));
        assert!(token_library().reference.1.is_match("Foo!A1:B2"));

        assert!(!token_library().reference.1.is_match("*"));

        // it can contain a `.` but it can't start with one
        assert!(token_library().reference.1.is_match("foo.bar"));
        assert!(!token_library().reference.1.is_match(".foo"));
    }

    #[test]
    fn display_tokenmatch() {
        let token_match = TokenMatch {
            token: Token::Comma,
            line_number: 22,
            line_offset: 3,
            position: None,
            source_code: build_source_code(),
            str_match: ",",
        };

        assert_eq!("`,`", token_match.to_string());
    }
}
