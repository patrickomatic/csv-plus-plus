//! # AST (abstract syntaX tree) Functions
//!
//! `Node` represents the building blocks of the parsed language
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;
use chrono;
use chrono::serde::ts_milliseconds;

type FunctionArgs = Vec<String>;
type FunctionName = String;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Node {
    Boolean(bool),

    #[serde(with = "ts_milliseconds")]
    DateTime(chrono::DateTime<chrono::Utc>),

    Float(f64),

    Function { 
        args: FunctionArgs,
        body: Box<Node>,
        name: FunctionName, 
    },

    FunctionCall {
        args: Vec<Node>,
        name: FunctionName,
    },

    InfixFunctionCall {
        left_arg: Box<Node>,
        right_arg: Box<Node>,
        operator: FunctionName,
    },

    Integer(i64),

    // XXX need to be able to init from a cell and row index
    Reference(String),

    // XXX map this to a static table with closures that do the logic?
    RuntimeValue(String),

    Text(String),
}

impl Node {
    pub fn from_key_value_args(_key_value_args: String) -> HashMap<String, Node> {
        // TODO parse arg
        todo!()
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Boolean(b) => 
                write!(f, "{}", if *b { "TRUE" } else { "FALSE" }),

            Node::DateTime(dt) => 
                write!(f, "{}", dt.to_string()),

            Node::Float(n) =>
                write!(f, "{}", n),

            Node::Function { args, body, name } =>
                write!(f, "{}({}) {}", name, args.join(", "), body),

            Node::FunctionCall { args, name } =>
                write!(f, "{}({})", name, args.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")),

            Node::InfixFunctionCall { left_arg, right_arg, operator } =>
                write!(f, "({} {} {})", left_arg, operator, right_arg),

            Node::Integer(n) =>
                write!(f, "{}", n),

            Node::Reference(r) =>
                write!(f, "{}", r),

            Node::RuntimeValue(_) =>
                write!(f, "(runtime value)"),

            Node::Text(t) =>
                write!(f, "\"{}\"", t),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_boolean() {
        assert_eq!("TRUE", Node::Boolean(true).to_string());

        assert_eq!("FALSE", Node::Boolean(false).to_string());
    }
    
    #[test]
    fn display_date() {
        let date_time = chrono::DateTime::from_utc(
            chrono::NaiveDate::from_ymd_opt(2022, 10, 12).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            chrono::Utc,
        );
        let date = Node::DateTime(date_time);
        assert_eq!("2022-10-12 00:00:00 UTC", date.to_string());
    }

    #[test]
    fn display_float() {
        assert_eq!("1.23", Node::Float(1.23).to_string());
    }

    #[test]
    fn display_function() {
        let function = Node::Function { 
            args: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            body: Box::new(Node::Integer(1)),
            name: "foo".to_string(),
        };
        assert_eq!("foo(a, b, c) 1", function.to_string());
    }

    #[test]
    fn display_function_call() {
        let function_call = Node::FunctionCall {
            args: vec![
                Node::Integer(1),
                Node::Text("foo".to_string()),
            ],
            name: "bar".to_string(),
        };
        assert_eq!("bar(1, \"foo\")", function_call.to_string());
    }

    #[test]
    fn display_infix_function_call() {
        let function_call = Node::InfixFunctionCall {
            left_arg: Box::new(Node::Integer(1)),
            right_arg: Box::new(Node::Text("foo".to_string())),
            operator: "*".to_string(),
        };
        assert_eq!("(1 * \"foo\")", function_call.to_string());
    }

    #[test]
    fn display_integer() {
        assert_eq!("1", Node::Integer(1).to_string());
    }

    #[test]
    fn display_reference() {
        assert_eq!("foo", Node::Reference("foo".to_string()).to_string());
    }

    #[test]
    fn display_text() {
        assert_eq!("\"foo\"", Node::Text("foo".to_string()).to_string());
    }
}
