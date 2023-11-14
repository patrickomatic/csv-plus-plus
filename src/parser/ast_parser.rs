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
use super::ast_lexer::*;
use crate::ast::{Ast, Node, Variables};
use crate::error::{BadInput, Error, ParseResult, Result};
use crate::Runtime;
use std::collections;

pub(crate) struct AstParser<'a> {
    lexer: &'a AstLexer<'a>,
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(lexer: &'a AstLexer<'a>) -> Self {
        AstParser { lexer }
    }

    /// Parse `input` from a `SourceCode`.
    pub(crate) fn parse(input: &'a str, single_expr: bool, runtime: &'a Runtime) -> Result<Ast> {
        let lexer =
            AstLexer::new(input, runtime).map_err(|e| runtime.source_code.code_syntax_error(e))?;

        AstParser::new(&lexer)
            .expr_bp(single_expr, 0)
            .map_err(|e| runtime.source_code.code_syntax_error(e))
    }

    /// Parse `input` from the command line, specified as a simple key/value string like
    /// "foo=1,bar=baz"
    ///
    // TODO: take multiple key values via the same flag.  similar to awk -v foo1=bar -v foo2=bar
    pub(crate) fn parse_key_value_str(
        key_values: &'a [String],
        runtime: &'a Runtime,
    ) -> Result<Variables> {
        let mut variables = collections::HashMap::new();

        for kv in key_values.iter() {
            if let Some((key, value)) = kv.split_once('=') {
                variables.insert(key.to_string(), Self::parse(value, false, runtime)?);
            } else {
                return Err(Error::InitError(format!(
                    "Invalid key/value variables: {kv}"
                )));
            }
        }

        Ok(variables)
    }

    /// The core pratt parser logic for parsing an expression of our AST.  
    pub(super) fn expr_bp(&self, single_expr: bool, min_bp: u8) -> ParseResult<Ast> {
        let lhs_token = self.lexer.next();

        let mut lhs = match lhs_token.token {
            // a starting parenthesis means we just need to recurse and consume (expect)
            // the close paren
            Token::OpenParen => {
                let expr = self.expr_bp(single_expr, 0)?;
                match self.lexer.next() {
                    TokenMatch {
                        token: Token::CloseParen,
                        ..
                    } => expr,
                    token => {
                        return Err(token.into_parse_error(format!(
                            "Expected close parenthesis (`)`), received ({token})"
                        )))
                    }
                }
            }

            // terminals
            Token::Boolean
            | Token::DateTime
            | Token::DoubleQuotedString
            | Token::Float
            | Token::Integer
            | Token::Reference => Ast::try_from(lhs_token)?,
            _ => {
                return Err(
                    lhs_token.into_parse_error("Invalid left-hand side expression ({lhs_token})")
                )
            }
        };

        loop {
            if single_expr {
                // in the case where we're just looking for a single expr, we can terminate
                // iteration when we see a reference (beginning of `foo := ...`) or `fn`.
                match self.lexer.peek().token {
                    Token::Reference | Token::FunctionDefinition => break,
                    // otherwise do nothing and the next match statement will do it's thing
                    // (regardless of the `single_expr` context)
                    _ => (),
                }
            }

            let op_token = self.lexer.peek();
            let op = match op_token.token {
                // end of an expression, stop looping
                Token::Comma | Token::CloseParen | Token::Eof => break,

                // an infix expression or a function definition
                Token::InfixOperator | Token::OpenParen => op_token.str_match,

                // otherwise undefined
                t => return Err(op_token.into_parse_error(format!("Unexpected token ({t:?})"))),
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
                        _ => return Err(op_token.into_parse_error("Unable to get id for fn")),
                    };

                    let mut args = vec![];

                    // consume arguments (expressions) until we see a close paren
                    loop {
                        match self.lexer.peek().token {
                            Token::CloseParen => {
                                self.lexer.next();
                                break;
                            }
                            Token::Comma => {
                                self.lexer.next();
                            }
                            _ => args.push(self.expr_bp(single_expr, 0)?),
                        }
                    }

                    Box::new(Node::FunctionCall { name: id, args })
                } else {
                    return Err(op_token.into_parse_error("Unexpected infix operator"));
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn test_parse(input: &str) -> Ast {
        let runtime: Runtime = (&TestSourceCode::new("xlsx", input)).into();
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
        let runtime: Runtime = (&TestSourceCode::new("csv", "foo,bar")).into();
        let parsed_vars =
            AstParser::parse_key_value_str(&["foo=bar".to_string(), "baz=1".to_string()], &runtime)
                .unwrap();

        assert_eq!(parsed_vars.len(), 2);
    }

    #[test]
    fn parse_key_value_str_empty() {
        let runtime: Runtime = (&TestSourceCode::new("csv", "foo,bar")).into();
        let parsed_vars = AstParser::parse_key_value_str(&[], &runtime).unwrap();

        assert_eq!(parsed_vars.len(), 0);
    }
}
