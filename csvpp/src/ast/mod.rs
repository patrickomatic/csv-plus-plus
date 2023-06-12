//! # AST (abstract syntaX tree) Functions
//!
//! `Node` represents a building block of the parsed language
use core::fmt::Debug;
use core::fmt::Display;
// use serde::{Serialize, Deserialize};
use std::collections::HashMap;

mod boolean;
mod date_time;
mod float;
mod function;
mod function_call;
mod infix_function_call;
mod integer;
mod reference;
mod runtime_value;
mod text;

pub use boolean::Boolean;
pub use date_time::DateTime;
pub use float::Float;
pub use function::Function;
pub use function_call::FunctionCall;
pub use infix_function_call::InfixFunctionCall;
pub use integer::Integer;
pub use reference::Reference;
pub use runtime_value::RuntimeValue;
pub use text::Text;

type FunctionArgs = Vec<String>;
type FunctionName = String;

// pub trait Node: Debug + Deserialize + Display + Serialize {
pub trait Node: Debug + Display {
    // TODO not sure yet how this will work
    // fn evaluate(position, variables) -> Cell;
}

pub fn from_key_value_args(_key_value_args: String) -> HashMap<String, Box<dyn Node>> {
    // TODO parse arg
    todo!()
}
