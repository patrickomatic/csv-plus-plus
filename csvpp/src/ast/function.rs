//! # Function
//!
//! A definition of a function.  Note that this is distinctly different than the calling of a
//! function (`FunctionCall`.)
//!
// use serde::{Serialize, Deserialize};
use std::any;
use std::fmt;
use super::{FunctionArgs, FunctionName, Node, NodeId, NodeWithId};

// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub struct Function { 
    pub args: FunctionArgs,
    pub body: Box<dyn Node>,
    pub name: FunctionName, 
}

impl NodeWithId for Function {
    fn id(&self) -> NodeId {
        self.name.clone()
    }
}

impl Node for Function {
    fn as_any(&self) -> &dyn any::Any { self }

    fn node_eq(&self, other: &dyn any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, { |f| 
            self.name == f.name
                && self.args == f.args // XXX we need to loop through each one?
                && self.body.node_eq(f.body.as_any())
        })

        /*
        if let Some(other_fn) = other.downcast_ref::<Function>() {
            return self.args == other_fn.args 
                && self.body.eq(&other_fn.body) 
                && self.name == other_fn.name
        } else {
            false
        }
        */
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}) {}", self.name, self.args.join(", "), self.body)
    }
}

impl Function {
    pub fn new(name: &str, args: FunctionArgs, body: Box<dyn Node>) -> Self {
        Function { args, body, name: name.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn display() {
        let function = Function::new(
            "foo",
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            Box::new(Integer(1)),
        );

        assert_eq!("foo(a, b, c) 1", function.to_string());
    }

    #[test]
    fn node_eq() {

    }
}
