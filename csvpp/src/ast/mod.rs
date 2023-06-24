//! # AST (abstract syntaX tree) Functions
//!
//! `Node` represents a building block of the parsed language, with a 
//!
use core::fmt::Debug;
use core::fmt::Display;
// use serde::{Serialize, Deserialize};
use std::any::Any;

mod boolean;
mod date_time;
mod float;
mod function;
mod function_call;
mod infix_function_call;
mod integer;
mod reference;
mod text;

pub use boolean::Boolean;
pub use date_time::DateTime;
pub use float::Float;
pub use function::Function;
pub use function_call::FunctionCall;
pub use infix_function_call::InfixFunctionCall;
pub use integer::Integer;
pub use reference::Reference;
pub use text::Text;

type NodeId = String;

type FunctionArgs = Vec<String>;
type FunctionName = String;

pub trait NodeWithId: Debug + Display {
    /// An `id` gives a function a `impl Node` a unique identifier.  For example a `Function`
    /// implements `NodeWithId` and `Reference` implements `id_ref()` in a way that can point to it
    fn id(&self) -> NodeId;
}

// TODO add Send + Sync?
// TODO add Serialize + Deserialize
pub trait Node: Any + Debug + Display {
    fn as_any(&self) -> &dyn Any;

    // TODO not sure yet how evaluation will work
    // fn evaluate(position, variables) -> Cell;
    
    /// What allows one Node (a `FunctionCall` or a `Reference`) to point to another node.  Think
    /// about if a user is referencing a variable - in the AST this would appear as a `Reference`
    /// which has an `id_ref() -> Some(reference.name)`.
    fn id_ref(&self) -> Option<NodeId> {
        None
    }

    fn node_eq(&self, other: &dyn Any) -> bool;
}

impl PartialEq for Box<dyn Node> {
    fn eq(&self, other: &Box<dyn Node>) -> bool {
        self.node_eq(other.as_any())
    }
}
