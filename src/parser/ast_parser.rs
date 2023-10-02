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
use super::ast_lexer::*;
use super::token_library::{Token, TokenMatch};
use crate::ast::{Ast, Node, Variables};
use crate::{Error, ParseResult, Result, Runtime};
use std::collections;

pub struct AstParser<'a> {
    lexer: &'a AstLexer<'a>,
    runtime: &'a Runtime,
}

impl<'a> AstParser<'a> {
    pub fn new(lexer: &'a AstLexer<'a>, runtime: &'a Runtime) -> Self {
        AstParser { lexer, runtime }
    }

    /// Parse `input` from a `SourceCode`.
    pub fn parse(input: &'a str, single_expr: bool, runtime: &'a Runtime) -> Result<Ast> {
        let lexer = AstLexer::new(input, runtime).map_err(|e| Error::CodeSyntaxError {
            message: e.to_string(),
            line_number: e.line_number,
            position: e.position,
            highlighted_lines: runtime
                .source_code
                .highlight_line(e.line_number, e.position),
        })?;
        let parser = AstParser::new(&lexer, runtime);

        parser.expr_bp(single_expr, 0)
    }

    /// Parse `input` from the command line, specified as a simple key/value string like
    /// "foo=1,bar=baz"
    ///
    // TODO: take multiple key values via the same flag.  similar to awk -v foo1=bar -v foo2=bar
    pub fn parse_key_value_str(
        key_values: &'a [String],
        runtime: &'a Runtime,
    ) -> Result<Variables> {
        let mut variables = collections::HashMap::new();

        for kv in key_values.iter() {
            if let Some((key, value)) = kv.split_once('=') {
                variables.insert(key.to_string(), Self::parse(value, false, runtime)?);
            } else {
                return Err(Error::InitError(format!(
                    "Invalid key/value variables: {kv}",
                )));
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
            TokenMatch {
                token: Token::OpenParen,
                ..
            } => {
                let expr = self.expr_bp(single_expr, 0)?;
                match self.lexer.next() {
                    TokenMatch {
                        token: Token::CloseParen,
                        ..
                    } => expr,
                    token => {
                        return self.syntax_error(
                            &token,
                            &format!("Expected close parenthesis, received ({:?})", token),
                        )
                    }
                }
            }

            // terminals
            TokenMatch {
                token: Token::Boolean,
                ..
            } => self.ast_from_str(&lhs_token, Node::boolean_from_str)?,

            TokenMatch {
                token: Token::DateTime,
                ..
            } => self.ast_from_str(&lhs_token, Node::datetime_from_str)?,

            TokenMatch {
                token: Token::DoubleQuotedString,
                ..
            } => self.ast_from_str(&lhs_token, Node::text_from_str)?,

            TokenMatch {
                token: Token::Float,
                ..
            } => self.ast_from_str(&lhs_token, Node::float_from_str)?,

            TokenMatch {
                token: Token::Integer,
                ..
            } => self.ast_from_str(&lhs_token, Node::integer_from_str)?,

            TokenMatch {
                token: Token::Reference,
                ..
            } => self.ast_from_str(&lhs_token, Node::reference_from_str)?,

            _ => {
                return self.syntax_error(
                    &lhs_token,
                    &format!("Invalid left-hand side expression ({:?})", lhs_token),
                )
            }
        };

        loop {
            if single_expr {
                // in the case where we're just looking for a single expr, we can terminate
                // iteration when we see a reference (beginning of `foo := ...`) or `fn`.
                match self.lexer.peek() {
                    TokenMatch {
                        token: Token::Reference,
                        ..
                    }
                    | TokenMatch {
                        token: Token::FunctionDefinition,
                        ..
                    } => break,
                    // otherwise do nothing and the next match statement will do it's thing
                    // (regardless of the `single_expr` context)
                    _ => (),
                }
            }

            let op_token = self.lexer.peek();
            let op = match op_token {
                // end of an expression, stop looping
                TokenMatch {
                    token: Token::Comma,
                    ..
                }
                | TokenMatch {
                    token: Token::CloseParen,
                    ..
                }
                | TokenMatch {
                    token: Token::Eof, ..
                } => break,

                // an infix expression or a function definition
                TokenMatch {
                    token: Token::InfixOperator,
                    str_match: op,
                    ..
                }
                | TokenMatch {
                    token: Token::OpenParen,
                    str_match: op,
                    ..
                } => op,

                // otherwise undefined
                _ => {
                    return self.syntax_error(
                        &op_token,
                        &format!("Unexpected token ({:?})", &op_token.token),
                    )
                }
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
                            TokenMatch {
                                token: Token::CloseParen,
                                ..
                            } => {
                                self.lexer.next();
                                break;
                            }
                            TokenMatch {
                                token: Token::Comma,
                                ..
                            } => {
                                self.lexer.next();
                            }
                            _ => args.push(self.expr_bp(single_expr, 0)?),
                        }
                    }

                    Box::new(Node::FunctionCall { name: id, args })
                } else {
                    return self.syntax_error(&op_token, "Unexpected infix operator");
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
            "=" | "<" | ">" | "<=" | ">=" | "<>" => (5, 6),
            "&" => (7, 8),
            "+" | "-" => (9, 10),
            "*" | "/" => (11, 12),
            "^" => (13, 14),
            _ => return None,
        })
    }

    fn syntax_error(&self, token: &TokenMatch, message: &str) -> Result<Ast> {
        Err(Error::CodeSyntaxError {
            line_number: token.line_number,
            message: message.to_string(),
            position: token.position,
            highlighted_lines: self
                .runtime
                .source_code
                .highlight_line(token.line_number, token.position),
        })
    }

    fn ast_from_str(
        &self,
        token: &TokenMatch,
        from_str_fn: fn(&str) -> ParseResult<Ast>,
    ) -> Result<Ast> {
        from_str_fn(token.str_match).map_err(|e| Error::CodeSyntaxError {
            highlighted_lines: self
                .runtime
                .source_code
                .highlight_line(token.line_number, token.position),
            line_number: token.line_number,
            position: token.position,
            message: e.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn test_parse(input: &str) -> Ast {
        let runtime: Runtime = TestFile::new("xlsx", input).into();
        AstParser::parse(input, false, &runtime).unwrap()
    }

    #[test]
    fn parse_float() {
        assert_eq!(test_parse("1.50"), Box::new(1.50.into()));
        assert_eq!(test_parse("0.65"), Box::new(0.65.into()));
    }

    #[test]
    fn parse_integer() {
        assert_eq!(test_parse("1"), Box::new(1.into()));
    }

    #[test]
    fn parse_infix_function() {
        assert_eq!(
            test_parse("1 * 2"),
            Box::new(Node::infix_fn_call(1.into(), "*", 2.into()))
        );
    }

    #[test]
    fn parse_function_call() {
        assert_eq!(
            test_parse("foo(bar, 1, 2)"),
            Box::new(Node::fn_call(
                "foo",
                &[Node::reference("bar"), 1.into(), 2.into()],
            ))
        );
    }

    #[test]
    fn parse_nested_function_call() {
        assert_eq!(
            test_parse("foo(1, 2 * 3)"),
            Box::new(Node::fn_call(
                "foo",
                &[1.into(), Node::infix_fn_call(2.into(), "*", 3.into()),],
            ))
        );
    }

    #[test]
    fn parse_explicit_precedence() {
        assert_eq!(
            test_parse("1 * ((2 + 3) - 4) / 5"),
            Box::new(Node::infix_fn_call(
                Node::infix_fn_call(
                    1.into(),
                    "*",
                    Node::infix_fn_call(
                        Node::infix_fn_call(2.into(), "+", 3.into()),
                        "-",
                        4.into()
                    )
                ),
                "/",
                5.into()
            ))
        );
    }

    #[test]
    fn parse_infix_precedence() {
        assert_eq!(
            test_parse("1 * 2 + 3 - 4 / 5"),
            Box::new(Node::infix_fn_call(
                Node::infix_fn_call(Node::infix_fn_call(1.into(), "*", 2.into()), "+", 3.into(),),
                "-",
                Node::infix_fn_call(4.into(), "/", 5.into(),),
            ))
        );
    }

    #[test]
    fn parse_key_value_str() {
        let runtime: Runtime = TestFile::new("csv", "foo,bar").into();
        let parsed_vars =
            AstParser::parse_key_value_str(&["foo=bar".to_string(), "baz=1".to_string()], &runtime)
                .unwrap();

        assert_eq!(parsed_vars.len(), 2);
    }

    #[test]
    fn parse_key_value_str_empty() {
        let runtime: Runtime = TestFile::new("csv", "foo,bar").into();
        let parsed_vars = AstParser::parse_key_value_str(&[], &runtime).unwrap();

        assert_eq!(parsed_vars.len(), 0);
    }
}
