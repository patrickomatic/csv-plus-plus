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
use crate::module::ModuleName;
use crate::{Result, Runtime};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
pub struct CodeSection {
    pub(crate) functions: Functions,
    pub(crate) required_modules: Vec<ModuleName>,
    pub(crate) variables: Variables,
}

impl fmt::Display for CodeSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Required Modules")?;
        for m in &self.required_modules {
            writeln!(f, "{m}")?;
        }

        writeln!(f, "\n# Variables")?;
        for ast in self.variables.values() {
            writeln!(f, "{ast}")?;
        }

        writeln!(f, "\n# Functions")?;
        for ast in self.functions.values() {
            writeln!(f, "fn {ast}")?;
        }
        Ok(())
    }
}

pub(crate) struct CodeSectionParser<'a> {
    lexer: AstLexer<'a>,
    runtime: &'a Runtime,
}

/// A recursive descent parser which relies on `AstParser` for individual expressions.  As
/// mentioned above, the contract here is that this parser handles parsing a series of
/// function, use statements and variable assignments and delegates to `AstParser` for handling
/// expressions
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

    fn parse_code_section(&'a self) -> Result<CodeSection> {
        let mut variables = HashMap::new();
        let mut functions = HashMap::new();
        let mut required_modules = Vec::new();

        loop {
            let next = self.lexer.next();
            match next.token {
                Token::Eof => break,
                Token::FunctionDefinition => {
                    let (fn_name, function) = self.parse_fn_definition()?;
                    functions.insert(fn_name, Box::new(function));
                }
                Token::UseModule => {
                    required_modules.push(self.parse_use_module()?);
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
            required_modules,
            variables,
        })
    }

    /// parses `use` followed by a module name
    fn parse_use_module(&self) -> Result<ModuleName> {
        self.lexer.next().try_into()
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
        let runtime: Runtime = (&TestSourceCode::new("csv", input)).into();
        CodeSectionParser::parse(input, &runtime).unwrap()
    }

    #[test]
    fn parse_function() {
        let cs = test("fn foo(a, b) a + b");
        let foo = cs.functions.get("foo").unwrap();

        let expected: Ast = Box::new(Node::fn_def(
            "foo",
            &["a", "b"],
            Node::infix_fn_call(Node::reference("a"), "+", Node::reference("b")),
        ));

        assert_eq!(foo, &expected);
    }

    #[test]
    fn parse_function_without_args() {
        let cs = test("fn foo() 1 * 2");
        let foo = cs.functions.get("foo").unwrap();

        let expected: Ast = Box::new(Node::fn_def(
            "foo",
            &[],
            Node::infix_fn_call(1.into(), "*", 2.into()),
        ));

        assert_eq!(foo, &expected);
    }

    #[test]
    fn parse_multiple_functions() {
        let cs = test(
            r#"
fn foo()
    1 * 2
fn bar(a, b)
    a + b
"#,
        );

        assert_eq!(cs.functions.len(), 2);
    }

    #[test]
    fn parse_variables() {
        let cs = test("foo := \"bar\"");

        assert!(cs.variables.get("foo").is_some());
    }

    #[test]
    fn parse_variables_and_functions() {
        let cs = test(
            r#"
fn foo_fn() 1 * 2
foo_var := 3 * 4 + 5
fn bar_fn(a, b) a + b
bar_var := D1
"#,
        );

        assert!(cs.functions.get("foo_fn").is_some());
        assert!(cs.functions.get("bar_fn").is_some());

        assert!(cs.variables.get("foo_var").is_some());
        assert!(cs.variables.get("bar_var").is_some());
    }

    #[test]
    fn parse_use_module() {
        let cs = test(
            r#"
use foo
"#,
        );

        assert_eq!(cs.required_modules.len(), 1);
        assert_eq!(cs.required_modules[0].0, "foo");
    }

    #[test]
    fn parse_use_module_multiple() {
        let cs = test(
            r#"
use foo
use bar
"#,
        );

        assert_eq!(cs.required_modules.len(), 2);
        assert_eq!(cs.required_modules[0].0, "foo");
        assert_eq!(cs.required_modules[1].0, "bar");
    }
}
