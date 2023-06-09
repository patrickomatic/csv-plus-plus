//! # Lexer Token Library 
//!
//! A set of shared tokens that the lexer can use to break the input into a stream of tokens.
//! There are more simplistic approaches such as going character-by-character or splitting on word
//! boundaries/spacing but neither of those will work very well for us when handling complex types
//! like double-quotes strings.
use regex::Regex;
use crate::error::Error;

pub struct TokenLibrary {
    pub boolean_true: Regex,
    pub boolean_false: Regex,
}

fn re_compile(regex_str: &str) -> Result<Regex, Error> {
    // TODO: wrap with \b?
    match Regex::new(regex_str) {
        Ok(r) => Ok(r),
        Err(m) => 
            Err(Error::InitError(format!("Error compiling regex /{}/: {}", regex_str, m))),
    }
}

impl TokenLibrary {
    pub fn build() -> Result<Self, Error> {
        Ok(Self {
            boolean_true: re_compile(r"true")?,
            boolean_false: re_compile(r"false")?,
        })
    }
}
