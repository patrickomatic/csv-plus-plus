//! # AST (abstract syntaX tree) Functions
//!
//! `Node` represents a building block of the parsed language, with a 
//!
use std::collections;
use crate::{A1, Result};

mod display;
// mod eval;
mod from_str;

mod builtin_function;
mod builtin_variable;
// mod references;

pub use builtin_function::BuiltinFunction;
pub use builtin_variable::BuiltinVariable;

type FunctionArgs = Vec<String>;
type FunctionName = String;
type VariableName = String;

pub type Functions = collections::HashMap<FunctionName, Ast>;
pub type Variables =  collections::HashMap<VariableName, Ast>;

pub type BuiltinFunctions = collections::HashMap<FunctionName, BuiltinFunction>;
pub type BuiltinVariables =  collections::HashMap<VariableName, BuiltinVariable>;

pub type Ast = Box<Node>;

pub type FunctionEval = Box<dyn Fn(&A1, &[Node]) -> Result<Node>>;
pub type VariableEval = Box<dyn Fn(&A1) -> Result<Node>>;

// TODO Serialize + Deserialize
#[derive(Clone, Debug, PartialEq)]
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

