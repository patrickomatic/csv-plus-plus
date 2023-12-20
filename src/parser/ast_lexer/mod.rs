//! # AstLexer
//!
use super::TokenMatcher;
use crate::error::ParseResult;
use crate::{ArcSourceCode, CharOffset, LineNumber};
use std::cell::RefCell;
use std::rc::Rc;

mod token;
mod token_library;
mod token_match;
mod unknown_token;

pub(crate) use token::Token;
pub(crate) use token_library::{TokenLibrary, CODE_SECTION_SEPARATOR};
pub(crate) use token_match::TokenMatch;
pub(crate) use unknown_token::UnknownToken;

pub(crate) struct AstLexer<'a> {
    tokens: Rc<RefCell<Vec<TokenMatch<'a>>>>,
    lines: LineNumber,
    eof_position: CharOffset,
    source_code: ArcSourceCode,
    position: Option<a1_notation::Address>,
}

/// For better or worse the tokens are not mutually exclusive - some of them are subsets of another
/// (for example 555.55 could be matched by both float and integer (integer can just match the first
/// part of it)) so it's important float is first. Another example is comments - they have to be
/// stripped out first
fn matchers_ordered(tl: &TokenLibrary) -> [&TokenMatcher<Token>; 17] {
    [
        &tl.newline,
        &tl.comment,
        &tl.double_quoted_string,
        &tl.fn_def,
        &tl.use_module,
        &tl.var_assign,
        &tl.comma,
        &tl.close_paren,
        &tl.open_paren,
        &tl.infix_operator,
        &tl.scope_eof,
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

impl<'a> AstLexer<'a> {
    pub(crate) fn new(
        input: &'a str,
        position: Option<a1_notation::Address>,
        source_code: ArcSourceCode,
    ) -> ParseResult<AstLexer<'a>> {
        let token_library = TokenLibrary::library();
        let mut line_number = 0;
        let mut line_offset = 0;

        let mut tokens: Vec<TokenMatch> = vec![];
        let mut p = input;

        loop {
            let mut matched = false;

            for token_matcher in matchers_ordered(token_library).iter() {
                let token = token_matcher.0;

                if let Some(m) = token_matcher.try_match(p) {
                    if token == Token::Newline {
                        // just count the newline but don't store it on `tokens`
                        line_number += 1;
                    } else if token != Token::Comment {
                        // we'll want to consume everything except for comments and newlines (no point
                        // in the parsing logic needing to consider them)
                        tokens.push(TokenMatch {
                            token,
                            str_match: m.str_match,
                            line_number,
                            line_offset: line_offset + m.len_leading_whitespace,
                            position,
                            source_code: source_code.clone(),
                        });
                    }

                    if token == Token::Newline {
                        // move line_offset back to the beginning
                        line_offset = 0;
                    } else {
                        line_offset += m.len_full_match;
                    }

                    // move the input past the match
                    p = &p[m.len_full_match..];
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
                    line_offset,
                    position,
                    source_code,
                }
                .into());
            }
        }

        tokens.reverse();

        Ok(AstLexer {
            tokens: Rc::new(RefCell::new(tokens)),
            eof_position: line_offset,
            lines: line_number,
            position,
            source_code,
        })
    }

    /// Consume and return the next `TokenMatch`
    pub(super) fn next(&self) -> TokenMatch {
        self.tokens.borrow_mut().pop().unwrap_or_else(|| self.eof())
    }

    /// Return but do not consume the next `TokenMatch`
    pub(super) fn peek(&self) -> TokenMatch {
        match self.tokens.borrow().last() {
            Some(t) => (*t).clone(),
            None => self.eof(),
        }
    }

    fn eof(&self) -> TokenMatch {
        TokenMatch {
            token: Token::Eof,
            str_match: "",
            line_number: self.lines,
            line_offset: self.eof_position,
            position: self.position,
            source_code: self.source_code.clone(),
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
        let lexer = AstLexer::new(
            "foo bar,\"a\",123 \n(d, b) + * 0.25",
            None,
            build_source_code(),
        )
        .unwrap();

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
        let lexer =
            AstLexer::new("# this is a comment\na_ref\n", None, build_source_code()).unwrap();

        assert_token_match_eq!(lexer, Token::Reference, "a_ref", 1, 0);
        assert_token_match_eq!(lexer, Token::Eof, "", 1, 5);
    }

    #[test]
    fn lexer_new_newlines() {
        let lexer = AstLexer::new("\n foo \n bar", None, build_source_code()).unwrap();

        assert_token_match_eq!(lexer, Token::Reference, "foo", 1, 1);
        assert_token_match_eq!(lexer, Token::Reference, "bar", 2, 1);
    }

    #[test]
    fn lexer_peek() {
        let lexer = AstLexer::new("foo (bar) + baz", None, build_source_code()).unwrap();

        assert_eq!(lexer.peek().token, Token::Reference);
        assert_eq!(lexer.peek().str_match, "foo");
        assert_eq!(lexer.peek().token, Token::Reference);
        assert_eq!(lexer.peek().str_match, "foo");
        assert_eq!(lexer.peek().token, Token::Reference);
        assert_eq!(lexer.peek().str_match, "foo");
    }
}
