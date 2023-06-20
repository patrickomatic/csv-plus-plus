//! # CodeSectionParser
//!
//! The `CodeSectionParser` relies on the `AstParser` to parse individual expressions, but it
//! handles the parsing of functions and variables.
//!
//! It will be looking for something like:
//!
//! ```bnf
//! <var_name> := <expr>
//! ```
//!
//! or 
//!
//! ```bnf
//! fn <function_name>(<fn-arg-1>, <fn-arg-2>, ...) <expr>
//! ```
//!
use std::collections::HashMap;

use super::ast_lexer::AstLexer;
use super::ast_parser::AstParser;
use crate::{Error, Function, Node, Result};
use crate::compiler::token_library::{Token, TokenLibrary, TokenMatch};

type Functions = HashMap<String, Function>;
type Variables =  HashMap<String, Box<dyn Node>>;

#[derive(Debug)]
pub struct FunctionsAndVariables {
    pub functions: Functions,
    pub variables: Variables,
}

pub struct CodeSectionParser<'a> {
    lexer: AstLexer<'a>,
}

/// A recursive descent parser which relies on `AstParser` for individual expressions.  As 
/// mentioned above, the contract here is that this parser handles parsing a series of 
/// function and variable references and delegates to `AstParser` for handling expressions
impl<'a> CodeSectionParser<'a> {
    pub fn parse(input: &'a str, tl: &'a TokenLibrary) -> Result<FunctionsAndVariables> {
        let lexer = AstLexer::new(input, tl)?;
        let mut parser = CodeSectionParser { lexer };
        
        Ok(parser.parse_code_section()?)
    }

    /*
    pub fn functions(&self) -> Functions {
        HashMap::new()
        // XXX
        //self.functions
    }

    pub fn variables(&self) -> Variables {
        HashMap::new()
        // XXX
        // self.variables
    }
    */

    fn parse_code_section(&'a mut self) -> Result<FunctionsAndVariables> {
        let mut variables = HashMap::new();
        let mut functions = HashMap::new();

        loop {
            match self.lexer.next() {
                TokenMatch(Token::FunctionDefinition, _) => {
                    let function = self.parse_fn_definition()?;
                    functions.insert(function.name.clone(), function);
                },
                TokenMatch(Token::Reference, r) => {
                    let var_id = r.to_string();
                    let expr = self.parse_variable_assign()?;
                    variables.insert(var_id, expr);
                },
                TokenMatch(t, m) => {
                    return Err(Error::CodeSyntaxError {
                        message: format!("Expected an `fn` or variable definition (`:=`) operator but saw ({:?})", t),
                        bad_input: m.to_string(),
                        line_number: 0, // XXX
                    })
                },
            }

            break;
        }

        Ok(FunctionsAndVariables { functions, variables })
    }

    fn parse_variable_assign(&'a mut self) -> Result<Box<dyn Node>> {
        // they better give us a :=
        match self.lexer.next() {
            TokenMatch(Token::VarAssign, _) => {
                // consume an expression
                Ok(self.parse_expr()?)
            }
            TokenMatch(t, m) => {
                Err(Error::CodeSyntaxError {
                    message: format!("Expected a variable definition operator (`:=`) but saw ({:?})", t),
                    bad_input: m.to_string(),
                    line_number: 0, // XXX
                })
            }
        }
    }

    /// We're looking to parse a string of the form:
    ///
    /// ```ebnf
    /// 'fn' <name-ref> '(' { <arg-ref> ',' } ')' <expr>
    /// ```
    fn parse_fn_definition(&'a mut self) -> Result<Function> {
        // expect the function name (as a `Reference`)
        let name = match self.lexer.next() {
            TokenMatch(Token::Reference, r) =>
                r.to_string(),
            TokenMatch(t, m) => 
                return Err(Error::CodeSyntaxError { 
                    bad_input: m.to_string(), 
                    line_number: 0,  // XXX
                    message: format!("Expected a function name but saw ({:?})", t),
                }),
        };

        // expect a `(`
        match self.lexer.next() {
            TokenMatch(Token::OpenParen, _) => (),
            TokenMatch(t, m) => 
                return Err(Error::CodeSyntaxError { 
                    bad_input: m.to_string(), 
                    line_number: 0,  // XXX
                    message: format!("Expected `(` but saw ({:?})", t),
                }),
        };

        let mut args = vec![];

        // here we're looking for zero to many References representing the function arguments.
        // this is different than a `FunctionCall` where the arguments to the function can be
        // expressions themselves.
        loop {
            match self.lexer.peek() {
                TokenMatch(Token::CloseParen, _) => {
                    self.lexer.next();
                    break
                },
                TokenMatch(Token::Comma, _) => {
                    self.lexer.next();
                },
                TokenMatch(Token::Reference, r) => {
                    let arg_name = r.to_string();
                    self.lexer.next();
                    args.push(arg_name);
                },
                TokenMatch(t, m) => {
                    return Err(Error::CodeSyntaxError { 
                        bad_input: m.to_string(), 
                        line_number: 0,  // XXX
                        message: format!("Expected `(` but saw ({:?})", t),
                    })
                },
            }
        }

        // and finally the body is just a single expression
        let body = self.parse_expr()?;

        Ok(Function { args, body, name })
    }

    fn parse_expr(&'a mut self) -> Result<Box<dyn Node>> {
        // create an `AstParser` and give it a mutable reference to our lexer
        let mut ast_parser = AstParser::new(&mut self.lexer);
        ast_parser.expr_bp(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{TokenLibrary, InfixFunctionCall, Integer, Reference};
    use super::*;

    fn build_token_library() -> TokenLibrary {
        TokenLibrary::build().unwrap()
    }

    fn test(input: &str) -> FunctionsAndVariables {
        CodeSectionParser::parse(input, &build_token_library()).unwrap()
    }
    
    #[test]
    fn parse_function() {
        let fns_and_vars = test("fn foo(a, b) a + b");
        let foo = fns_and_vars.functions.get("foo").unwrap();

        assert_eq!(foo.name, "foo");
        assert_eq!(foo.args, vec!["a".to_string(), "b".to_string()]);

        let body: Box<dyn Node> = Box::new(
            InfixFunctionCall::new(Reference::new("a"), "+", Reference::new("b")));
        assert_eq!(&foo.body, &body);
    }

    #[test]
    fn parse_function_no_args() {
        let fns_and_vars = test("fn foo() 1 * 2");
        let foo = fns_and_vars.functions.get("foo").unwrap();

        assert_eq!(foo.name, "foo");
        assert_eq!(foo.args.len(), 0);

        let body: Box<dyn Node> = Box::new(
            InfixFunctionCall::new(Integer(1), "*", Integer(2)));
        assert_eq!(&foo.body, &body);
    }

    #[test]
    fn parse_variables() {
        let fns_and_vars = test("foo := \"bar\"");
        let foo = fns_and_vars.variables.get("foo");

        assert!(foo.is_some());
    }
}
