//! # Variable
//!
use std::any;
use std::fmt;

use crate::{A1, Result};
use super::{Ast, FunctionName, Node, NodeId, NodeWithId};

#[derive(Clone, Debug)]
pub struct Variable { 
    pub body: Ast,
    pub name: FunctionName, 
}

impl NodeWithId for Variable {
    fn id(&self) -> NodeId {
        self.name.clone()
    }
}

impl Node for Variable {
    fn as_any(&self) -> &dyn any::Any { self }

    fn eval_var(&self, _a1: &A1) -> Result<Option<Ast>> {
        Ok(Some(self.body.clone()))
    }

    fn node_eq(&self, other: &dyn any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, { |f| 
            self.name == f.name && self.body.node_eq(f.body.as_any())
        })
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} := {}", self.name, self.body)
    }
}

impl Variable {
    pub fn new(name: &str, body: Ast) -> Self {
        Variable { body, name: name.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn display() {
        let var = Variable::new("foo", Box::new(Integer(1)));

        assert_eq!("foo := 1", var.to_string());
    }

    #[test]
    fn node_eq() {

    }
}

