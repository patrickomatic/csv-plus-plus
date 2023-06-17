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
    // * function calls
    // * function definitions
    // * parenthesis grouping (I think it should work but write a test?)
    fn expr_bp(&mut self, min_bp: u8) -> Result<Box<dyn Node>> {
        dbg!("at the top");
        let mut lhs = match self.lexer.next() {
            // non-terminals we'll recurse on the LHS
            TokenMatch(Token::OpenParen, _) => self.expr_bp(0)?,

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
                // TokenMatch(Token::Comma, op) => op.to_owned(),
                TokenMatch(Token::CloseParen, op) => op.to_owned(),
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
                    dbg!("parsing a fuction");
                    dbg!(&id);
                    // function call
                    let mut args = vec![];

                    // consume arguments (expressions) until we see a close paren
                    loop {
                        dbg!(self.lexer.peek());
                        match self.lexer.peek() {
                            TokenMatch(Token::CloseParen, _) => {
                                self.lexer.next();
                                break
                            },
                            TokenMatch(Token::Comma, _) => {
                                self.lexer.next();
                            },
                            _ => {
                                dbg!("recursing");
                                args.push(self.expr_bp(0)?)
                            },
                        }
                    }

                    Box::new(FunctionCall { name: id.unwrap(), args, })
                } else {
                    // XXX
                    panic!("foo")
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
                lhs = Box::new(InfixFunctionCall { 
                    left_arg: lhs,
                    right_arg: rhs,
                    operator: op,
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
            ":="                        => (2, 1),
            ","                         => (3, 4), // XXX I don't think we need this here because
                                                   // we handle commas explicitly above
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

    fn token_library() -> TokenLibrary {
        TokenLibrary::build().unwrap()
    }

    #[test]
    fn parse_integer() {
        let tl = token_library();
        let node = AstParser::parse("1", &tl).unwrap();

        assert!(Node::eq(&*node, &Integer(1)))
    }

    #[test]
    fn parse_infix_function() {
        let tl = token_library();
        let node = AstParser::parse("1 * 2", &tl).unwrap();

        assert!(Node::eq(&*node, &InfixFunctionCall {
            operator: "*".to_string(),
            left_arg: Box::new(Integer(1)),
            right_arg: Box::new(Integer(1)),
        }));
    }

    #[test]
    fn parse_function_call() {
        let tl = token_library();
        let node = AstParser::parse("foo(bar, 1, 2)", &tl).unwrap();

        assert!(Node::eq(&*node, &FunctionCall {
            name: "foo".to_string(),
            args: vec![
                Box::new(Reference("bar".to_string())),
                Box::new(Integer(1)),
                Box::new(Integer(2)),
            ],
        }))
    }
}
