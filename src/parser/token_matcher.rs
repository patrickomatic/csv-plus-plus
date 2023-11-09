use crate::Error;
use regex::Regex;

#[derive(Debug)]
pub(crate) struct TokenMatcher<T>(pub(crate) T, pub(crate) Regex);

#[derive(Debug, PartialEq)]
pub(super) struct StrMatch<'a> {
    pub(super) len_leading_whitespace: usize,
    pub(super) len_full_match: usize,
    pub(super) str_match: &'a str,
}

fn whitespace_start(input: &str) -> usize {
    input
        .chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n' && *ch != '\r')
        .count()
}

impl<T> TokenMatcher<T> {
    pub(crate) fn new(regex_str: &str, token: T) -> Result<Self, Error> {
        Ok(TokenMatcher(
            token,
            // this regex is kinda tricky but it's "all spaces but not newlines"
            Regex::new(format!(r"^[^\S\r\n]*{regex_str}").as_str()).map_err(|m| {
                Error::InitError(format!("Error compiling regex /{regex_str}/: {m}"))
            })?,
        ))
    }

    pub(super) fn try_match<'a>(&self, input: &'a str) -> Option<StrMatch<'a>> {
        if let Some(m) = self.1.find(input) {
            let str_match = m.as_str();
            Some(StrMatch {
                len_leading_whitespace: whitespace_start(str_match),
                len_full_match: str_match.len(),
                str_match: str_match.trim(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::cell_lexer::Token;

    #[test]
    fn try_match_some() {
        let token_matcher = TokenMatcher::new(r"\w+", Token::Identifier).unwrap();
        assert_eq!(
            token_matcher.try_match("  foo"),
            Some(StrMatch {
                len_leading_whitespace: 2,
                len_full_match: 5,
                str_match: "foo",
            })
        );
    }

    #[test]
    fn try_match_none() {
        let token_matcher = TokenMatcher::new(r"\w+", Token::Identifier).unwrap();
        assert_eq!(token_matcher.try_match("  !!!"), None);
    }
}
