//! # csv++ AST Parser
//!
//! ## Inspired by:
//!
//! * [https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html](Simple but
//! Powerful Pratt Parsing)
//!
//! * [https://news.ycombinator.com/item?id=24480504](Which Parsing Approach?)
// TODO
// * lexer support for floats
use crate::{Error, Node, TokenLibrary};
use super::token_library::{Token, TokenMatch, TokenMatcher};

struct Lexer<'a> {
    tokens: Vec<TokenMatch<'a>>,
}

/// Unfortunately the tokens are not mutually exclusive - some of them are subsets of another (for
/// example 555.55 could be matched by both float and integer (integer can just match the first
/// part of it) so it's important float is first. Another example is comments - they have to be
/// stripped out first
fn matchers_ordered(tl: &TokenLibrary) -> [&TokenMatcher; 12] {
    [
        &tl.comment,
        &tl.double_quoted_string,
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

impl<'a> Lexer<'a> {
    fn new(input: &'a str, token_library: &'a TokenLibrary) -> Result<Lexer<'a>, Error> {
        let mut tokens: Vec<TokenMatch> = vec![];
        let mut p = input.clone();

        loop {
            let mut matched = false;

            for TokenMatcher(token, regex) in matchers_ordered(token_library).iter() {
                if let Some(m) = regex.find(p) {
                    tokens.push(TokenMatch(token.clone(), m.as_str().trim()));

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
                    line_number: 0, // XXX
                    message: "Error parsing input: invalid token".to_string(),
                    bad_input: p.to_string(),
                })
            }
        }

        tokens.reverse();

        Ok(Lexer { tokens })
    }

    fn next(&mut self) -> TokenMatch {
        self.tokens.pop().unwrap_or_else(|| self.eof())
    }

    fn peek(&mut self) -> TokenMatch {
        match self.tokens.last() {
            Some(t) => t.clone(),
            None => self.eof(),
        }
    }

    fn eof(&self) -> TokenMatch {
        TokenMatch(Token::Eof, "")
    }
}

pub struct AstParser<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> AstParser<'a> {
    pub fn parse(input: &'a str, tl: &'a TokenLibrary) -> Result<Node, Error> {
        let mut lexer = Lexer::new(input, tl)?;
        let mut parser = AstParser { lexer: &mut lexer };
        parser.expr_bp(0)
    }

    fn expr_bp(&mut self, _min_bp: u8) -> Result<Node, Error> {
        /*
        let mut lhs = match lexer.next() {
            _ => todo!(),
        }

        loop {
            let op = match lexer.peek() {
                Token::Eof => break,
                Token::InfixOperator(op) => op,

            }
            break;
        }
        */

        Ok(Node::Integer(1))
    }

    fn prefix_binding_power(&self, op: &str) -> ((), u8) {
        match op {
            // TODO `=` could work here, where it signifies that we even begin parsing
            _ => panic!("unknown binding power for operator: {:?}", op),
        }
    }

    fn postfix_binding_power(&self, op: &str) -> Option<(u8, ())> {
        let bp = match op {
            "(" => (15, ()),
            _ => return None,
        };
        Some(bp)
    }

    fn infix_binding_power(&self, op: &str) -> Option<(u8, u8)> {
        let bp = match op {
            ":=" => (2, 1),
            "," => (3, 4),
            "=" | "<" | ">" | "<=" | ">=" | "<>" => (5, 6),
            "&" => (7, 8),
            "+" | "-" => (9, 10),
            "*" | "/" => (11, 12),
            "^" => (13, 14),
            _ => return None,

        };
        Some(bp)
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
        let mut lexer = Lexer::new("foo bar,\"a\",123 (d, b) + *", &tl).unwrap();

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
        let mut lexer = Lexer::new("# this is a comment\na_ref\n", &tl).unwrap();

        assert_eq!(lexer.next(), TokenMatch(Token::Comment, "# this is a comment"));
        assert_eq!(lexer.next(), TokenMatch(Token::Reference, "a_ref"));
        assert_eq!(lexer.next(), TokenMatch(Token::Eof, ""));
    }

    #[test]
    fn lexer_peek() {
        let tl = token_library();
        let mut lexer = Lexer::new("foo (bar) + baz", &tl).unwrap();

        assert_eq!(lexer.peek(), TokenMatch(Token::Reference, "foo"));
        assert_eq!(lexer.peek(), TokenMatch(Token::Reference, "foo"));
        assert_eq!(lexer.peek(), TokenMatch(Token::Reference, "foo"));
    }

    #[test]
    fn ast_parser_parse_integer() {
        let tl = token_library();
        let node = AstParser::parse("1", &tl).unwrap();

        assert_eq!(node, Node::Integer(1));
    }

    /*
    #[test]
    fn ast_parser_parse_infix_function() {
        let node = AstParser::parse("1 * 2".to_string()).unwrap();

        assert_eq!(node, ast::Node::InfixFunctionCall {
            operator: "*".to_string(),
            left_arg: Box::new(ast::Node::Integer(1)),
            right_arg: Box::new(ast::Node::Integer(1)),
        });
    }
    */
}
