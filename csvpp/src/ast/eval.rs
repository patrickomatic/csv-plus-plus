use crate::{A1, Result};
use super::Node;


impl Node {
    pub fn eval_fn(&self, position: &A1, arguments: &[Node]) -> Result<Option<Node>> {
        Ok(match self {
            Self::Function { args, name, body } =>
                Some(todo!()),
            Self::BuiltinFunction { eval, .. } =>
                Some(eval(position, arguments)?),
            _ => None,
        })
    }

    pub fn eval_var(&self, position: &A1) -> Result<Option<Node>> {
        Ok(match self {
            Self::Variable { body, .. } => 
                Some(*body.clone()),
            Self::BuiltinVariable { eval, .. } => 
                Some(eval(position)?),
            _ => None,
        })
    }
}
