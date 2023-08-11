use serde::{Deserialize, Serialize};
use super::{Ast, FunctionArgs, FunctionName};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Node {
    Boolean(bool),
    DateTime(chrono::DateTime<chrono::Utc>),
    Float(f64),
    Function { 
        args: FunctionArgs,
        body: Ast,
        name: FunctionName, 
    },
    FunctionCall {
        args: Vec<Ast>,
        name: FunctionName,
    },
    InfixFunctionCall { 
        left: Ast,
        operator: FunctionName,
        right: Ast,
    },
    Integer(i64),
    Reference(String),
    Text(String),
    Variable {
        body: Ast,
        name: FunctionName, 
    },
}

/// Most of these just make testing easier to not have to call .to_string() constantly, but they're
/// also nice for some of the code that the builtins call.
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

    pub(crate) fn var(name: &str, body: Self) -> Self {
        Self::Variable {
            name: name.to_string(),
            body: Box::new(body),
        }
    }
}

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        Node::Boolean(value)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Node {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self::DateTime(value)
    }
}

impl From<f64> for Node {
    fn from(value: f64) -> Self {
        Node::Float(value)
    }
}

impl From<isize> for Node {
    fn from(value: isize) -> Self {
        Node::Integer(value as i64)
    }
}

impl From<i64> for Node {
    fn from(value: i64) -> Self {
        Node::Integer(value)
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Integer(value as i64)
    }
}

impl From<a1_notation::A1> for Node {
    fn from(value: a1_notation::A1) -> Self {
        Node::Reference(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn node_from_a1_notation() {
        let a1 = a1_notation::A1::from_str("A1").unwrap();

        assert_eq!(Node::reference("A1"), a1.into());
    }
}
