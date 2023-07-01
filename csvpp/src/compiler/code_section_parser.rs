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

use crate::{Error, Result};
use crate::ast::{Ast, Function, Functions, Variable, Variables};
use super::token_library::{Token, TokenLibrary, TokenMatch};
use super::ast_lexer::AstLexer;
use super::ast_parser::AstParser;

#[derive(Debug)]
pub struct CodeSection {
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
    pub fn parse(input: &'a str, tl: &'a TokenLibrary) -> Result<CodeSection> {
        let lexer = AstLexer::new(input, tl)?;
        let parser = CodeSectionParser { lexer };
        
        parser.parse_code_section()
    }

    /// our entry point - just expects a series of variable and function definitions in any order
    fn parse_code_section(&'a self) -> Result<CodeSection> {
        let mut variables = HashMap::new();
        let mut functions = HashMap::new();

        loop {
            match self.lexer.next() {
                TokenMatch(Token::Eof, _) => break,
                TokenMatch(Token::FunctionDefinition, _) => {
                    let function = self.parse_fn_definition()?;
                    let fn_name = function.name.clone();
                    let ast: Ast = Box::new(function);

                    functions.insert(fn_name, ast);
                },
                TokenMatch(Token::Reference, r) => {
                    let expr = self.parse_variable_assign()?;
                    let variable: Ast = Box::new(Variable::new(r, expr));

                    variables.insert(r.to_string(), variable);
                },
                TokenMatch(t, m) => {
                    return Err(Error::CodeSyntaxError {
                        message: format!("Expected an `fn` or variable definition (`:=`) operator but saw ({:?})", t),
                        bad_input: m.to_string(),
                        line_number: 0, // XXX
                    })
                },
            }
        }

        Ok(CodeSection { functions, variables })
    }

    /// parses a `:=` folloed by an `<expr>`
    fn parse_variable_assign(&'a self) -> Result<Ast> {
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
    fn parse_fn_definition(&'a self) -> Result<Function> {
        // expect the function name (as a `Reference`)
        let name = match self.lexer.next() {
            TokenMatch(Token::Reference, r) => r,
            TokenMatch(t, m) => return Err(Error::CodeSyntaxError { 
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

        // here we're looking for zero or more References representing the function arguments.
        // this is different than a `FunctionCall` where the arguments to the function can be
        // expressions themselves.
        loop {
            match self.lexer.next() {
                TokenMatch(Token::CloseParen, _) => {
                    break
                },
                TokenMatch(Token::Comma, _) => (),
                TokenMatch(Token::Reference, r) => {
                    args.push(r.to_string());
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

        // Ok(Function::new( { args, body, name })
        Ok(Function::new(name, args, body))
    }

    fn parse_expr(&'a self) -> Result<Ast> {
        // create an `AstParser` with a reference to our lexer so it can continue consuming our
        // stream of tokens
        AstParser::new(&self.lexer).expr_bp(true, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenLibrary;
    use crate::ast::{InfixFunctionCall, Integer, Reference};
    use super::*;

    fn build_token_library() -> TokenLibrary {
        TokenLibrary::build().unwrap()
    }

    fn test(input: &str) -> CodeSection {
        CodeSectionParser::parse(input, &build_token_library()).unwrap()
    }
    
    #[test]
    fn parse_function() {
        let fns_and_vars = test("fn foo(a, b) a + b");
        let foo = fns_and_vars.functions.get("foo").unwrap();

        let expected: Ast = Box::new(
                   Function {
                       name: "foo".to_owned(),
                       args: vec!["a".to_string(), "b".to_string()],
                       body: Box::new(
                           InfixFunctionCall::new(Reference::new("a"), "+", Reference::new("b")),
                       ),
                   });
            
        assert_eq!(foo, &expected);
    }

    #[test]
    fn parse_function_without_args() {
        let fns_and_vars = test("fn foo() 1 * 2");
        let foo = fns_and_vars.functions.get("foo").unwrap();

        let expected: Ast = Box::new(
                   Function {
                       name: "foo".to_owned(),
                       args: vec![],
                       body: Box::new(
                           InfixFunctionCall::new(Integer(1), "*", Integer(2)),
                       ),
                   });

        assert_eq!(foo, &expected);
    }

    #[test]
    fn parse_multiple_functions() {
        let fns_and_vars = test(r#"
fn foo()
    1 * 2
fn bar(a, b)
    a + b
"#);

        assert_eq!(fns_and_vars.functions.len(), 2);
    }

    #[test]
    fn parse_variables() {
        let fns_and_vars = test("foo := \"bar\"");

        assert!(fns_and_vars.variables.get("foo").is_some());
    }

    #[test]
    fn parse_variables_and_functions() {
        let fns_and_vars = test(r#"
fn foo_fn() 1 * 2
foo_var := 3 * 4 + 5
fn bar_fn(a, b) a + b
bar_var := D1
"#);

        assert!(fns_and_vars.functions.get("foo_fn").is_some());
        assert!(fns_and_vars.functions.get("bar_fn").is_some());

        assert!(fns_and_vars.variables.get("foo_var").is_some());
        assert!(fns_and_vars.variables.get("bar_var").is_some());
    }
}
