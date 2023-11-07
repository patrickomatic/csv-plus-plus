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
use super::ast_lexer::{AstLexer, Token, TokenMatch};
use super::ast_parser::AstParser;
use crate::ast::{Ast, Functions, Node, VariableValue, Variables};
use crate::{Result, Runtime};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeSection {
    pub(crate) functions: Functions,
    pub(crate) variables: Variables,
}

pub(crate) struct CodeSectionParser<'a> {
    lexer: AstLexer<'a>,
    runtime: &'a Runtime,
}

/// A recursive descent parser which relies on `AstParser` for individual expressions.  As
/// mentioned above, the contract here is that this parser handles parsing a series of
/// function and variable references and delegates to `AstParser` for handling expressions
impl<'a> CodeSectionParser<'a> {
    pub(crate) fn parse(input: &'a str, runtime: &'a Runtime) -> Result<CodeSection> {
        runtime.progress("Parsing code section");

        CodeSectionParser {
            lexer: AstLexer::new(input, runtime)
                .map_err(|e| runtime.source_code.code_syntax_error(e))?,
            runtime,
        }
        .parse_code_section()
    }

    /// our entry point - just expects a series of variable and function definitions in any order
    fn parse_code_section(&'a self) -> Result<CodeSection> {
        let mut variables = HashMap::new();
        let mut functions = HashMap::new();

        loop {
            let next = self.lexer.next();
            match next.token {
                Token::Eof => break,
                Token::FunctionDefinition => {
                    let (fn_name, function) = self.parse_fn_definition()?;
                    functions.insert(fn_name, Box::new(function));
                }
                Token::Reference => {
                    variables.insert(
                        next.str_match.to_string(),
                        Box::new(Node::var(
                            next.str_match,
                            VariableValue::Ast(self.parse_variable_assign()?),
                        )),
                    );
                }
                _ => {
                    return Err(
                        next.into_error("Expected an `fn` or variable definition (`:=`) operator")
                    )
                }
            }
        }

        Ok(CodeSection {
            functions,
            variables,
        })
    }

    /// parses a `:=` folloed by an `<expr>`
    fn parse_variable_assign(&'a self) -> Result<Ast> {
        let next = self.lexer.next();
        // they better give us a :=
        match next.token {
            // consume an expression
            Token::VarAssign => Ok(self.parse_expr()?),
            _ => Err(next.into_error("Expected a variable definition operator (`:=`)")),
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
            TokenMatch {
                token: Token::Reference,
                str_match: r,
                ..
            } => r,
            token => return Err(token.into_error("Expected a function name")),
        };

        // expect a `(`
        match self.lexer.next() {
            TokenMatch {
                token: Token::OpenParen,
                ..
            } => (),
            token => return Err(token.into_error("Expected `(`")),
        };

        let mut args = vec![];

        // here we're looking for zero or more References representing the function arguments.
        // this is different than a `FunctionCall` where the arguments to the function can be
        // expressions themselves.
        loop {
            let next = self.lexer.next();
            match next.token {
                Token::CloseParen => break,
                Token::Comma => (),
                Token::Reference => {
                    args.push(next.str_match.to_string());
                }
                _ => return Err(next.into_error("Expected `(`")),
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
        // create an `AstParser` with a reference to our lexer so it can continue consuming the
        // same stream of tokens
        AstParser::new(&self.lexer)
            .expr_bp(true, 0)
            .map_err(|e| self.runtime.source_code.code_syntax_error(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Ast;
    use crate::test_utils::*;

    fn test(input: &str) -> CodeSection {
        let runtime: Runtime = TestFile::new("csv", input).into();
        CodeSectionParser::parse(input, &runtime).unwrap()
    }

    #[test]
    fn parse_function() {
        let fns_and_vars = test("fn foo(a, b) a + b");
        let foo = fns_and_vars.functions.get("foo").unwrap();

        let expected: Ast = Box::new(Node::fn_def(
            "foo",
            &["a", "b"],
            Node::infix_fn_call(Node::reference("a"), "+", Node::reference("b")),
        ));

        assert_eq!(foo, &expected);
    }

    #[test]
    fn parse_function_without_args() {
        let fns_and_vars = test("fn foo() 1 * 2");
        let foo = fns_and_vars.functions.get("foo").unwrap();

        let expected: Ast = Box::new(Node::fn_def(
            "foo",
            &[],
            Node::infix_fn_call(1.into(), "*", 2.into()),
        ));

        assert_eq!(foo, &expected);
    }

    #[test]
    fn parse_multiple_functions() {
        let fns_and_vars = test(
            r#"
fn foo()
    1 * 2
fn bar(a, b)
    a + b
"#,
        );

        assert_eq!(fns_and_vars.functions.len(), 2);
    }

    #[test]
    fn parse_variables() {
        let fns_and_vars = test("foo := \"bar\"");

        assert!(fns_and_vars.variables.get("foo").is_some());
    }

    #[test]
    fn parse_variables_and_functions() {
        let fns_and_vars = test(
            r#"
fn foo_fn() 1 * 2
foo_var := 3 * 4 + 5
fn bar_fn(a, b) a + b
bar_var := D1
"#,
        );

        assert!(fns_and_vars.functions.get("foo_fn").is_some());
        assert!(fns_and_vars.functions.get("bar_fn").is_some());

        assert!(fns_and_vars.variables.get("foo_var").is_some());
        assert!(fns_and_vars.variables.get("bar_var").is_some());
    }
}
