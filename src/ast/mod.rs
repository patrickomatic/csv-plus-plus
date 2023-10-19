//! # AST (abstract syntaX tree) Functions
//!
//! `Node` represents a building block of the parsed language, with a
//!
use crate::error::EvalResult;
use std::collections;

mod node;
pub use node::{Node, VariableValue};

/// traits that are implemented for `Node`
mod display;
mod eval;
mod from;

/// functionality related to ASTs
mod builtin_function;
mod builtin_variable;
mod references;

pub(crate) use builtin_function::BuiltinFunction;
pub(crate) use builtin_variable::BuiltinVariable;
pub(crate) use references::AstReferences;

pub type Ast = Box<Node>;

// TODO: make it a &[String]?
type FunctionArgs = Vec<String>;
type FunctionName = String;
type VariableName = String;

pub(crate) type Functions = collections::HashMap<FunctionName, Ast>;
pub(crate) type Variables = collections::HashMap<VariableName, Ast>;

pub(crate) type BuiltinFunctions = collections::HashMap<FunctionName, BuiltinFunction>;
pub(crate) type BuiltinVariables = collections::HashMap<VariableName, BuiltinVariable>;

pub(crate) type FunctionEval = Box<dyn Fn(a1_notation::Address, &[Ast]) -> EvalResult<Node>>;
pub(crate) type VariableEval = Box<dyn Fn(a1_notation::Address) -> EvalResult<Node>>;
