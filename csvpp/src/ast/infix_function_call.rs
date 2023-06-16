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
    pub left_arg: Box<dyn Node>,
    pub operator: FunctionName,
    pub right_arg: Box<dyn Node>,
}

impl NodeWithId for InfixFunctionCall {
    fn id(&self) -> super::NodeId {
        self.operator.clone()
    }
}

impl Node for InfixFunctionCall {
    fn eq(&self, other: &dyn Any) -> bool {
        if let Some(other_fn) = other.downcast_ref::<InfixFunctionCall>() {
            return self.left_arg.eq(&other_fn.left_arg)
                && self.operator == other_fn.operator
                && self.right_arg.eq(&other_fn.right_arg)
        }

        false
    }
}

impl fmt::Display for InfixFunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.left_arg, self.operator, self.right_arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    fn test_infix_fn() -> InfixFunctionCall {
        InfixFunctionCall { 
            left_arg: Box::new(Integer(1)),
            right_arg: Box::new(Integer(2)),
            operator: "*".to_string(),
        }
    }

    #[test]
    fn display() {
        assert_eq!("(1 * 2)", &test_infix_fn().to_string());
    }

    #[test]
    fn eq_true() {
        assert!(Node::eq(&test_infix_fn(), &test_infix_fn()));
    }

    #[test]
    fn eq_false() {
        assert!(!Node::eq(&test_infix_fn(), &InfixFunctionCall {
            left_arg: Box::new(Integer(1)),
            right_arg: Box::new(Integer(2)),
            operator: "+".to_string(),
        }));

        assert!(!Node::eq(&test_infix_fn(), &Integer(5)))
    }
}
