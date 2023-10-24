//! # Lexer Token Library
//!
//! A set of shared tokens that the lexer can use to break the input into a stream of tokens.
//! There are more simplistic approaches such as going character-by-character or splitting on word
//! boundaries/spacing but neither of those will work very well for us when handling complex types
//! like double-quotes strings.
use super::{Token, TokenMatcher};
use crate::Error;

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
    pub(crate) date_time: Matcher,
    pub(crate) double_quoted_string: Matcher,
    pub(crate) infix_operator: Matcher,
    pub(crate) integer: Matcher,
    pub(crate) float: Matcher,
    pub(crate) fn_def: Matcher,
    pub(crate) newline: Matcher,
    pub(crate) open_paren: Matcher,
    pub(crate) reference: Matcher,
    pub(crate) var_assign: Matcher,
}

impl TokenLibrary {
    pub(crate) fn build() -> Result<Self, Error> {
        Ok(Self {
            boolean_true: TokenMatcher::new(r"true", Token::Boolean)?,
            boolean_false: TokenMatcher::new(r"false", Token::Boolean)?,
            comma: TokenMatcher::new(r",", Token::Comma)?,
            comment: TokenMatcher::new(r"(?m)#.*", Token::Comment)?,
            code_section_eof: TokenMatcher::new(r"---", Token::CodeSectionEof)?,
            close_paren: TokenMatcher::new(r"\)", Token::CloseParen)?,
            date_time: TokenMatcher::new(
                r"(?x)
                 # just a date (and optional TZ)
                 (?<date0>\d{2,4}-\d{1,2}-\d{1,2})\s*(?<tz0>\w+)?
                 | 
                 # just a time (and an optional TZ)
                 (?<time1>\d+:\d{1,2}(\d+)?)\s*(?<tz1>\w+)?
                 |
                 # a time and date
                 (?<date2>\d{2,4}-\d{1,2}-\d{1,2})\s+(?<time2>\d+:\d{1,2}(\d+)?)\s*(?<tz2>\w+)?
                ",
                Token::DateTime,
            )?,
            double_quoted_string: TokenMatcher::new(
                r#""(?:[^"\\]|\\(?:["\\/bfnrt]|u[0-9a-fA-F]{4}))*""#,
                Token::DoubleQuotedString,
            )?,
            infix_operator: TokenMatcher::new(
                r"(\^|\+|-|\*|/|&|<|>|<=|>=|<>)",
                Token::InfixOperator,
            )?,
            integer: TokenMatcher::new(r"-?\d+", Token::Integer)?,
            float: TokenMatcher::new(r"-?\d+\.\d*", Token::Float)?,
            fn_def: TokenMatcher::new(r"fn", Token::FunctionDefinition)?,
            newline: TokenMatcher::new(r"\n", Token::Newline)?,
            open_paren: TokenMatcher::new(r"\(", Token::OpenParen)?,
            reference: TokenMatcher::new(r"[$!\w:]+", Token::Reference)?,
            var_assign: TokenMatcher::new(r":=", Token::VarAssign)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::test_utils::*;

    fn token_library() -> TokenLibrary {
        TokenLibrary::build().unwrap()
    }

    #[test]
    fn build_boolean() {
        assert!(token_library().boolean_true.1.is_match("true"));
        assert!(token_library().boolean_false.1.is_match("false"));

        assert!(!token_library().boolean_true.1.is_match("false"));
        assert!(!token_library().boolean_false.1.is_match("true"));
    }

    #[test]
    fn build_code_section_eof() {
        assert!(token_library().code_section_eof.1.is_match("---"));
    }

    #[test]
    fn build_comment() {
        assert!(token_library().comment.1.is_match("# this is a comment"));
    }

    #[test]
    fn build_date_time() {
        assert!(token_library().date_time.1.is_match("2022-01-12"));
        assert!(token_library().date_time.1.is_match("2022-01-12 EST"));
        assert!(token_library().date_time.1.is_match("2022-01-12 11:00"));
        assert!(token_library().date_time.1.is_match("2022-01-12 11:00 EST"));
    }

    #[test]
    fn build_double_quoted_string() {
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
    fn build_integer() {
        assert!(token_library().integer.1.is_match("555"));
        assert!(token_library().integer.1.is_match("-555"));

        assert!(!token_library().integer.1.is_match("foo"));
    }

    #[test]
    fn build_float() {
        assert!(token_library().float.1.is_match("555.55"));
        assert!(token_library().float.1.is_match("-555.55"));

        assert!(!token_library().float.1.is_match("555"));
    }

    #[test]
    fn build_reference() {
        assert!(token_library().reference.1.is_match("foo"));
        assert!(token_library().reference.1.is_match("A1:B2"));
        assert!(token_library().reference.1.is_match("Foo!A1:B2"));

        assert!(!token_library().reference.1.is_match("*"));
    }

    #[test]
    fn display_tokenmatch() {
        let token_match = TokenMatch {
            token: Token::Comma,
            str_match: ",",
            line_number: 22,
            line_offset: 3,
            source_code: &build_source_code(),
        };

        assert_eq!("`,`", token_match.to_string());
    }
}
