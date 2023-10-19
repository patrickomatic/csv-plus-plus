//! # AstLexer
//!
use crate::error::ParseResult;
use crate::{CharOffset, LineNumber, Runtime, SourceCode};
use std::cell::RefCell;
use std::rc::Rc;

mod token;
mod token_library;
mod token_match;
mod token_matcher;
mod unknown_token;

pub(crate) use token::Token;
pub(crate) use token_library::{TokenLibrary, CODE_SECTION_SEPARATOR};
pub(crate) use token_match::TokenMatch;
pub(crate) use token_matcher::TokenMatcher;
pub(crate) use unknown_token::UnknownToken;

pub(crate) struct AstLexer<'a> {
    tokens: Rc<RefCell<Vec<TokenMatch<'a>>>>,
    lines: LineNumber,
    eof_position: CharOffset,
    source_code: &'a SourceCode,
}

fn whitespace_start(input: &str) -> usize {
    input
        .chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n' && *ch != '\r')
        .count()
}

impl<'a> AstLexer<'a> {
    pub(crate) fn new(input: &'a str, runtime: &'a Runtime) -> ParseResult<AstLexer<'a>> {
        let mut line_number = 0;
        let mut position = 0;

        let mut tokens: Vec<TokenMatch> = vec![];
        let mut p = input;

        loop {
            let mut matched = false;

            for TokenMatcher(token, regex) in
                TokenMatcher::matchers_ordered(&runtime.token_library).iter()
            {
                if let Some(m) = regex.find(p) {
                    if *token == Token::Newline {
                        // just count the newline but don't store it on `tokens`
                        line_number += 1;
                    } else if *token != Token::Comment {
                        let str_match = m.as_str();
                        // we'll want to consume everything except for comments and newlines (no point
                        // in the parsing logic needing to consider them)
                        tokens.push(TokenMatch {
                            token: *token,
                            str_match: str_match.trim(),
                            line_number,
                            line_offset: position + whitespace_start(str_match),
                            source_code: &runtime.source_code,
                        });
                    }

                    if *token == Token::Newline {
                        // move position back to the beginning
                        position = 0;
                    } else {
                        position += m.len();
                    }

                    // move the input past the match
                    p = &p[m.len()..];
                    matched = true;

                    break;
                }
            }

            if p.trim().is_empty() {
                break;
            }

            if !matched {
                // we did a round of all the tokens and didn't match any of them - invalid syntax
                return Err(UnknownToken {
                    bad_input: p.to_string(),
                    line_number,
                    line_offset: position,
                    source_code: &runtime.source_code,
                }
                .into());
            }
        }

        tokens.reverse();

        Ok(AstLexer {
            tokens: Rc::new(RefCell::new(tokens)),
            eof_position: position,
            lines: line_number,
            source_code: &runtime.source_code,
        })
    }

    /// Consume and return the next `TokenMatch`
    pub(super) fn next(&self) -> TokenMatch {
        self.tokens.borrow_mut().pop().unwrap_or_else(|| self.eof())
    }

    /// Return but do not consume the next `TokenMatch`
    pub(super) fn peek(&self) -> TokenMatch {
        match self.tokens.borrow().last() {
            Some(t) => *t,
            None => self.eof(),
        }
    }

    fn eof(&self) -> TokenMatch {
        TokenMatch {
            token: Token::Eof,
            str_match: "",
            line_number: self.lines,
            line_offset: self.eof_position,
            source_code: self.source_code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    macro_rules! assert_token_match_eq {
        ($lexer:ident, $token:path, $str_match:expr, $line_number:expr, $line_offset:expr) => {{
            {
                let _tok = $lexer.next();
                assert_eq!(_tok.token, $token);
                assert_eq!(_tok.str_match, $str_match);
                assert_eq!(_tok.line_number, $line_number);
                assert_eq!(_tok.line_offset, $line_offset);
            }
        }};
    }

    #[test]
    fn lexer_new() {
        let runtime = build_runtime();
        let lexer = AstLexer::new("foo bar,\"a\",123 \n(d, b) + * 0.25", &runtime).unwrap();

        assert_token_match_eq!(lexer, Token::Reference, "foo", 0, 0);
        assert_token_match_eq!(lexer, Token::Reference, "bar", 0, 4);
        assert_token_match_eq!(lexer, Token::Comma, ",", 0, 7);
        assert_token_match_eq!(lexer, Token::DoubleQuotedString, "\"a\"", 0, 8);
        assert_token_match_eq!(lexer, Token::Comma, ",", 0, 11);
        assert_token_match_eq!(lexer, Token::Integer, "123", 0, 12);
        assert_token_match_eq!(lexer, Token::OpenParen, "(", 1, 0);
        assert_token_match_eq!(lexer, Token::Reference, "d", 1, 1);
        assert_token_match_eq!(lexer, Token::Comma, ",", 1, 2);
        assert_token_match_eq!(lexer, Token::Reference, "b", 1, 4);
        assert_token_match_eq!(lexer, Token::CloseParen, ")", 1, 5);
        assert_token_match_eq!(lexer, Token::InfixOperator, "+", 1, 7);
        assert_token_match_eq!(lexer, Token::InfixOperator, "*", 1, 9);
        assert_token_match_eq!(lexer, Token::Float, "0.25", 1, 11);
        assert_token_match_eq!(lexer, Token::Eof, "", 1, 15);
        assert_token_match_eq!(lexer, Token::Eof, "", 1, 15);
    }

    #[test]
    fn lexer_new_comment() {
        let runtime = build_runtime();
        let lexer = AstLexer::new("# this is a comment\na_ref\n", &runtime).unwrap();

        assert_token_match_eq!(lexer, Token::Reference, "a_ref", 1, 0);
        assert_token_match_eq!(lexer, Token::Eof, "", 1, 5);
    }

    #[test]
    fn lexer_new_newlines() {
        let runtime = build_runtime();
        let lexer = AstLexer::new("\n foo \n bar", &runtime).unwrap();

        assert_token_match_eq!(lexer, Token::Reference, "foo", 1, 1);
        assert_token_match_eq!(lexer, Token::Reference, "bar", 2, 1);
    }

    #[test]
    fn lexer_peek() {
        let runtime = build_runtime();
        let lexer = AstLexer::new("foo (bar) + baz", &runtime).unwrap();

        assert_eq!(lexer.peek().token, Token::Reference);
        assert_eq!(lexer.peek().str_match, "foo");
        assert_eq!(lexer.peek().token, Token::Reference);
        assert_eq!(lexer.peek().str_match, "foo");
        assert_eq!(lexer.peek().token, Token::Reference);
        assert_eq!(lexer.peek().str_match, "foo");
    }
}
