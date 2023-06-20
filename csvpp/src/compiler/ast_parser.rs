//! # csv++ AST Parser
//!
//! ## Inspired by:
//!
//! * [https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html](Simple but
//! Powerful Pratt Parsing)
//!
//! * [https://news.ycombinator.com/item?id=24480504](Which Parsing Approach?)
//!
use std::str::FromStr;
use crate::{
    Boolean, 
    Error, 
    Float, 
    FunctionCall, 
    InfixFunctionCall,
    Integer, 
    Node, 
    Reference, 
    Result, 
    Text, 
    TokenLibrary, 
};
use super::token_library::{Token, TokenMatch};
use super::ast_lexer::*;

pub struct AstParser<'a> {
    lexer: AstLexer<'a>,
}

impl<'a> AstParser<'a> {
    pub fn parse(
        input: &'a str,
        tl: &'a TokenLibrary
    ) -> Result<Box<dyn Node>> {
        let lexer = AstLexer::new(input, tl)?;
        let mut parser = AstParser { lexer };

        parser.expr_bp(0)
    }

    // TODO: need to handle:
    // * variable definitions
    // * function definitions
    // * parenthesis grouping (I think it should work but write a test?)
    fn expr_bp(&mut self, min_bp: u8) -> Result<Box<dyn Node>> {
        let mut lhs = match self.lexer.next() {
            // a starting parenthesis means we just need to recurse and consume (expect)
            // the close paren 
            TokenMatch(Token::OpenParen, _) => {
                let expr = self.expr_bp(0)?;

                match self.lexer.next() {
                    TokenMatch(Token::CloseParen, _) => 
                        expr,
                    TokenMatch(t, bad_input) => 
                        return Err(Error::CodeSyntaxError {
                            message: format!("Expected close parenthesis, received ({:?})", t),
                            bad_input: bad_input.to_string(),
                            line_number: 0, // XXX
                        }),
                }
            }

            // terminals
            TokenMatch(Token::Boolean, b) => Box::new(Boolean::from_str(b)?),
            TokenMatch(Token::DoubleQuotedString, t) => Box::new(Text::from_str(t)?),
            TokenMatch(Token::Float, f) => Box::new(Float::from_str(f)?),
            TokenMatch(Token::Integer, i) => Box::new(Integer::from_str(i)?),
            TokenMatch(Token::Reference, t) => Box::new(Reference::from_str(t)?),

            TokenMatch(t, m) => return Err(Error::CodeSyntaxError {
                message: format!("Invalid left-hand side expression ({:?})", t),
                bad_input: m.to_string(),
                line_number: 0, // XXX
            }),
        };

        loop {
            let op = match self.lexer.peek() {
                // end of an expression
                TokenMatch(Token::Comma, _) => break,
                TokenMatch(Token::CloseParen, _) => break,
                TokenMatch(Token::Eof, _) => break,

                TokenMatch(Token::InfixOperator, op) => op.to_owned(),
                TokenMatch(Token::OpenParen, op) => op.to_owned(),
                TokenMatch(token, v) => return Err(Error::CodeSyntaxError { 
                    bad_input: v.to_string(), 
                    line_number: 0, // XXX
                    message: format!("Unexpected token ({:?})", token),
                }),
            };

            if let Some((l_bp, ())) = self.postfix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }
                
                // consume the token we peeked
                self.lexer.next();

                let id = lhs.id_ref();
                lhs = if op == "(" && id.is_some() {
                    // function call
                    let mut args = vec![];

                    // consume arguments (expressions) until we see a close paren
                    loop {
                        match self.lexer.peek() {
                            TokenMatch(Token::CloseParen, _) => {
                                self.lexer.next();
                                break
                            },
                            TokenMatch(Token::Comma, _) => {
                                self.lexer.next();
                            },
                            _ => 
                                args.push(self.expr_bp(0)?)
                        }
                    }

                    Box::new(FunctionCall { name: id.unwrap(), args, })
                } else {
                    return Err(Error::CodeSyntaxError {
                        bad_input: op,
                        line_number: 0, // XXX
                        message: "Unexpected infix operator".to_string(),
                    })
                };

                continue;
            }

            if let Some((l_bp, r_bp)) = self.infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                // consume the token we peeked
                self.lexer.next();

                let rhs = self.expr_bp(r_bp)?;
                lhs = Box::new(InfixFunctionCall { left: lhs, operator: op, right: rhs });

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn postfix_binding_power(&self, op: &str) -> Option<(u8, ())> {
        Some(match op {
            "(" => (15, ()),
            _ => return None,
        })
    }

    fn infix_binding_power(&self, op: &str) -> Option<(u8, u8)> {
        Some(match op {
            ":="                        => (2, 1),
            "=" | "<"  | ">"  | 
                  "<=" | ">=" | "<>"    => (5, 6),
            "&"                         => (7, 8),
            "+" | "-"                   => (9, 10),
            "*" | "/"                   => (11, 12),
            "^"                         => (13, 14),
            _                           => return None,

        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(input: &str) -> Box<dyn Node> {
        let tl = TokenLibrary::build().unwrap();
        AstParser::parse(input, &tl).unwrap()
    }

    #[test]
    fn parse_integer() {
        let equal_to: Box<dyn Node> = Box::new(Integer(1));

        assert_eq!(&equal_to, &test_parse("1"));
    }

    #[test]
    fn parse_infix_function() {
        let equal_to: Box<dyn Node> = Box::new(
            InfixFunctionCall::new(Integer(1), "*", Integer(2)),
        );

        assert_eq!(&equal_to, &test_parse("1 * 2"));
    }

    #[test]
    fn parse_function_call() {
        let equal_to: Box<dyn Node> = Box::new(FunctionCall::new(
            "foo",
            vec![
                Box::new(Reference::new("bar")),
                Box::new(Integer(1)),
                Box::new(Integer(2)),
            ],
        ));

        assert_eq!(&equal_to, &test_parse("foo(bar, 1, 2)"));
    }

    #[test]
    fn parse_nested_function_call() {
        let equal_to: Box<dyn Node> = Box::new(FunctionCall::new(
            "foo",
            vec![
                Box::new(Integer(1)),
                Box::new(InfixFunctionCall::new(Integer(2), "*", Integer(3))),
            ],
        ));

        assert_eq!(&equal_to, &test_parse("foo(1, 2 * 3)"));
    }

    #[test]
    fn parse_explicit_precedence() {
        let equal_to: Box<dyn Node> = Box::new(
            InfixFunctionCall::new(
                InfixFunctionCall::new(
                    Integer(1),
                    "*",
                    InfixFunctionCall::new(
                        InfixFunctionCall::new(
                            Integer(2),
                            "+",
                            Integer(3),
                        ),
                        "-",
                        Integer(4),
                    ),
                ),
                "/",
                Integer(5),
            )
        );

        assert_eq!(&equal_to, &test_parse("1 * ((2 + 3) - 4) / 5"));
    }

    #[test]
    fn parse_infix_precedence() {
        let equal_to: Box<dyn Node> = Box::new(
            InfixFunctionCall::new(
                InfixFunctionCall::new(
                    InfixFunctionCall::new(Integer(1), "*", Integer(2)),
                    "+",
                    Integer(3),
                ),
                "-",
                InfixFunctionCall::new(Integer(4), "/", Integer(5)),
            )
        );

        assert_eq!(&equal_to, &test_parse("1 * 2 + 3 - 4 / 5"));
    }
}
