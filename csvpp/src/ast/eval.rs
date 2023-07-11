use std::collections;
use crate::{A1, Result};
use super::{Ast, Node};

impl Node {
    pub fn eval_functions(&self, _functions: Vec<String>, _position: &A1) -> Result<Node> {
        /*
        Ok(match self {
            Self::Function { args, name, body } =>
                Some(todo!()),
            Self::BuiltinFunction { eval, .. } =>
                Some(eval(position, arguments)?),
            _ => None,
        })
        */

        Ok(self.clone())
    }

    pub fn replace_function(&self, _fn_id: &str, _replacement: Node) -> Self {
        todo!()
    }

    pub fn eval_variables(&self, variable_values: collections::HashMap<String, Ast>) -> Result<Node> {
        let mut evaled_ast = self.clone();
        for (var_id, replacement) in variable_values {
            evaled_ast = evaled_ast.replace_variable(&var_id, replacement);
        }

        Ok(evaled_ast)
    }

    fn replace_variable(&self, var_id: &str, replacement: Ast) -> Self {
        match self {
            Node::FunctionCall { args, name } => {
                // recursively call for each arg to a function
                let mut replaced_args = vec![];
                for arg in args {
                    replaced_args.push(Box::new(arg.replace_variable(var_id, replacement.clone())));
                }

                Node::FunctionCall {
                    args: replaced_args,
                    name: name.to_string(),
                }
            },

            // a reference matching our variable - replace it
            Node::Reference(r) if var_id == r => *replacement.clone(),

            // otherwise keep the Node unmodified
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn eval_functions() {
    }

    #[test]
    fn eval_variables() {
    }
}
