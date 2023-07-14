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

    /// Do a depth-first-search on the AST, "calling" the function wherever we see a 
    /// `Node::FunctionCall` with the matching name.  Calling a function can result in two main 
    /// paths:
    ///
    /// * We get a `Node::Function` back.  This means we're calling a user defined function.  To
    /// handle this take each of the args, find the corresponding arg in the calling site and
    /// replace it in the `body`.  Then return that resulting body.
    ///
    /// * We get any other kind of `Node` back.  This will happen if a builtin function is called -
    /// in this case just replace the calling site with that value.
    ///
    /// Anything else in the AST just gets left alone and included in the final output as-is.
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
                    // terminal and return it.  typically when a builtin is called it will hit this
                    // path because they (at least most of them) return `Node::Reference`s
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

            // also recurse for infix functions
            Self::InfixFunctionCall { left, operator, right } => {
                Ok(Node::InfixFunctionCall {
                    left: Box::new(left.call_function(fn_id, resolve_fn)?),
                    operator: operator.clone(),
                    right: Box::new(right.call_function(fn_id, resolve_fn)?),
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

            Node::InfixFunctionCall { left, operator, right } =>
                Node::InfixFunctionCall { 
                    left: Box::new(left.replace_variable(var_id, replacement.clone())), 
                    operator: operator.clone(), 
                    right: Box::new(right.replace_variable(var_id, replacement.clone())),
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
    use super::*;

    #[test]
    fn eval_functions_nomatch() {
        let ast = Box::new(Node::Reference("foo".to_owned()));

        assert_eq!(
            ast, 
            Box::new(ast.eval_functions(vec!["bar".to_owned(), "baz".to_owned()], |_fn_id, _args| {
                Ok(Box::new(Node::Integer(42)))
            }).unwrap()));
    }

    #[test]
    fn eval_functions_builtin() {
        let ast = Box::new(Node::FunctionCall { 
            args: vec![],
            name: "foo_builtin".to_owned(),
        });

        assert_eq!(
            Box::new(Node::Integer(42)),
            Box::new(ast.eval_functions(vec!["foo_builtin".to_owned()], |_fn_id, _args| {
                Ok(Box::new(Node::Integer(42)))
            }).unwrap()));
    }

    #[test]
    fn eval_functions_user_defined() {
        let ast = Box::new(Node::FunctionCall { 
            args: vec![
                Box::new(Node::Integer(1)),
                Box::new(Node::Integer(2)),
            ],
            name: "my_func".to_owned(),
        });

        assert_eq!(
            Box::new(Node::InfixFunctionCall {
                left: Box::new(Node::Integer(1)),
                operator: "+".to_string(),
                right: Box::new(Node::Integer(2)),
            }),
            Box::new(ast.eval_functions(vec!["my_func".to_owned()], |_fn_id, _args| {
                Ok(Box::new(Node::Function {
                    name: "my_func".to_owned(),
                    args: vec!["a".to_string(), "b".to_string()],
                    body: Box::new(Node::InfixFunctionCall { 
                        left: Box::new(Node::Reference("a".to_string())), 
                        operator: "+".to_string(), 
                        right: Box::new(Node::Reference("b".to_string())), 
                    }),
                }))
            }).unwrap()));
    }

    #[test]
    fn eval_variables_nomatch() {
        let ast = Box::new(Node::Reference("foo".to_owned()));
        let mut values = collections::HashMap::new();
        values.insert("bar".to_string(), Box::new(Node::Integer(1)));

        assert_eq!(Box::new(ast.eval_variables(values).unwrap()), ast);
    }

    #[test]
    fn eval_variables_replaced() {
        let ast = Box::new(Node::Reference("foo".to_owned()));
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Box::new(Node::Integer(1)));

        assert_eq!(
            Box::new(ast.eval_variables(values).unwrap()),
            Box::new(Node::Integer(1)));
    }

    #[test]
    fn eval_variables_multiple() {
        let ast = Box::new(Node::FunctionCall {
            name: "my_func".to_owned(),
            args: vec![
                Box::new(Node::Reference("foo".to_owned())),
                Box::new(Node::Reference("bar".to_owned())),
            ],
        });
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Box::new(Node::Integer(1)));
        values.insert("bar".to_string(), Box::new(Node::Integer(2)));

        assert_eq!(
            Box::new(ast.eval_variables(values).unwrap()),
            Box::new(Node::FunctionCall {
                name: "my_func".to_owned(),
                args: vec![Box::new(Node::Integer(1)), Box::new(Node::Integer(2))],
            }));
    }

    #[test]
    fn eval_variables_nested_fn_call() {
        let ast = Box::new(Node::FunctionCall {
            name: "outer_func".to_owned(),
            args: vec![
                Box::new(Node::FunctionCall {
                    name: "my_func".to_owned(),
                    args: vec![
                        Box::new(Node::Reference("foo".to_owned())),
                        Box::new(Node::Reference("bar".to_owned())),
                    ],
                }),
            ],
        });
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Box::new(Node::Integer(1)));
        values.insert("bar".to_string(), Box::new(Node::Integer(2)));

        assert_eq!(
            Box::new(ast.eval_variables(values).unwrap()),
            Box::new(Node::FunctionCall {
                name: "outer_func".to_owned(),
                args: vec![
                    Box::new(Node::FunctionCall {
                        name: "my_func".to_owned(),
                        args: vec![Box::new(Node::Integer(1)), Box::new(Node::Integer(2))],
                    }),
                ],
            }));
    }

    #[test]
    fn eval_variables_nested_infix_fn_call() {
        let ast = Box::new(Node::InfixFunctionCall {
            left: Box::new(Node::FunctionCall {
                name: "my_func".to_owned(),
                args: vec![
                    Box::new(Node::Reference("foo".to_owned())),
                    Box::new(Node::Reference("bar".to_owned())),
                ],
            }),
            operator: "*".to_owned(),
            right: Box::new(Node::Integer(5)),
        });
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Box::new(Node::Integer(3)));
        values.insert("bar".to_string(), Box::new(Node::Integer(4)));

        assert_eq!(
            Box::new(ast.eval_variables(values).unwrap()),
            Box::new(Node::InfixFunctionCall {
                left: Box::new(Node::FunctionCall {
                    name: "my_func".to_owned(),
                    args: vec![Box::new(Node::Integer(3)), Box::new(Node::Integer(4))],
                }),
                operator: "*".to_owned(),
                right: Box::new(Node::Integer(5)),
            }));
    }
}
