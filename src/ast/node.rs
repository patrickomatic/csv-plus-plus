use super::{Ast, FunctionArgs, FunctionName, VariableName, VariableValue};

/// The most basic building block of our language AST.  The AST is made recursive by the fact that
/// function calls and infix function calls can be composed.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Node {
    /// A wrapper around a `bool`, in spreadsheets it will come out as TRUE or FALSE
    Boolean(bool),

    /// A date, time or both
    DateTime(crate::DateTime),

    /// A float (with a decimal) value
    Float(f64),

    /// A function definition
    ///
    /// * `args` - The arguments that the function can take.  When called, each of the args will be
    /// expanded in the `body`
    ///
    /// * `body` - An AST of any kind which can contain references to it's various `args`.  They'll
    /// be resolved each time it's called (just as you'd expect for a function call)
    ///
    /// * `name` - The name you can call the function by.  Letters, numbers and underscore are
    /// allowed.
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
    Integer(i64),

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

/// Most of these just make testing easier to not have to call .to_string() constantly, but they're
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
            args: args.iter().map(|a| a.to_string()).collect(),
            body: body.into(),
        }
    }

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
