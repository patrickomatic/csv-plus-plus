//! # AstLexer
//!
use std::cell::RefCell;
use std::rc::Rc;

use crate::{Error, Result, TokenLibrary};
use super::token_library::{Token, TokenMatch, TokenMatcher};

pub struct AstLexer<'a> {
    tokens: Rc<RefCell<Vec<TokenMatch<'a>>>>,
}

/// For better or worse the tokens are not mutually exclusive - some of them are subsets of another 
/// (for example 555.55 could be matched by both float and integer (integer can just match the first
/// part of it) so it's important float is first. Another example is comments - they have to be
/// stripped out first
fn matchers_ordered(tl: &TokenLibrary) -> [&TokenMatcher; 14] {
    [
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

impl<'a> AstLexer<'a> {
    pub fn new(
        input: &'a str,
        token_library: &'a TokenLibrary
    ) -> Result<AstLexer<'a>> {
        let mut tokens: Vec<TokenMatch> = vec![];
        let mut p = input;

        loop {
            let mut matched = false;

            for TokenMatcher(token, regex) in matchers_ordered(token_library).iter() {
                if let Some(m) = regex.find(p) {
                    if *token != Token::Comment {
                        // we'll want to consume everything except for comments (no point in the 
                        // parsing logic needing to consider them)
                        tokens.push(TokenMatch(*token, m.as_str().trim()));
                    }

                    // move the input past the match
                    p = &p[m.end()..];
                    matched = true;

                    break;
                }
            }

            if p.trim().is_empty() {
                break;
            }

            if !matched {
                // we did a round of all the tokens and didn't match any of them - invalid syntax
                return Err(Error::CodeSyntaxError {
                    bad_input: p.to_string(),
                    line_number: 0, // XXX
                    message: "Error parsing input: invalid token".to_string(),
                })
            }
        }

        tokens.reverse();

        Ok(AstLexer { 
            tokens: Rc::new(RefCell::new(tokens)),
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
        TokenMatch(Token::Eof, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_library() -> TokenLibrary {
        TokenLibrary::build().unwrap()
    }

    #[test]
    fn lexer_new() {
        let tl = token_library();
        let lexer = AstLexer::new("foo bar,\"a\",123 (d, b) + *", &tl).unwrap();

        assert_eq!(lexer.next(), TokenMatch(Token::Reference, "foo"));
        assert_eq!(lexer.next(), TokenMatch(Token::Reference, "bar"));
        assert_eq!(lexer.next(), TokenMatch(Token::Comma, ","));
        assert_eq!(lexer.next(), TokenMatch(Token::DoubleQuotedString,"\"a\""));
        assert_eq!(lexer.next(), TokenMatch(Token::Comma, ","));
        assert_eq!(lexer.next(), TokenMatch(Token::Integer, "123"));
        assert_eq!(lexer.next(), TokenMatch(Token::OpenParen, "("));
        assert_eq!(lexer.next(), TokenMatch(Token::Reference, "d"));
        assert_eq!(lexer.next(), TokenMatch(Token::Comma, ","));
        assert_eq!(lexer.next(), TokenMatch(Token::Reference, "b"));
        assert_eq!(lexer.next(), TokenMatch(Token::CloseParen, ")"));
        assert_eq!(lexer.next(), TokenMatch(Token::InfixOperator, "+"));
        assert_eq!(lexer.next(), TokenMatch(Token::InfixOperator, "*"));
        assert_eq!(lexer.next(), TokenMatch(Token::Eof, ""));
        assert_eq!(lexer.next(), TokenMatch(Token::Eof, ""));
    }

    #[test]
    fn lexer_new_comment() {
        let tl = token_library();
        let lexer = AstLexer::new("# this is a comment\na_ref\n", &tl).unwrap();

        assert_eq!(lexer.next(), TokenMatch(Token::Reference, "a_ref"));
        assert_eq!(lexer.next(), TokenMatch(Token::Eof, ""));
    }

    #[test]
    fn lexer_new_newlines() {
        let tl = token_library();
        let lexer = AstLexer::new("\n foo \n bar", &tl).unwrap();

        assert_eq!(lexer.next(), TokenMatch(Token::Reference, "foo"));
        assert_eq!(lexer.next(), TokenMatch(Token::Reference, "bar"));
    }

    #[test]
    fn lexer_peek() {
        let tl = token_library();
        let lexer = AstLexer::new("foo (bar) + baz", &tl).unwrap();

        assert_eq!(lexer.peek(), TokenMatch(Token::Reference, "foo"));
        assert_eq!(lexer.peek(), TokenMatch(Token::Reference, "foo"));
        assert_eq!(lexer.peek(), TokenMatch(Token::Reference, "foo"));
    }
}
