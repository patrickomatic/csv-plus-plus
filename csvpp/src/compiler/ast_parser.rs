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
use crate::{Error, InnerResult, Result, SourceCode};
use crate::ast::{Ast, Node, Variables};
use super::token_library::{Token, TokenMatch};
use super::ast_lexer::*;

pub struct AstParser<'a> {
    lexer: &'a AstLexer<'a>,
    source_code: Option<&'a SourceCode>,
}

impl<'a> AstParser<'a> {
    pub fn new(lexer: &'a AstLexer<'a>, source_code: Option<&'a SourceCode>) -> Self {
        AstParser { lexer, source_code }
    }

    /// Parse `input` from a `SourceCode`.
    pub fn parse(
        input: &'a str,
        single_expr: bool,
        source_code: Option<&'a SourceCode>,
    ) -> Result<Ast> {
        let lexer = AstLexer::new(input).map_err(|e| {
            if let Some(source) = source_code {
                Error::CodeSyntaxError { 
                    message: e.to_string(), 
                    line_number: e.line_number,
                    position: e.position,
                    highlighted_lines: source.highlight_line(e.line_number, e.position),
                }
            } else {
                Error::InitError(e.to_string())
            }
        })?;
        let parser = AstParser::new(&lexer, source_code);

        parser.expr_bp(single_expr, 0)
    }

    /// Parse `input` from the command line, specified as a simple key/value string like
    /// "foo=1,bar=baz"
    ///
    // TODO: take multiple key values via the same flag.  similar to awk -v foo1=bar -v foo2=bar
    pub fn parse_key_value_str(key_values: Vec<&'a str>) -> Result<Variables> {
        let mut variables = collections::HashMap::new();

        for kv in key_values.iter() {
            if let Some((key, value)) = kv.split_once('=') {
                variables.insert(
                    key.to_string(),
                    Self::parse(value, false, None)?);
            } else {
                return Err(Error::InitError(
                        format!("Invalid key/value variables: {}", kv)))
            }
        }
        
        Ok(variables)
    }

    /// The core pratt parser logic for parsing an expression of our AST.  
    pub fn expr_bp(&self, single_expr: bool, min_bp: u8) -> Result<Ast> {
        let lhs_token = self.lexer.next();

        let mut lhs = match lhs_token {
            // a starting parenthesis means we just need to recurse and consume (expect)
            // the close paren 
            TokenMatch { token: Token::OpenParen,  .. } => {
                let expr = self.expr_bp(single_expr, 0)?;
                match self.lexer.next() {
                    TokenMatch { token: Token::CloseParen, .. } => expr,
                    token => 
                        return self.syntax_error(
                            &token,
                            &format!("Expected close parenthesis, received ({:?})", token)),
                }
            }

            // terminals
            TokenMatch { token: Token::Boolean, .. } => 
                self.ast_from_str(&lhs_token, Node::boolean_from_str)?,

            TokenMatch { token: Token::DateTime, .. } => 
                self.ast_from_str(&lhs_token, Node::datetime_from_str)?,

            TokenMatch { token: Token::DoubleQuotedString, .. } => 
                self.ast_from_str(&lhs_token, Node::text_from_str)?,

            TokenMatch { token: Token::Float, .. } => 
                self.ast_from_str(&lhs_token, Node::float_from_str)?,

            TokenMatch { token: Token::Integer, .. } => 
                self.ast_from_str(&lhs_token, Node::integer_from_str)?,

            TokenMatch { token: Token::Reference, .. } => 
                self.ast_from_str(&lhs_token, Node::reference_from_str)?,

            _ =>
                return self.syntax_error(
                    &lhs_token,
                    &format!("Invalid left-hand side expression ({:?})", lhs_token)),
        };

        loop {
            if single_expr {
                // in the case where we're just looking for a single expr, we can terminate
                // iteration when we see a reference (beginning of `foo := ...`) or `fn`. 
                match self.lexer.peek() {
                    TokenMatch { token: Token::Reference, .. } 
                        | TokenMatch { token: Token::FunctionDefinition, .. } => break,
                    // otherwise do nothing and the next match statement will do it's thing
                    // (regardless of the `single_expr` context)
                    _ => (),
                }
            }

            let op_token = self.lexer.peek();
            let op = match op_token {
                // end of an expression, stop looping
                TokenMatch { token: Token::Comma, .. } 
                    | TokenMatch { token: Token::CloseParen, .. }
                    | TokenMatch { token: Token::Eof, .. } 
                    => break,

                // an infix expression or a function definition
                TokenMatch { token: Token::InfixOperator, str_match: op, .. }
                    | TokenMatch { token: Token::OpenParen, str_match: op, .. } 
                    => op,

                // otherwise undefined
                _ =>
                    return self.syntax_error(
                        &op_token,
                        &format!("Unexpected token ({:?})", &op_token.token)),
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
                        _ => return self.syntax_error(&op_token, "Unable to get id for fn"),
                    };

                    let mut args = vec![];

                    // consume arguments (expressions) until we see a close paren
                    loop {
                        match self.lexer.peek() {
                            TokenMatch { token: Token::CloseParen, .. } => {
                                self.lexer.next();
                                break
                            },
                            TokenMatch { token: Token::Comma, .. } => {
                                self.lexer.next();
                            },
                            _ => 
                                args.push(self.expr_bp(single_expr, 0)?)
                        }
                    }

                    Box::new(Node::FunctionCall { name: id, args })
                } else {
                    return self.syntax_error(&op_token, "Unexpected infix operator")
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

    fn syntax_error(&self, token: &TokenMatch, message: &str) -> Result<Ast> {
        if let Some(source_code) = self.source_code {
            Err(Error::CodeSyntaxError { 
                line_number: token.line_number,
                message: message.to_string(),
                position: token.position,
                highlighted_lines: source_code.highlight_line(token.line_number, token.position)
            })
        } else {
            Err(Error::InitError(message.to_string()))
        }
    }

    fn ast_from_str(&self, token: &TokenMatch, from_str_fn: fn(&str) -> InnerResult<Ast>) -> Result<Ast> {
        from_str_fn(token.str_match).map_err(|e| {
            if let Some(source_code) = self.source_code {
                Error::CodeSyntaxError {
                    highlighted_lines: source_code.highlight_line(token.line_number, token.position),
                    line_number: token.line_number,
                    position: token.position,
                    message: e.to_string()
                }
            } else {
                // we haven't even loaded the source code yet - this happens when we're trying to
                // parse the CLI-supplied key/values
                Error::InitError(e.to_string())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(input: &str) -> Ast {
        AstParser::parse(input, false, None).unwrap()
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
        let parsed_vars = AstParser::parse_key_value_str(vec!["foo=bar", "baz=1"]).unwrap();

        assert_eq!(parsed_vars.len(), 2);
    }

    #[test]
    fn parse_key_value_str_empty() {
        let parsed_vars = AstParser::parse_key_value_str(vec![]).unwrap();

        assert_eq!(parsed_vars.len(), 0);
    }
}
