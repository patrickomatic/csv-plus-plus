use super::{Ast, FunctionArgs, FunctionName, VariableName, VariableValue};
use crate::error::{BadInput, ParseResult};
use crate::parser::TokenInput;

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum NumberSign {
    Negative,
    Positive,
}

/// The most basic building block of our language AST.  The AST is made recursive by the fact that
/// function calls and infix function calls can be composed.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Node {
    /// A wrapper around a `bool`, in spreadsheets it will come out as TRUE or FALSE
    Boolean(bool),

    /// A date, time or both
    DateTime(crate::DateTime),

    /// A float (with a decimal) value
    Float {
        percentage: bool,
        sign: Option<NumberSign>,
        value: f64,
    },

    /// A function definition
    ///
    /// * `args` - The arguments that the function can take.  When called, each of the args will be
    ///     expanded in the `body`
    ///
    /// * `body` - An AST of any kind which can contain references to it's various `args`.  They'll
    ///     be resolved each time it's called (just as you'd expect for a function call)
    ///
    /// * `name` - The name you can call the function by.  Letters, numbers and underscore are
    ///     allowed.
    Function {
        args: FunctionArgs,
        body: Ast,
        name: FunctionName,
    },

    /// The calling of a function.  When calling a function each of the given args will be
    /// evaluated in turn, then interpolated into the `body` of the `Node::Function`.
    FunctionCall { args: Vec<Ast>, name: FunctionName },

    /// Like a `Node::FunctionCall` but it has two and only two params.  Think of `1 + 1`, `2 * 2`,
    /// etc.
    InfixFunctionCall {
        left: Ast,
        operator: FunctionName,
        right: Ast,
    },

    /// An integer
    Integer {
        percentage: bool,
        sign: Option<NumberSign>,
        value: i64,
    },

    /// Somewhat of a catch-all type - when parsing the source code we come across a string like
    /// "abc" which could either be a valid A1 reference or a reference to a variable.  If it's an
    /// A1 we want to leave it alone in the final result, if it's a variable we need to resolve it
    Reference(String),

    /// A string.
    Text(String),

    /// A variable definition.
    Variable {
        name: VariableName,
        value: VariableValue,
    },
}

fn text_unquote(tm: impl BadInput + TokenInput) -> ParseResult<String> {
    let t = tm.input();
    Ok(if t.starts_with('"') {
        let mut unquoted = String::new();
        let mut saw_start_quote = false;
        let mut last_was_quote = false;

        for c in t.chars() {
            if c == '"' {
                // two quotes in a row means it's quoted
                if last_was_quote {
                    unquoted.push(c);
                    last_was_quote = false;
                } else if !saw_start_quote {
                    saw_start_quote = true;
                } else {
                    last_was_quote = true;
                }
            } else {
                if last_was_quote {
                    return Err(tm.into_parse_error("Malformed double-quoted string"));
                }
                unquoted.push(c);
            }
        }

        if !last_was_quote {
            return Err(tm.into_parse_error("Unterminated double-quoted string"));
        }

        unquoted
    } else {
        t.to_string()
    })
}

/// Most of these just make testing easier to not have to call `.to_string()` constantly, but they're
/// also nice for some of the code that needs to build ASTs.
impl Node {
    #[cfg(test)]
    pub(crate) fn fn_def<N, A>(name: N, args: &[&str], body: A) -> Self
    where
        N: Into<String>,
        A: Into<Ast>,
    {
        Self::Function {
            name: name.into(),
            args: args.iter().map(ToString::to_string).collect(),
            body: body.into(),
        }
    }

    #[cfg(test)]
    pub(crate) fn fn_call<N, A>(name: N, args: &[A]) -> Self
    where
        N: Into<String>,
        A: Into<Ast> + Clone,
    {
        Self::FunctionCall {
            name: name.into(),
            args: args.iter().map(|a| (*a).clone().into()).collect(),
        }
    }

    #[cfg(test)]
    pub(crate) fn infix_fn_call<L, O, R>(left: L, operator: O, right: R) -> Self
    where
        L: Into<Ast>,
        O: Into<String>,
        R: Into<Ast>,
    {
        Self::InfixFunctionCall {
            left: left.into(),
            operator: operator.into(),
            right: right.into(),
        }
    }

    pub(crate) fn reference<S: Into<String>>(r: S) -> Self {
        Self::Reference(r.into())
    }

    pub(crate) fn parse_text(t: impl BadInput + TokenInput) -> ParseResult<Self> {
        Ok(Self::Text(text_unquote(t)?))
    }

    #[cfg(test)]
    pub(crate) fn text<S: Into<String>>(t: S) -> Self {
        Self::Text(t.into())
    }

    pub(crate) fn var<S: Into<String>>(name: S, value: VariableValue) -> Self {
        Self::Variable {
            name: name.into(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn parse_text_unquoted() {
        assert_eq!(
            Node::parse_text(build_ast_token_match("foo", build_source_code())).unwrap(),
            Node::Text("foo".to_string())
        );
    }

    #[test]
    fn parse_text_quoted() {
        let source_code = build_source_code();

        assert_eq!(
            Node::parse_text(build_ast_token_match("\"foo\"", source_code.clone())).unwrap(),
            Node::Text("foo".to_string())
        );
        assert_eq!(
            Node::parse_text(build_ast_token_match(
                "\"foo \"\"bar\"\"\"",
                source_code.clone()
            ))
            .unwrap(),
            Node::Text("foo \"bar\"".to_string())
        );
    }

    #[test]
    fn parse_text_error() {
        let source_code = build_source_code();
        assert!(Node::parse_text(build_ast_token_match("\"foo", source_code.clone())).is_err());
    }
}
