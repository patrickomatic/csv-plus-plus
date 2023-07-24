//! # AstLexer
//!
use std::cell::RefCell;
use std::rc::Rc;

use crate::{Error, Result, TokenLibrary};
use super::token_library::{Token, TokenMatch, TokenMatcher};

pub struct AstLexer<'a> {
    tokens: Rc<RefCell<Vec<TokenMatch<'a>>>>,
    lines: usize,
    eof_position: usize,
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
        &tl.integer,
        &tl.float,
        &tl.boolean_true,
        &tl.boolean_false,
        &tl.reference,
    ]
}

fn whitespace_start(input: &str) -> usize {
    input.chars().take_while(|ch| ch.is_whitespace() && *ch != '\n').count()
}

impl<'a> AstLexer<'a> {
    pub fn new(
        input: &'a str,
        token_library: &'a TokenLibrary
    ) -> Result<AstLexer<'a>> {
        let mut line_number = 1;
        let mut position = 1;

        let mut tokens: Vec<TokenMatch> = vec![];
        let mut p = input;

        loop {
            let mut matched = false;

            for TokenMatcher(token, regex) in matchers_ordered(token_library).iter() {
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
                            position: position + whitespace_start(str_match),
                        });
                    }

                    if *token == Token::Newline {
                        // move position back to the beginning
                        position = 1;
                    } else {
                        position += m.len();
                    }

                    // move the input past the match
                    p = &p[m.len()..];
                    matched = true;

                    break;
                }
            }

            dbg!(&p);
            if p.trim().is_empty() {
                break;
            }

            if !matched {
                // we did a round of all the tokens and didn't match any of them - invalid syntax
                return Err(Error::CodeSyntaxError {
                    message: format!("Error parsing input - invalid token: {}", p),
                    line_number,
                    position,
                })
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
    pub fn next(&self) -> TokenMatch {
        self.tokens.borrow_mut().pop().unwrap_or_else(|| self.eof())
    }

    /// Return but do not consume the next `TokenMatch`
    pub fn peek(&self) -> TokenMatch {
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
            position: self.eof_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_token_match(token: Token, str_match: &str, line_number: usize, position: usize) -> TokenMatch {
        TokenMatch { token, str_match, line_number, position }
    }

    fn build_token_library() -> TokenLibrary {
        TokenLibrary::build().unwrap()
    }

    #[test]
    fn lexer_new() {
        let tl = build_token_library();
        let lexer = AstLexer::new("foo bar,\"a\",123 \n(d, b) + *", &tl).unwrap();

        assert_eq!(lexer.next(), build_token_match(Token::Reference, "foo", 1, 1));
        assert_eq!(lexer.next(), build_token_match(Token::Reference, "bar", 1, 5));
        assert_eq!(lexer.next(), build_token_match(Token::Comma, ",", 1, 8));
        assert_eq!(lexer.next(), build_token_match(Token::DoubleQuotedString,"\"a\"", 1, 9));
        assert_eq!(lexer.next(), build_token_match(Token::Comma, ",", 1, 12));
        assert_eq!(lexer.next(), build_token_match(Token::Integer, "123", 1, 13));
        assert_eq!(lexer.next(), build_token_match(Token::OpenParen, "(", 2, 1));
        assert_eq!(lexer.next(), build_token_match(Token::Reference, "d", 2, 2));
        assert_eq!(lexer.next(), build_token_match(Token::Comma, ",", 2, 3));
        assert_eq!(lexer.next(), build_token_match(Token::Reference, "b", 2, 5));
        assert_eq!(lexer.next(), build_token_match(Token::CloseParen, ")", 2, 6));
        assert_eq!(lexer.next(), build_token_match(Token::InfixOperator, "+", 2, 8));
        assert_eq!(lexer.next(), build_token_match(Token::InfixOperator, "*", 2, 10));
        assert_eq!(lexer.next(), build_token_match(Token::Eof, "", 2, 11));
        assert_eq!(lexer.next(), build_token_match(Token::Eof, "", 2, 11));
    }

    #[test]
    fn lexer_new_comment() {
        let tl = build_token_library();
        let lexer = AstLexer::new("# this is a comment\na_ref\n", &tl).unwrap();

        assert_eq!(lexer.next(), build_token_match(Token::Reference, "a_ref", 2, 1));
        assert_eq!(lexer.next(), build_token_match(Token::Eof, "", 2, 6));
    }

    #[test]
    fn lexer_new_newlines() {
        let tl = build_token_library();
        let lexer = AstLexer::new("\n foo \n bar", &tl).unwrap();

        assert_eq!(lexer.next(), build_token_match(Token::Reference, "foo", 2, 2));
        assert_eq!(lexer.next(), build_token_match(Token::Reference, "bar", 3, 2));
    }

    #[test]
    fn lexer_peek() {
        let tl = build_token_library();
        let lexer = AstLexer::new("foo (bar) + baz", &tl).unwrap();

        assert_eq!(lexer.peek(), build_token_match(Token::Reference, "foo", 1, 1));
        assert_eq!(lexer.peek(), build_token_match(Token::Reference, "foo", 1, 1));
        assert_eq!(lexer.peek(), build_token_match(Token::Reference, "foo", 1, 1));
    }
}
