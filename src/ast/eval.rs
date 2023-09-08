//! # Eval
//!
//! The main functions for evaluating a function or variable.
//!
use std::collections;
use crate::InnerResult;
use super::{Ast, Node};

impl Node {
    /// Evaluate the given `functions` calling `resolve_fn` upon each occurence to render a
    /// replacement.  Unlike variable resolution, we can't produce the values up front because the
    /// resolution function requires being called with the `arguments` at the call site.
    pub fn eval_functions(
        &self,
        functions: &[String],
        resolve_fn: impl Fn(&str, Vec<Ast>) -> InnerResult<Ast>
    ) -> InnerResult<Node> {
        let mut evaled_ast = self.clone();
        for fn_name in functions {
            evaled_ast = evaled_ast.call_function(fn_name, &resolve_fn)?;
        }

        Ok(evaled_ast)
    }

    /// Use the mapping in `variable_values` to replace each variable referenced in the AST with
    /// it's given replacement.
    pub fn eval_variables(
        &self,
        variable_values: collections::HashMap<String, Ast>,
    ) -> InnerResult<Node> {
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
        resolve_fn: &impl Fn(&str, Vec<Ast>) -> InnerResult<Ast>
    ) -> InnerResult<Self> {
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
                    called_args.push(arg.call_function(fn_id, resolve_fn)?);
                }

                Ok(Node::fn_call(name, &called_args))
            },

            // also recurse for infix functions
            Self::InfixFunctionCall { left, operator, right } => {
                Ok(Node::infix_fn_call(
                        left.call_function(fn_id, resolve_fn)?,
                        operator,
                        right.call_function(fn_id, resolve_fn)?))
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
                    replaced_args.push(arg.replace_variable(var_id, replacement.clone()));
                }

                Node::fn_call(name, &replaced_args)
            },

            Node::InfixFunctionCall { left, operator, right } =>
                Node::infix_fn_call(
                    left.replace_variable(var_id, replacement.clone()), 
                    operator, 
                    right.replace_variable(var_id, replacement.clone())),

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
        let ast = Box::new(Node::reference("foo"));

        assert_eq!(
            ast, 
            Box::new(ast.eval_functions(&["bar".to_owned(), "baz".to_owned()], |_fn_id, _args| {
                Ok(Box::new(42.into()))
            }).unwrap()));
    }

    #[test]
    fn eval_functions_builtin() {
        let ast = Box::new(Node::fn_call("foo_builtin", &[]));

        assert_eq!(
            Box::new(42.into()),
            Box::new(ast.eval_functions(&["foo_builtin".to_owned()], |_fn_id, _args| {
                Ok(Box::new(42.into()))
            }).unwrap()));
    }

    #[test]
    fn eval_functions_user_defined() {
        let ast = Box::new(Node::fn_call("my_func", &[1.into(), 2.into()]));

        assert_eq!(
            Box::new(Node::infix_fn_call(1.into(), "+", 2.into())),

            Box::new(ast.eval_functions(&["my_func".to_owned()], |_fn_id, _args| {
                Ok(Box::new(Node::fn_def("my_func", &["a", "b"],
                    Node::infix_fn_call(Node::reference("a"), "+", Node::reference("b")))))
            }).unwrap()));
    }

    #[test]
    fn eval_variables_nomatch() {
        let ast = Box::new(Node::reference("foo"));
        let mut values = collections::HashMap::new();
        values.insert("bar".to_string(), Box::new(1.into()));

        assert_eq!(Box::new(ast.eval_variables(values).unwrap()), ast);
    }

    #[test]
    fn eval_variables_replaced() {
        let ast = Box::new(Node::reference("foo"));
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Box::new(1.into()));

        assert_eq!(
            Box::new(ast.eval_variables(values).unwrap()),
            Box::new(1.into()));
    }

    #[test]
    fn eval_variables_multiple() {
        let ast = Box::new(Node::fn_call(
            "my_func",
            &[Node::reference("foo"), Node::reference("bar")],
        ));
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Box::new(1.into()));
        values.insert("bar".to_string(), Box::new(2.into()));

        assert_eq!(
            Box::new(ast.eval_variables(values).unwrap()),
            Box::new(Node::fn_call("my_func", &[1.into(), 2.into()])));
    }

    #[test]
    fn eval_variables_nested_fn_call() {
        let ast = Box::new(
            Node::fn_call("outer_func",
                          &[Node::fn_call("my_func", 
                                              &[Node::reference("foo"), Node::reference("bar")])],
        ));
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Box::new(1.into()));
        values.insert("bar".to_string(), Box::new(2.into()));

        assert_eq!(
            Box::new(ast.eval_variables(values).unwrap()),
            Box::new(Node::fn_call("outer_func", &[
                    Node::fn_call("my_func", &[1.into(), 2.into()])])));
    }

    #[test]
    fn eval_variables_nested_infix_fn_call() {
        let ast = Box::new(Node::infix_fn_call(
            Node::fn_call(
                "my_func",
                &[Node::reference("foo"), Node::reference("bar")],
            ),
            "*",
            5.into()));

        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Box::new(3.into()));
        values.insert("bar".to_string(), Box::new(4.into()));

        assert_eq!(
            Box::new(ast.eval_variables(values).unwrap()),
            Box::new(Node::infix_fn_call(
                Node::fn_call("my_func", &[3.into(), 4.into()]),
                "*",
                5.into())));
    }
}
