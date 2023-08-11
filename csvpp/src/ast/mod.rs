//! # AST (abstract syntaX tree) Functions
//!
//! `Node` represents a building block of the parsed language, with a 
//!
use std::collections;
use crate::InnerResult;

mod node;
pub use node::Node;

/// traits that are implemented for `Node`
mod display;
mod eval;
mod from_str;

/// functionality related to ASTs
mod builtin_function;
mod builtin_variable;
mod references;

pub use builtin_function::BuiltinFunction;
pub use builtin_variable::BuiltinVariable;
pub(crate) use references::AstReferences;

type FunctionArgs = Vec<String>;
type FunctionName = String;
type VariableName = String;

pub type Functions = collections::HashMap<FunctionName, Ast>;
pub type Variables =  collections::HashMap<VariableName, Ast>;

pub type BuiltinFunctions = collections::HashMap<FunctionName, BuiltinFunction>;
pub type BuiltinVariables =  collections::HashMap<VariableName, BuiltinVariable>;

pub type Ast = Box<Node>;

pub type FunctionEval = Box<dyn Fn(&a1_notation::A1, &[Ast]) -> InnerResult<Node>>;
pub type VariableEval = Box<dyn Fn(&a1_notation::A1) -> InnerResult<Node>>;
