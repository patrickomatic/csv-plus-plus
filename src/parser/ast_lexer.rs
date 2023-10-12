//! # AstLexer
//!
use super::token_library::{Token, TokenLibrary, TokenMatch, TokenMatcher};
use crate::error::{BadInput, ParseResult};
use crate::{CharOffset, LineNumber, Runtime};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub struct AstLexer<'a> {
    tokens: Rc<RefCell<Vec<TokenMatch<'a>>>>,
    lines: LineNumber,
    eof_position: CharOffset,
}

/// For better or worse the tokens are not mutually exclusive - some of them are subsets of another
/// (for example 555.55 could be matched by both float and integer (integer can just match the first
/// part of it) so it's important float is first. Another example is comments - they have to be
/// stripped out first
fn matchers_ordered(tl: &TokenLibrary) -> [&TokenMatcher; 15] {
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
        // float has to be happen before integer!  it needs to greedy match 1.5, where integer will
        // also match the first part 1, but not the rest
        &tl.float,
        &tl.integer,
        &tl.boolean_true,
        &tl.boolean_false,
        &tl.reference,
    ]
}

fn whitespace_start(input: &str) -> usize {
    input
        .chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n' && *ch != '\r')
        .count()
}

#[derive(Debug)]
pub(crate) struct UnknownToken {
    pub(crate) bad_input: String,
    pub(crate) line_number: LineNumber,
    pub(crate) line_offset: CharOffset,
}

impl fmt::Display for UnknownToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut shortened_bad_input = self.bad_input.clone();
        shortened_bad_input.truncate(50);
        write!(f, "{shortened_bad_input}")
    }
}

impl BadInput for UnknownToken {
    fn line_number(&self) -> LineNumber {
        self.line_number
    }

    fn line_offset(&self) -> CharOffset {
        self.line_offset
    }
}

impl<'a> AstLexer<'a> {
    pub(crate) fn new(input: &'a str, runtime: &'a Runtime) -> ParseResult<AstLexer<'a>> {
        let mut line_number = 0;
        let mut position = 0;

        let mut tokens: Vec<TokenMatch> = vec![];
        let mut p = input;

        loop {
            let mut matched = false;

            for TokenMatcher(token, regex) in matchers_ordered(&runtime.token_library).iter() {
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
                return Err(runtime.source_code.parse_error(
                    UnknownToken {
                        bad_input: p.to_string(),
                        line_number,
                        line_offset: position,
                    },
                    "Error parsing input - invalid token",
                ));
            }
        }

        tokens.reverse();

        Ok(AstLexer {
            tokens: Rc::new(RefCell::new(tokens)),
            eof_position: position,
            lines: line_number,
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn build_runtime() -> Runtime {
        TestFile::new("csv", "foo,bar").into()
    }

    fn build_token_match(
        token: Token,
        str_match: &str,
        line_number: LineNumber,
        line_offset: CharOffset,
    ) -> TokenMatch {
        TokenMatch {
            token,
            str_match,
            line_number,
            line_offset,
        }
    }

    #[test]
    fn lexer_new() {
        let runtime = build_runtime();
        let lexer = AstLexer::new("foo bar,\"a\",123 \n(d, b) + * 0.25", &runtime).unwrap();

        assert_eq!(
            lexer.next(),
            build_token_match(Token::Reference, "foo", 0, 0)
        );
        assert_eq!(
            lexer.next(),
            build_token_match(Token::Reference, "bar", 0, 4)
        );
        assert_eq!(lexer.next(), build_token_match(Token::Comma, ",", 0, 7));
        assert_eq!(
            lexer.next(),
            build_token_match(Token::DoubleQuotedString, "\"a\"", 0, 8)
        );
        assert_eq!(lexer.next(), build_token_match(Token::Comma, ",", 0, 11));
        assert_eq!(
            lexer.next(),
            build_token_match(Token::Integer, "123", 0, 12)
        );
        assert_eq!(lexer.next(), build_token_match(Token::OpenParen, "(", 1, 0));
        assert_eq!(lexer.next(), build_token_match(Token::Reference, "d", 1, 1));
        assert_eq!(lexer.next(), build_token_match(Token::Comma, ",", 1, 2));
        assert_eq!(lexer.next(), build_token_match(Token::Reference, "b", 1, 4));
        assert_eq!(
            lexer.next(),
            build_token_match(Token::CloseParen, ")", 1, 5)
        );
        assert_eq!(
            lexer.next(),
            build_token_match(Token::InfixOperator, "+", 1, 7)
        );
        assert_eq!(
            lexer.next(),
            build_token_match(Token::InfixOperator, "*", 1, 9)
        );
        assert_eq!(lexer.next(), build_token_match(Token::Float, "0.25", 1, 11));
        assert_eq!(lexer.next(), build_token_match(Token::Eof, "", 1, 15));
        assert_eq!(lexer.next(), build_token_match(Token::Eof, "", 1, 15));
    }

    #[test]
    fn lexer_new_comment() {
        let runtime = build_runtime();
        let lexer = AstLexer::new("# this is a comment\na_ref\n", &runtime).unwrap();

        assert_eq!(
            lexer.next(),
            build_token_match(Token::Reference, "a_ref", 1, 0)
        );
        assert_eq!(lexer.next(), build_token_match(Token::Eof, "", 1, 5));
    }

    #[test]
    fn lexer_new_newlines() {
        let runtime = build_runtime();
        let lexer = AstLexer::new("\n foo \n bar", &runtime).unwrap();

        assert_eq!(
            lexer.next(),
            build_token_match(Token::Reference, "foo", 1, 1)
        );
        assert_eq!(
            lexer.next(),
            build_token_match(Token::Reference, "bar", 2, 1)
        );
    }

    #[test]
    fn lexer_peek() {
        let runtime = build_runtime();
        let lexer = AstLexer::new("foo (bar) + baz", &runtime).unwrap();

        assert_eq!(
            lexer.peek(),
            build_token_match(Token::Reference, "foo", 0, 0)
        );
        assert_eq!(
            lexer.peek(),
            build_token_match(Token::Reference, "foo", 0, 0)
        );
        assert_eq!(
            lexer.peek(),
            build_token_match(Token::Reference, "foo", 0, 0)
        );
    }
}
