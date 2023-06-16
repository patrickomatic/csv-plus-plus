//! # Function Call
//!
//! The calling of a function.  This is the branching point of our AST and we don't evaluate
//! function calls until we're ready to write to the target format.
//!
// use serde::{Deserialize, Serialize};
use std::any;
use std::fmt;

// #[derive(Debug, Deserialize, PartialEq, Serialize)]
#[derive(Debug)]
pub struct FunctionCall {
    pub args: Vec<Box<dyn super::Node>>,
    pub name: super::FunctionName,
}

impl super::Node for FunctionCall {
    fn id_ref(&self) -> Option<super::NodeId> {
        Some(self.name.clone())
    }

    fn eq(&self, other: &dyn any::Any) -> bool {
        if let Some(other_fn) = other.downcast_ref::<FunctionCall>() {
            if self.name != other_fn.name || self.args.len() != other_fn.args.len() {
                return false
            }

            for (i, arg) in self.args.iter().enumerate() {
                if !arg.eq(&other_fn.args[i]) {
                    return false
                }
            }
            
            return true
        }

        false
    }
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let args_to_string = self.args
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{}({})", self.name, args_to_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn display() {
        let function_call = FunctionCall {
            args: vec![Box::new(Integer(1)), Box::new(Text("foo".to_string()))],
            name: "bar".to_string(),
        };
        assert_eq!("bar(1, \"foo\")", function_call.to_string());
    }
}
