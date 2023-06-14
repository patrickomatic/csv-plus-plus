//! # Node
//!
//! `Node` represents a building block of the parsed language
/*
use core::fmt::Debug;
use core::fmt::Display;
// use serde::{Serialize, Deserialize};

// pub trait Node: Debug + Deserialize + Display + Serialize {
pub trait Node: Debug + Display {
    // TODO: not sure yet how evaluation will work
    // fn evaluate(position, variables) -> Cell;
    
    /// By overriding the default implementation, a `Node` can point to a `NodeWithId`
    fn id_ref(&self) -> Option<NodeId> {
        None
    }
}
*/
