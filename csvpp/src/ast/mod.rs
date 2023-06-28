//! # AST (abstract syntaX tree) Functions
//!
//! `Node` represents a building block of the parsed language, with a 
//!
use core::fmt::Debug;
use core::fmt::Display;
use std::any::Any;
use std::collections::HashMap;

mod boolean;
mod builtin_function;
mod builtin_variable;
mod date_time;
mod float;
mod function;
mod function_call;
mod infix_function_call;
mod integer;
mod reference;
mod text;
mod variable;

pub use boolean::Boolean;
pub use builtin_function::BuiltinFunction;
pub use builtin_variable::BuiltinVariable;
pub use date_time::DateTime;
pub use float::Float;
pub use function::Function;
pub use function_call::FunctionCall;
pub use infix_function_call::InfixFunctionCall;
pub use integer::Integer;
pub use reference::Reference;
pub use text::Text;
pub use variable::Variable;

use crate::{A1, Result};

pub type Ast = Box<dyn Node>;

type NodeId = String;

type FunctionArgs = Vec<String>;
type FunctionName = String;
type VariableName = String;

pub type Functions = HashMap<FunctionName, Ast>;
pub type Variables =  HashMap<VariableName, Ast>;

pub trait NodeWithId: Debug + Display {
    /// An `id` gives a function a `impl Node` a unique identifier.  For example a `Function`
    /// implements `NodeWithId` and `Reference` implements `id_ref()` in a way that can point to it
    fn id(&self) -> NodeId;
}

// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
trait NodeClone {
    fn clone_box(&self) -> Ast;
}

// TODO add Send + Sync?
// TODO add Serialize + Deserialize
pub trait Node: Any + Debug + Display + NodeClone {
    fn as_any(&self) -> &dyn Any;
    //
    // FunctionCalls and References do the calling (NodeWithId)
    //
    // Functions, Variables, BuiltinFunctions, BuiltinVariables 

    /// For `Node`s that want to be "called" like a function and replaced in the AST with this
    /// result.
    fn eval_fn(&self, _position: &A1, _arguments: &[Ast]) -> Result<Option<Ast>> {
        Ok(None)
    }

    /// For `Node`s that want to be "called" like a variable and replaced in the AST with this
    /// result.
    fn eval_var(&self, _position: &A1) -> Result<Option<Ast>> {
        Ok(None)
    }
    
    /// What allows one Node (a `FunctionCall` or a `Reference`) to point to another node.  Think
    /// about if a user is referencing a variable - in the AST this would appear as a `Reference`
    /// which has an `id_ref() -> Some(reference.name)`.
    fn id_ref(&self) -> Option<NodeId> {
        None
    }

    /// Each implementation needs to define this to define equivalency. They'll downcast to the
    /// concrete type and return `false` for anything else.
    fn node_eq(&self, other: &dyn Any) -> bool;
}

impl<T> NodeClone for T
where
    T: 'static + Node + Clone,
{
    fn clone_box(&self) -> Ast {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Ast {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn Node> {
    fn eq(&self, other: &Ast) -> bool {
        self.node_eq(other.as_any())
    }
}
