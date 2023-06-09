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
// use std::cell::Lazy;
use regex::Regex;

use crate::ast;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    CloseParen,
    Comma,
    Eof,
    Equals,
    InfixOperator(String),
    Integer(i64),
    Float(f64),
    OpenParen,
    Reference(String),
}


struct Lexer {
    tokens: Vec<Token>,
}

// XXX XXX XXX apply regexes iteratively to split up the line
// XXX ignore comments until the end of the line
impl Lexer {
    fn new(input: &str) -> Lexer {
        let re = Regex::new(r"\s*\b\s*|\s+").unwrap();
        // XXX I think I need the lexer to be able to handle strings..splitting on 
        let mut tokens = re.split(input)
            .filter(|it| !it.is_empty())
            .map(|tok| match tok {
                "(" => Token::OpenParen,
                ")" => Token::CloseParen,
                "," => Token::Comma,
                "=" => Token::Equals,
                "+" | "*" | "-" | "/"  | "^"  | "&"
                    | "<" | ">" | "<=" | ">=" | "<>" 
                    => Token::InfixOperator(tok.to_string()),
                _ => {
                    // XXX need to handle floats
                    match tok.parse::<i64>() {
                        Ok(n) => Token::Integer(n),
                        Err(_) => Token::Reference(tok.to_string()),
                    }
                }
            })
            .collect::<Vec<_>>();

        tokens.reverse();

        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        match self.tokens.last() {
            Some(t) => t.clone(),
            None => Token::Eof,
        }
    }
}

pub struct AstParser<'a> {
    lexer: &'a mut Lexer,
}

impl<'a> AstParser<'a> {
    // TODO throw a CsvppError instead
    pub fn parse(input: String) -> Result<ast::Node, String> {
        let mut lexer = Lexer::new(input.as_str());
        let mut parser = AstParser { lexer: &mut lexer };
        parser.expr_bp(0)
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<ast::Node, String> {
        /*
        let mut lhs = match lexer.next() {
            Token::Reference(r) => 
        }

        loop {
            let op = match lexer.peek() {
                Token::Eof => break,
                Token::InfixOperator(op) => op,



            }
        }
        */

        Ok(ast::Node::Integer(1))
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

    #[test]
    fn lexer_new() {
        let mut lexer = Lexer::new("foo =bar,\"a\",123 (d, b) + *");

        assert_eq!(lexer.next(), Token::Reference("foo".to_string()));
        assert_eq!(lexer.next(), Token::Equals);
        assert_eq!(lexer.next(), Token::Reference("bar".to_string()));
        assert_eq!(lexer.next(), Token::Comma);
        assert_eq!(lexer.next(), Token::Reference("\"a\"".to_string()));
        assert_eq!(lexer.next(), Token::Comma);
        assert_eq!(lexer.next(), Token::Integer(123));
        assert_eq!(lexer.next(), Token::OpenParen);
        assert_eq!(lexer.next(), Token::Reference("d".to_string()));
        assert_eq!(lexer.next(), Token::Comma);
        assert_eq!(lexer.next(), Token::Reference("b".to_string()));
        assert_eq!(lexer.next(), Token::CloseParen);
        assert_eq!(lexer.next(), Token::InfixOperator("+".to_string()));
        assert_eq!(lexer.next(), Token::InfixOperator("*".to_string()));
    }

    #[test]
    fn lexer_peek() {
        let mut lexer = Lexer::new("foo (bar) + baz");

        assert_eq!(lexer.peek(), Token::Reference("foo".to_string()));
        assert_eq!(lexer.peek(), Token::Reference("foo".to_string()));
        assert_eq!(lexer.peek(), Token::Reference("foo".to_string()));
    }

    #[test]
    fn ast_parser_parse_integer() {
        let node = AstParser::parse("1".to_string()).unwrap();

        assert_eq!(node, ast::Node::Integer(1));
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
