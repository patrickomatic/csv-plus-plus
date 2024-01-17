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
use super::ast_lexer::{AstLexer, Token};
use crate::ast::{Ast, Node, Variables};
use crate::error::{BadInput, Error, ParseResult, Result};
use crate::{ArcSourceCode, SourceCode};
use std::collections;
use std::path;

macro_rules! left_power {
    ($p:expr) => {
        ($p, $p + 1)
    };
}

pub(crate) struct AstParser<'a> {
    lexer: &'a AstLexer<'a>,
    single_expr: bool,
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(lexer: &'a AstLexer<'a>, single_expr: bool) -> Self {
        AstParser { lexer, single_expr }
    }

    /// Parse `input` from a `SourceCode`.
    pub(crate) fn parse(
        input: &'a str,
        single_expr: bool,
        position: Option<a1_notation::Address>,
        source_code: ArcSourceCode,
    ) -> ParseResult<Ast> {
        AstParser::new(&AstLexer::new(input, position, source_code)?, single_expr).expr_bp(0)
    }

    /// Parse `input` from the command line, specified as a simple key/value string like
    /// "foo=1,bar=baz"
    ///
    // TODO: take multiple key values via the same flag.  similar to awk -v foo1=bar -v foo2=bar
    pub(crate) fn parse_key_value_str<P: Into<path::PathBuf> + Clone>(
        key_values: &'a [String],
        input_filename: P,
    ) -> Result<Variables> {
        let mut variables = collections::HashMap::new();
        let input_filename = input_filename.into();

        for kv in key_values {
            if let Some((key, value)) = kv.split_once('=') {
                // the lexer requires that we have a `SourceCode`, but we're trying to parse something that
                // came from the CLI. so we kinda need to fudge together a SourceCode here:
                let source_input = format!("{key} := {value}");
                let source_code =
                    ArcSourceCode::new(SourceCode::new(source_input, input_filename.clone()));

                variables.insert(
                    key.to_string(),
                    Self::parse(value, false, None, source_code.clone())
                        .map_err(|e| source_code.code_syntax_error(e))?,
                );
            } else {
                return Err(Error::InitError(format!(
                    "Invalid key/value variables: {kv}"
                )));
            }
        }

        Ok(variables)
    }

    /// The core pratt parser logic for parsing an expression of our AST.  
    pub(super) fn expr_bp(&self, min_bp: u8) -> ParseResult<Ast> {
        let lhs_token = self.lexer.next();

        let mut lhs = match lhs_token.token {
            // a starting parenthesis means we just need to recurse and consume (expect)
            // the close paren
            Token::OpenParen => {
                let expr = self.expr_bp(0)?;
                let t = self.lexer.next();
                match t.token {
                    Token::CloseParen => expr,
                    _ => return Err(t.into_parse_error("Expected close parenthesis (`)`)")),
                }
            }

            // terminals
            Token::Boolean
            | Token::DoubleQuotedString
            | Token::Float
            | Token::Integer
            | Token::Reference => Ast::try_from(lhs_token)?,
            _ => return Err(lhs_token.into_parse_error("Expected an expression")),
        };

        loop {
            if self.single_expr {
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

            if let Some((l_bp, ())) = Self::postfix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }

                // consume the token we peeked
                self.lexer.next();

                lhs = if op == "(" {
                    // function call
                    let Node::Reference(id) = lhs.into_inner() else {
                        return Err(op_token.into_parse_error("Unable to get id for fn"));
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
                            _ => args.push(self.expr_bp(0)?),
                        }
                    }

                    Node::FunctionCall { name: id, args }.into()
                } else {
                    return Err(op_token.into_parse_error("Unexpected infix operator"));
                };

                continue;
            }

            if let Some((l_bp, r_bp)) = Self::infix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }

                // consume the token we peeked
                self.lexer.next();

                let rhs = self.expr_bp(r_bp)?;
                lhs = Node::InfixFunctionCall {
                    left: lhs,
                    operator: op.to_owned(),
                    right: rhs,
                }
                .into();

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    #[allow(dead_code)]
    fn prefix_binding_power(op: &str) -> Option<((), u8)> {
        Some(match op {
            "+" | "-" => ((), 4),
            _ => return None,
        })
    }

    fn postfix_binding_power(op: &str) -> Option<(u8, ())> {
        Some(match op {
            "(" => (16, ()),
            "%" => (5, ()),
            _ => return None,
        })
    }

    fn infix_binding_power(op: &str) -> Option<(u8, u8)> {
        Some(match op {
            ":" => left_power!(1),
            "!" => left_power!(2),
            "~" => left_power!(3),
            "^" => left_power!(6),
            "*" | "/" => left_power!(7),
            "+" | "-" => left_power!(9),
            "&" => left_power!(11),
            "=" | "<" | ">" | "<=" | ">=" | "<>" => left_power!(13),
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::*;

    fn test_parse(input: &str) -> Ast {
        let source_code: SourceCode = (&TestSourceCode::new("xlsx", input)).into();
        AstParser::parse(input, false, None, ArcSourceCode::new(source_code)).unwrap()
    }

    #[test]
    fn parse_float() {
        assert_eq!(test_parse("1.50"), 1.50.into());
        assert_eq!(test_parse("0.65"), 0.65.into());
    }

    #[test]
    fn parse_integer() {
        assert_eq!(test_parse("1"), 1.into());
    }

    #[test]
    fn parse_infix_function() {
        assert_eq!(test_parse("1 * 2"), Node::infix_fn_call(1, "*", 2).into());
    }

    #[test]
    fn parse_function_call() {
        assert_eq!(
            test_parse("foo(bar, 1, 2)"),
            Node::fn_call("foo", &[Node::reference("bar"), 1.into(), 2.into()],).into()
        );
    }

    #[test]
    fn parse_nested_function_call() {
        assert_eq!(
            test_parse("foo(1, 2 * 3)"),
            Node::fn_call("foo", &[1.into(), Node::infix_fn_call(2, "*", 3)]).into()
        );
    }

    #[test]
    fn parse_explicit_precedence() {
        assert_eq!(
            test_parse("1 * ((2 + 3) - 4) / 5"),
            Node::infix_fn_call(
                Node::infix_fn_call(
                    1,
                    "*",
                    Node::infix_fn_call(Node::infix_fn_call(2, "+", 3), "-", 4)
                ),
                "/",
                5
            )
            .into()
        );
    }

    #[test]
    fn parse_infix_precedence() {
        assert_eq!(
            test_parse("1 * 2 + 3 - 4 / 5"),
            Node::infix_fn_call(
                Node::infix_fn_call(
                    Ast::new(1.into()),
                    "*",
                    Node::infix_fn_call(
                        Node::infix_fn_call(Ast::new(2.into()), "+", Ast::new(3.into())),
                        "-",
                        Ast::new(4.into())
                    ),
                ),
                "/",
                Ast::new(5.into())
            )
            .into()
        );
    }

    #[test]
    fn parse_key_value_str() {
        let parsed_vars = AstParser::parse_key_value_str(
            &["foo=bar".to_string(), "baz=1".to_string()],
            "foo.csvpp",
        )
        .unwrap();

        assert_eq!(parsed_vars.len(), 2);
    }

    #[test]
    fn parse_key_value_str_empty() {
        let parsed_vars = AstParser::parse_key_value_str(&[], "foo.csvpp").unwrap();

        assert_eq!(parsed_vars.len(), 0);
    }
}
