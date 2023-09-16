use a1_notation::{Address, Column};
use crate::Expand;
use serde::{Deserialize, Serialize};
use super::{Ast, FunctionArgs, FunctionName};

/// The most basic building block of our language AST.  The AST is made recursive by the fact that
/// function calls and infix function calls can be composed.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Node {
    /// A wrapper around a `bool`, in spreadsheets it will come out as TRUE or FALSE
    Boolean(bool),

    /// A date, time or both
    DateTime(chrono::DateTime<chrono::Utc>),

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
    FunctionCall {
        args: Vec<Ast>,
        name: FunctionName,
    },

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
        name: FunctionName, 
        value: VariableValue,
    },
}

/// Variables can occur in 
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VariableValue {
    /// It's scoped to point at an absolute cell
    Absolute(Address),

    /// If a variable is defined in the code section it will have an AST as a value
    Ast(Ast),

    /// It's scoped as a column relative to an expand
    Relative {
        column: Column,
        scope: Expand,
    },
}

/// Most of these just make testing easier to not have to call .to_string() constantly, but they're
/// also nice for some of the code that the builtins call and need to build ASTs.
impl Node {
    #[cfg(test)]
    pub(crate) fn fn_def(name: &str, args: &[&str], body: Self) -> Self {
        Self::Function {
            name: name.to_string(),
            args: args.iter().map(|a| a.to_string()).collect(),
            body: Box::new(body),
        }
    }

    pub(crate) fn fn_call(name: &str, args: &[Self]) -> Self {
        Self::FunctionCall {
            name: name.to_string(),
            args: args.iter().map(|a| Box::new(a.to_owned())).collect(),
        }
    }

    pub(crate) fn infix_fn_call(left: Self, operator: &str, right: Self) -> Self {
        Self::InfixFunctionCall {
            left: Box::new(left),
            operator: operator.to_string(),
            right: Box::new(right),
        }
    }

    pub(crate) fn reference(r: &str) -> Self {
        Self::Reference(r.to_string())
    }

    pub(crate) fn text(t: &str) -> Self {
        Self::Text(t.to_string())
    }

    #[cfg(test)]
    pub(crate) fn var(name: &str, value: VariableValue) -> Self {
        Self::Variable {
            name: name.to_string(),
            value,
        }
    }
}
