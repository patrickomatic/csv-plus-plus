//! # AST (abstract syntaX tree) Functions
//!
//! `Node` represents a building block of the parsed language, with a
//!
use std::collections;

mod node;
pub(crate) use node::Node;

/// traits that are implemented for `Node`
mod display;
mod eval;
mod from;

/// functionality related to ASTs
mod references;
mod variable_value;

pub(crate) use references::AstReferences;
pub(crate) use variable_value::VariableValue;

pub(crate) type Ast = Box<Node>;

// TODO: make it a &[String]?
type FunctionArgs = Vec<String>;
type FunctionName = String;
type VariableName = String;

pub(crate) type Functions = collections::HashMap<FunctionName, Ast>;
pub(crate) type Variables = collections::HashMap<VariableName, Ast>;
