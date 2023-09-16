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
use crate::{Error, Result, SourceCode};
use crate::ast::{Ast, Functions, Node, Variables, VariableValue};
use super::token_library::{Token, TokenMatch};
use super::ast_lexer::AstLexer;
use super::ast_parser::AstParser;

#[derive(Debug)]
pub struct CodeSection {
    pub functions: Functions,
    pub variables: Variables,
}

pub struct CodeSectionParser<'a> {
    lexer: AstLexer<'a>,
    source_code: &'a SourceCode,
}

/// A recursive descent parser which relies on `AstParser` for individual expressions.  As 
/// mentioned above, the contract here is that this parser handles parsing a series of 
/// function and variable references and delegates to `AstParser` for handling expressions
impl<'a> CodeSectionParser<'a> {
    pub fn parse(input: &'a str, source_code: &'a SourceCode) -> Result<CodeSection> {
        let lexer = AstLexer::new(input).map_err(|e| {
            Error::CodeSyntaxError { 
                message: e.to_string(), 
                line_number: e.line_number,
                position: e.position, 
                highlighted_lines: source_code.highlight_line(e.line_number, e.position),
            }
        })?;

        let parser = CodeSectionParser { lexer, source_code };
        
        parser.parse_code_section()
    }

    /// our entry point - just expects a series of variable and function definitions in any order
    fn parse_code_section(&'a self) -> Result<CodeSection> {
        let mut variables = HashMap::new();
        let mut functions = HashMap::new();

        loop {
            match self.lexer.next() {
                TokenMatch { token: Token::Eof, .. } => break,
                TokenMatch { token: Token::FunctionDefinition, .. } => {
                    let (fn_name, function) = self.parse_fn_definition()?;
                    functions.insert(fn_name, Box::new(function));
                },
                TokenMatch { token: Token::Reference, str_match: r, .. } => {
                    variables.insert(r.to_string(), 
                                     Box::new(Node::Variable {
                                         name: r.to_string(),
                                         value: VariableValue::Ast(self.parse_variable_assign()?),
                                     }));
                },
                token => {
                    return Err(self.token_match_to_error(
                            &token, 
                            format!("Expected an `fn` or variable definition (`:=`) operator but saw `{token}`")))
                },
            }
        }

        Ok(CodeSection { functions, variables })
    }

    /// parses a `:=` folloed by an `<expr>`
    fn parse_variable_assign(&'a self) -> Result<Ast> {
        // they better give us a :=
        match self.lexer.next() {
            TokenMatch { token: Token::VarAssign, .. } => {
                // consume an expression
                Ok(self.parse_expr()?)
            }
            token => 
                Err(self.token_match_to_error(
                        &token,
                        format!("Expected a variable definition operator (`:=`) but saw `{token}`"))),
        }
    }

    /// We're looking to parse a string of the form:
    ///
    /// ```ebnf
    /// 'fn' <name-ref> '(' { <arg-ref> ',' } ')' <expr>
    /// ```
    fn parse_fn_definition(&'a self) -> Result<(String, Node)> {
        // expect the function name (as a `Reference`)
        let name = match self.lexer.next() {
            TokenMatch { token: Token::Reference, str_match: r, .. } => r,
            token =>
                return Err(self.token_match_to_error(&token, format!("Expected a function name but saw `{token}`"))),
        };

        // expect a `(`
        match self.lexer.next() {
            TokenMatch { token: Token::OpenParen, .. } => (),
            token => 
                return Err(self.token_match_to_error(&token, format!("Expected `(` but saw `{token}`"))),
        };

        let mut args = vec![];

        // here we're looking for zero or more References representing the function arguments.
        // this is different than a `FunctionCall` where the arguments to the function can be
        // expressions themselves.
        loop {
            match self.lexer.next() {
                TokenMatch { token: Token::CloseParen, .. } => {
                    break
                },
                TokenMatch { token: Token::Comma, .. } => (),
                TokenMatch { token: Token::Reference, str_match: r, .. } => {
                    args.push(r.to_string());
                },
                t => 
                    return Err(self.token_match_to_error(&t, format!("Expected `(` but saw `{t}`"))),
            }
        }

        let function = Node::Function { 
            name: name.to_owned(), 
            args, 
            body: self.parse_expr()?,
        };

        Ok((name.to_owned(), function))
    }

    fn parse_expr(&'a self) -> Result<Ast> {
        // create an `AstParser` with a reference to our lexer so it can continue consuming our
        // stream of tokens
        AstParser::new(&self.lexer, Some(self.source_code)).expr_bp(true, 0)
    }

    fn token_match_to_error(&'a self, token: &TokenMatch, message: String) -> Error {
        Error::CodeSyntaxError {
            highlighted_lines: self.source_code.highlight_line(token.line_number, token.position),
            line_number: token.line_number,
            message,
            position: token.position,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path;
    use crate::ast::Ast;
    use super::*;

    fn test(input: &str) -> CodeSection {
        let source_code = SourceCode::new(input, path::PathBuf::from("foo.csvpp")).unwrap();
        CodeSectionParser::parse(input, &source_code).unwrap()
    }
    
    #[test]
    fn parse_function() {
        let fns_and_vars = test("fn foo(a, b) a + b");
        let foo = fns_and_vars.functions.get("foo").unwrap();

        let expected: Ast = Box::new(
            Node::fn_def("foo", &["a", "b"], 
                          Node::infix_fn_call(Node::reference("a"), "+", Node::reference("b"))));

        assert_eq!(foo, &expected);
    }

    #[test]
    fn parse_function_without_args() {
        let fns_and_vars = test("fn foo() 1 * 2");
        let foo = fns_and_vars.functions.get("foo").unwrap();

        let expected: Ast = Box::new(
            Node::fn_def(
                "foo", 
                &[], 
                Node::infix_fn_call(1.into(), "*", 2.into())));

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
