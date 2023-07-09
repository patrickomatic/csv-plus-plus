//! # csv++ AST Parser
//!
//! A parser that's used for parsing individual expressions.  
//!
//! I used a Pratt parser here because it proivdes the benefits of a hand-written parser (better,
//! more contextual error messaging vs a parser-generator) and it also handls the recursive LHS
//! problems where a recursive descent parser wouldn't.
//!
//! ## Inspired by:
//!
//! * [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
//! * [Which Parsing Approach?](https://news.ycombinator.com/item?id=24480504)
//!
// TODO:
//
// * Handle line numbers
//
use std::collections;
use crate::{Error, Result, TokenLibrary};
use crate::ast::{Ast, Node, Variables};
use super::token_library::{Token, TokenMatch};
use super::ast_lexer::*;

pub struct AstParser<'a> {
    lexer: &'a AstLexer<'a>,
}

impl<'a> AstParser<'a> {
    pub fn new(lexer: &'a AstLexer<'a>) -> Self {
        AstParser { lexer }
    }

    /// Parse `input` from a `SourceCode`.
    pub fn parse(
        input: &'a str,
        single_expr: bool,
        tl: &'a TokenLibrary
    ) -> Result<Ast> {
        let lexer = AstLexer::new(input, tl)?;
        let parser = AstParser::new(&lexer);

        parser.expr_bp(single_expr, 0)
    }

    /// Parse `input` from the command line, specified as a simple key/value string like
    /// "foo=1,bar=baz"
    ///
    // TODO: take multiple key values via the same flag.  similar to awk -v foo1=bar -v foo2=bar
    pub fn parse_key_value_str(
        key_values: Vec<&'a str>,
        tl: &'a TokenLibrary
    ) -> Result<Variables> {
        let mut variables = collections::HashMap::new();

        for kv in key_values.iter() {
            if let Some((key, value)) = kv.split_once('=') {
                variables.insert(key.to_string(), Self::parse(value, false, tl)?);
            } else {
                return Err(Error::InitError(
                        format!("Invalid key/value variables: {}", kv)))
            }
        }
        
        Ok(variables)
    }

    /// The core pratt parser logic for parsing an expression of our AST.  
    pub fn expr_bp(&self, single_expr: bool, min_bp: u8) -> Result<Ast> {
        let mut lhs = match self.lexer.next() {
            // a starting parenthesis means we just need to recurse and consume (expect)
            // the close paren 
            TokenMatch(Token::OpenParen, _) => {
                let expr = self.expr_bp(single_expr, 0)?;
                match self.lexer.next() {
                    TokenMatch(Token::CloseParen, _) => expr,
                    TokenMatch(t, bad_input) => 
                        return Err(Error::CodeSyntaxError {
                            message: format!("Expected close parenthesis, received ({:?})", t),
                            bad_input: bad_input.to_string(),
                            line_number: 0, // XXX
                        }),
                }
            }

            // terminals
            TokenMatch(Token::Boolean, b) => Node::boolean_from_str(b)?,
            TokenMatch(Token::DateTime, d) => Node::datetime_from_str(d)?,
            TokenMatch(Token::DoubleQuotedString, t) => Node::text_from_str(t)?,
            TokenMatch(Token::Float, f) => Node::float_from_str(f)?,
            TokenMatch(Token::Integer, i) => Node::integer_from_str(i)?,
            TokenMatch(Token::Reference, r) => Node::reference_from_str(r)?,

            TokenMatch(t, m) => return Err(Error::CodeSyntaxError {
                message: format!("Invalid left-hand side expression ({:?})", t),
                bad_input: m.to_string(),
                line_number: 0, // XXX
            }),
        };

        loop {
            if single_expr {
                // in the case where we're just looking for a single expr, we can terminate
                // iteration when we see a reference (beginning of `foo := ...`) or `fn`. 
                match self.lexer.peek() {
                    TokenMatch(Token::Reference, _) 
                        | TokenMatch(Token::FunctionDefinition, _) => break,
                    // otherwise do nothing and the next match statement will do it's thing
                    // (regardless of the `single_expr` context)
                    _ => (),
                }
            }

            let op = match self.lexer.peek() {
                // end of an expression, stop looping
                TokenMatch(Token::Comma, _) 
                    | TokenMatch(Token::CloseParen, _) 
                    | TokenMatch(Token::Eof, _) 
                    => break,

                // an infix expression or a function definition
                TokenMatch(Token::InfixOperator, op) 
                    | TokenMatch(Token::OpenParen, op) 
                    => op,

                // otherwise undefined
                TokenMatch(token, v) => return Err(Error::CodeSyntaxError { 
                    bad_input: v.to_string(), 
                    line_number: 0, // XXX
                    message: format!("Unexpected token ({:?})", token),
                }),
            };

            if let Some((l_bp, ())) = self.postfix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }
                
                // consume the token we peeked
                self.lexer.next();

                lhs = if op == "(" {
                    // function call
                    let id = match *lhs {
                        Node::Reference(id) => id,
                        _ => return Err(Error::CodeSyntaxError { 
                            bad_input: lhs.to_string(), 
                            line_number: 0, // XXX
                            message: "Unable to get id for fn".to_string(),
                        }),
                    };

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
                                args.push(self.expr_bp(single_expr, 0)?)
                        }
                    }

                    Box::new(Node::FunctionCall { name: id, args })
                } else {
                    return Err(Error::CodeSyntaxError {
                        bad_input: op.to_owned(),
                        line_number: 0, // XXX
                        message: "Unexpected infix operator".to_string(),
                    })
                };

                continue;
            }

            if let Some((l_bp, r_bp)) = self.infix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }

                // consume the token we peeked
                self.lexer.next();

                let rhs = self.expr_bp(single_expr, r_bp)?;
                lhs = Box::new(Node::InfixFunctionCall {
                    left: lhs, 
                    operator: op.to_owned(), 
                    right: rhs,
                });

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

    fn test_parse(input: &str) -> Ast {
        let tl = TokenLibrary::build().unwrap();
        AstParser::parse(input, false, &tl).unwrap()
    }

    #[test]
    fn parse_integer() {
        assert_eq!(
            test_parse("1"), 
            Box::new(Node::Integer(1)));
    }

    #[test]
    fn parse_infix_function() {
        assert_eq!(
            test_parse("1 * 2"),
            Box::new(Node::InfixFunctionCall {
                left: Box::new(Node::Integer(1)),
                operator: "*".to_owned(), 
                right: Box::new(Node::Integer(2)),
            }));
    }

    #[test]
    fn parse_function_call() {
        assert_eq!(
            test_parse("foo(bar, 1, 2)"),
            Box::new(Node::FunctionCall {
                name: "foo".to_owned(),
                args: vec![
                    Box::new(Node::Reference("bar".to_owned())),
                    Box::new(Node::Integer(1)),
                    Box::new(Node::Integer(2)),
                ],
            }));
    }

    #[test]
    fn parse_nested_function_call() {
        assert_eq!(
            test_parse("foo(1, 2 * 3)"),
            Box::new(Node::FunctionCall {
                name: "foo".to_owned(),
                args: vec![
                    Box::new(Node::Integer(1)),
                    Box::new(Node::InfixFunctionCall {
                        left: Box::new(Node::Integer(2)),
                        operator: "*".to_owned(),
                        right: Box::new(Node::Integer(3))
                    }),
                ],
            }));
    }

    #[test]
    fn parse_explicit_precedence() {
        assert_eq!(
            test_parse("1 * ((2 + 3) - 4) / 5"),
            Box::new(Node::InfixFunctionCall {
                left: Box::new(Node::InfixFunctionCall {
                    left: Box::new(Node::Integer(1)),
                    operator: "*".to_owned(),
                    right: Box::new(Node::InfixFunctionCall {
                        left: Box::new(Node::InfixFunctionCall {
                            left: Box::new(Node::Integer(2)),
                            operator: "+".to_owned(),
                            right: Box::new(Node::Integer(3)),
                        }),
                        operator: "-".to_owned(),
                        right: Box::new(Node::Integer(4)),
                    }),
                }),
                operator: "/".to_owned(),
                right: Box::new(Node::Integer(5)),
            }));
    }

    #[test]
    fn parse_infix_precedence() {
        assert_eq!(
            test_parse("1 * 2 + 3 - 4 / 5"),
            Box::new(Node::InfixFunctionCall {
                left: Box::new(Node::InfixFunctionCall {
                    left: Box::new(Node::InfixFunctionCall {
                        left: Box::new(Node::Integer(1)), 
                        operator: "*".to_owned(), 
                        right: Box::new(Node::Integer(2)),
                    }),
                    operator: "+".to_owned(),
                    right: Box::new(Node::Integer(3)),
                }),
                operator: "-".to_owned(),
                right: Box::new(Node::InfixFunctionCall {
                    left: Box::new(Node::Integer(4)),
                    operator: "/".to_owned(),
                    right: Box::new(Node::Integer(5)),
                }),
            }));
    }

    #[test]
    fn parse_key_value_str() {
        let tl = TokenLibrary::build().unwrap();
        let parsed_vars = AstParser::parse_key_value_str(vec!["foo=bar", "baz=1"], &tl).unwrap();

        assert_eq!(parsed_vars.len(), 2);
    }

    #[test]
    fn parse_key_value_str_empty() {
        let tl = TokenLibrary::build().unwrap();
        let parsed_vars = AstParser::parse_key_value_str(vec![], &tl).unwrap();

        assert_eq!(parsed_vars.len(), 0);
    }
}
