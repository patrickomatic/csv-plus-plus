//! # InfixFunctionCall
//!
//! A function call that has exactly two arguments - a left hand side and a right hand side.
//!
// use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt;

use crate::Node;
use super::{FunctionName, NodeWithId};

// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub struct InfixFunctionCall {
    pub left: Box<dyn Node>,
    pub operator: FunctionName,
    pub right: Box<dyn Node>,
}

impl NodeWithId for InfixFunctionCall {
    fn id(&self) -> super::NodeId {
        self.operator.clone()
    }
}

impl Node for InfixFunctionCall {
    fn as_any(&self) -> &dyn Any { self }

    fn node_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |f| {
            self.operator == f.operator
                && self.left.node_eq(f.left.as_any())
                && self.right.node_eq(f.right.as_any())
        })
    }
}

impl fmt::Display for InfixFunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl InfixFunctionCall {
    pub fn new(left: Box<dyn Node>, operator: &str, right: Box<dyn Node>) -> InfixFunctionCall {
        InfixFunctionCall { left, operator: operator.to_string(), right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    fn test_infix_fn() -> InfixFunctionCall {
        InfixFunctionCall::new(Box::new(Integer(1)), "*", Box::new(Integer(2)))
    }

    #[test]
    fn display() {
        assert_eq!("(1 * 2)", &test_infix_fn().to_string());
    }

    #[test]
    fn node_eq_true() {
        assert!(Node::node_eq(&test_infix_fn(), &test_infix_fn()));
    }

    #[test]
    fn node_eq_false() {
        // different kind of Node
        assert!(!Node::node_eq(&test_infix_fn(), &Integer(5)));

        // different operator
        assert!(!Node::node_eq(&test_infix_fn(), 
                               &InfixFunctionCall::new(Box::new(Integer(1)), "+", Box::new(Integer(2)))));

        // left and right are switched
        assert!(!Node::node_eq(&test_infix_fn(), 
                               &InfixFunctionCall::new(Box::new(Integer(2)), "*", Box::new(Integer(1)))))
    }
}
