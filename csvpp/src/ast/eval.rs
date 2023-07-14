//! # Eval
//!
//! The main functions for evaluating a function or variable.
//!
use std::collections;
use crate::Result;
use super::{Ast, Node};

impl Node {
    /// Evaluate the given `functions` calling `resolve_fn` upon each occurence to render a
    /// replacement.  Unlike variable resolution, we can't produce the values up front because the
    /// resolution function requires being called with the `arguments` at the call site.
    ///
    pub fn eval_functions(
        &self,
        functions: Vec<String>,
        resolve_fn: impl Fn(&str, Vec<Ast>) -> Result<Ast>
    ) -> Result<Node> {
        let mut evaled_ast = self.clone();
        for fn_name in functions {
            evaled_ast = evaled_ast.call_function(&fn_name, &resolve_fn)?;
        }

        Ok(evaled_ast)
    }

    /// Use the mapping in `variable_values` to replace each variable referenced in the AST with
    /// it's given replacement.
    pub fn eval_variables(&self, variable_values: collections::HashMap<String, Ast>) -> Result<Node> {
        let mut evaled_ast = self.clone();
        for (var_id, replacement) in variable_values {
            evaled_ast = evaled_ast.replace_variable(&var_id, replacement);
        }

        Ok(evaled_ast)
    }

    fn call_function(
        &self,
        fn_id: &str,
        resolve_fn: &impl Fn(&str, Vec<Ast>) -> Result<Ast>
    ) -> Result<Self> {
        match self {
            // handle a function that we're calling
            Self::FunctionCall { args, name } if name == fn_id =>
                // call the resolve function which will either give us the result of a builtin
                // (typically a terminal node) or a user-defined function (always a
                // `Node::Function`)
                match *resolve_fn(fn_id, args.to_vec())? {
                    // when we get a `Node::Function`, take the body and replace each of it's
                    // arguments in the body.  For example:
                    //
                    // fn foo(a, b) a + b
                    //
                    // called as:
                    //
                    // foo(1, 2)
                    //
                    // will evaluate to:
                    //
                    // (1 + 2)
                    Self::Function { args: resolved_args, body, .. } => {
                        let mut evaled_body = *body;
                        for (i, resolved_arg) in resolved_args.iter().enumerate() {
                            evaled_body = evaled_body.replace_variable(resolved_arg, args[i].clone());
                        }

                        Ok(evaled_body)
                    }

                    // otherwise the function resolved to a non-function.  just treat that as
                    // terminal and return it
                    node => Ok(node),
                },

            // it's a function call but not the one we're looking for - recurse through the
            // arguments
            Self::FunctionCall { args, name } => {
                let mut called_args = vec![];
                for arg in args {
                    called_args.push(Box::new(arg.call_function(fn_id, resolve_fn)?));
                }

                Ok(Node::FunctionCall {
                    args: called_args,
                    name: name.to_string(),
                })
            },

            // otherwise just don't modify it
            _ => Ok(self.clone()),
        }
    }

    /// Depth-first-search replacing `Node::Reference`s of `var_id` with `replacement`.
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

            // a reference matching our variable - take the replacement
            Node::Reference(r) if var_id == r => *replacement,

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
        // TODO
    }

    #[test]
    fn eval_variables() {
        // TODO
    }
}
