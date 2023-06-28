//! # Function
//!
//! A definition of a function.  Note that this is distinctly different than the calling of a
//! function (`FunctionCall`.)
//!
use std::any;
use std::fmt;
use super::{Ast, FunctionArgs, FunctionName, Node, NodeId, NodeWithId};

#[derive(Clone, Debug)]
pub struct Function { 
    pub args: FunctionArgs,
    pub body: Ast,
    pub name: FunctionName, 
}

impl NodeWithId for Function {
    fn id(&self) -> NodeId {
        self.name.clone()
    }
}

impl Node for Function {
    fn as_any(&self) -> &dyn any::Any { self }

    fn eval_fn(&self, _position: &crate::A1, _arguments: &[Ast]) -> crate::Result<Option<Ast>> {
        todo!()
    }

    fn node_eq(&self, other: &dyn any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, { |f| 
            self.name == f.name
                && self.args == f.args
                && self.body.node_eq(f.body.as_any())
        })
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}) {}", self.name, self.args.join(", "), self.body)
    }
}

impl Function {
    pub fn new(name: &str, args: FunctionArgs, body: Ast) -> Self {
        Function { 
            args, 
            body, 
            name: name.to_string(),
        }
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
