use super::{Token, TokenLibrary};
use crate::Error;
use regex::Regex;

#[derive(Clone, Debug)]
pub(crate) struct TokenMatcher(pub(crate) Token, pub(crate) Regex);

impl TokenMatcher {
    pub(crate) fn new(regex_str: &str, token: Token) -> Result<Self, Error> {
        Ok(TokenMatcher(
            token,
            // this regex is kinda tricky but it's "all spaces but not newlines"
            Regex::new(format!(r"^[^\S\r\n]*{regex_str}").as_str()).map_err(|m| {
                Error::InitError(format!("Error compiling regex /{regex_str}/: {m}"))
            })?,
        ))
    }
}

impl TokenMatcher {
    /// For better or worse the tokens are not mutually exclusive - some of them are subsets of another
    /// (for example 555.55 could be matched by both float and integer (integer can just match the first
    /// part of it)) so it's important float is first. Another example is comments - they have to be
    /// stripped out first
    pub(crate) fn matchers_ordered(tl: &TokenLibrary) -> [&TokenMatcher; 16] {
        [
            &tl.newline,
            &tl.comment,
            &tl.double_quoted_string,
            &tl.fn_def,
            &tl.var_assign,
            &tl.comma,
            &tl.close_paren,
            &tl.open_paren,
            &tl.infix_operator,
            &tl.code_section_eof,
            &tl.date_time,
            // float has to be happen before integer!  it needs to greedy match 1.5, where integer will
            // also match the first part 1, but not the rest
            &tl.float,
            &tl.integer,
            &tl.boolean_true,
            &tl.boolean_false,
            &tl.reference,
        ]
    }
}
