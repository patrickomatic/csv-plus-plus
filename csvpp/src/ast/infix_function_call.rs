//! # InfixFunctionCall
//!
//! A function call that has exactly two arguments - a left hand side and a right hand side.
//!
// use serde::{Deserialize, Serialize};
use std::fmt;

// #[derive(Debug, Deserialize, PartialEq, Serialize)]
#[derive(Debug)]
pub struct InfixFunctionCall {
    left_arg: Box<dyn super::Node>,
    right_arg: Box<dyn super::Node>,
    operator: super::FunctionName,
}

impl super::Node for InfixFunctionCall {}

impl fmt::Display for InfixFunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.left_arg, self.operator, self.right_arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn display() {
        assert_eq!("(1 * 2)", InfixFunctionCall {
            left_arg: Box::new(Integer(1)),
            right_arg: Box::new(Integer(2)),
            operator: "*".to_string(),
        }.to_string())
    }
}
