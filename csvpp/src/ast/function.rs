//! # Function
//!
//! A definition of a function.  Note that this is distinctly different than the calling of a
//! function (`FunctionCall`.)
//!
// use serde::{Serialize, Deserialize};
use std::any;
use std::fmt;

// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub struct Function { 
    args: super::FunctionArgs,
    body: Box<dyn super::Node>,
    name: super::FunctionName, 
}

impl super::NodeWithId for Function {
    fn id(&self) -> super::NodeId {
        self.name.clone()
    }
}

impl super::Node for Function {
    fn eq(&self, other: &dyn any::Any) -> bool {
        if let Some(other_fn) = other.downcast_ref::<Function>() {
            return self.args == other_fn.args 
                && self.body.eq(&other_fn.body) 
                && self.name == other_fn.name
        }

        false
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}) {}", self.name, self.args.join(", "), self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn display() {
        let function = Function { 
            args: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            body: Box::new(Integer(1)),
            name: "foo".to_string(),
        };
        assert_eq!("foo(a, b, c) 1", function.to_string());
    }
}
