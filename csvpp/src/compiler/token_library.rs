//! # Lexer Token Library 
//!
//! A set of shared tokens that the lexer can use to break the input into a stream of tokens.
//! There are more simplistic approaches such as going character-by-character or splitting on word
//! boundaries/spacing but neither of those will work very well for us when handling complex types
//! like double-quotes strings.
use regex::Regex;
// use std::fmt;

use crate::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Boolean,
    CloseParen,
    Comma,
    Comment,
    CodeSectionEof,
    // TODO
    // DateTime,
    DoubleQuotedString,
    Eof,
    InfixOperator,
    Integer,
    Float,
    FunctionDefinition,
    OpenParen,
    Reference,
    VarAssign,
}

#[derive(Clone, Debug)]
pub struct TokenMatcher(pub Token, pub Regex);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TokenMatch<'a>(pub Token, pub &'a str);

impl TokenMatcher {
    fn new(regex_str: &str, token: Token) -> Result<TokenMatcher, Error> {
        match Regex::new(format!(r"^\s*{}", regex_str).as_str()) {
            Ok(r) =>
                Ok(TokenMatcher(token, r)),
            Err(m) =>
                Err(Error::InitError(format!("Error compiling regex /{}/: {}", regex_str, m))),
        }
    }
}

#[derive(Debug)]
pub struct TokenLibrary {
    pub boolean_true: TokenMatcher,
    pub boolean_false: TokenMatcher,
    pub code_section_eof: TokenMatcher,
    pub comma: TokenMatcher,
    pub comment: TokenMatcher,
    pub close_paren: TokenMatcher,
    // TODO
    // pub date_time: TokenMatcher,
    pub double_quoted_string: TokenMatcher,
    pub infix_operator: TokenMatcher,
    pub integer: TokenMatcher,
    pub float: TokenMatcher,
    pub fn_def: TokenMatcher,
    pub open_paren: TokenMatcher,
    pub reference: TokenMatcher,
    pub var_assign: TokenMatcher,
}

impl TokenLibrary {
    pub fn build() -> Result<Self, Error> {
        Ok(Self {
            boolean_true: TokenMatcher::new(r"true", Token::Boolean)?,
            boolean_false: TokenMatcher::new(r"false", Token::Boolean)?,
            comma: TokenMatcher::new(r",", Token::Comma)?,
            comment: TokenMatcher::new(r"(?m)#.*", Token::Comment)?,
            code_section_eof: TokenMatcher::new(r"---", Token::CodeSectionEof)?,
            close_paren: TokenMatcher::new(r"\)", Token::CloseParen)?,
            double_quoted_string: 
                TokenMatcher::new(r#""(?:[^"\\]|\\(?:["\\/bfnrt]|u[0-9a-fA-F]{4}))*""#, Token::DoubleQuotedString)?,
            infix_operator: TokenMatcher::new(r"(\^|\+|-|\*|/|&|<|>|<=|>=|<>)", Token::InfixOperator)?,
            integer: TokenMatcher::new(r"-?\d+", Token::Integer)?,
            float: TokenMatcher::new(r"-?\d+\.\d*", Token::Float)?,
            fn_def: TokenMatcher::new(r"fn", Token::FunctionDefinition)?,
            open_paren: TokenMatcher::new(r"\(", Token::OpenParen)?,
            reference: TokenMatcher::new(r"[$!\w:]+", Token::Reference)?,
            var_assign: TokenMatcher::new(r":=", Token::VarAssign)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_library() -> TokenLibrary {
        TokenLibrary::build().unwrap()
    }

    #[test]
    fn boolean() {
        assert!(token_library().boolean_true.1.is_match("true"));
        assert!(token_library().boolean_false.1.is_match("false"));

        assert!(!token_library().boolean_true.1.is_match("false"));
        assert!(!token_library().boolean_false.1.is_match("true"));
    }

    #[test]
    fn code_section_eof() {
        assert!(token_library().code_section_eof.1.is_match("---"));
    }

    #[test]
    fn comment() {
        assert!(token_library().comment.1.is_match("# this is a comment"));
    }

    #[test]
    fn double_quoted_string() {
        assert!(token_library().double_quoted_string.1.is_match("\"this is a string\""));
        assert!(token_library().double_quoted_string.1.is_match("\"with an \\\" escaped quote\""));

        assert!(!token_library().double_quoted_string.1.is_match("\"missing end quote"));
        assert!(!token_library().double_quoted_string.1.is_match("foo"));
    }

    #[test]
    fn integer() {
        assert!(token_library().integer.1.is_match("555"));
        assert!(token_library().integer.1.is_match("-555"));

        assert!(!token_library().integer.1.is_match("foo"));
    }

    #[test]
    fn float() {
        assert!(token_library().float.1.is_match("555.55"));
        assert!(token_library().float.1.is_match("-555.55"));

        assert!(!token_library().float.1.is_match("555"));
    }

    #[test]
    fn reference() {
        assert!(token_library().reference.1.is_match("foo"));
        assert!(token_library().reference.1.is_match("A1:B2"));
        assert!(token_library().reference.1.is_match("Foo!A1:B2"));

        assert!(!token_library().reference.1.is_match("*"));
    }
}
