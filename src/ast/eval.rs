//! # Eval
//!
//! The main functions for evaluating a function or variable.
//!
use super::{Ast, FunctionName, Functions, Node};
use std::collections;

impl Node {
    /// Evaluate the given `functions` calling `resolve_fn` upon each occurence to render a
    /// replacement.  Unlike variable resolution, we can't produce the values up front because the
    /// resolution function requires being called with the `arguments` at the call site.
    pub(crate) fn eval_functions(
        self,
        fns_to_resolve: &[FunctionName],
        functions: &Functions,
    ) -> Node {
        let mut evaled_ast = self;
        for fn_name in fns_to_resolve {
            if let Some(fn_ast) = functions.get(fn_name) {
                evaled_ast = evaled_ast.call_function(fn_name, fn_ast);
            } else {
                // TODO: log a warning that we tried to resolve an unknown function
                // this is one of those things that should never happen (since `fns_to_resolve`
                // is only comprised of functions we know about)
            }
        }

        evaled_ast
    }

    /// Use the mapping in `variable_values` to replace each variable referenced in the AST with
    /// it's given replacement.
    pub(crate) fn eval_variables(self, variable_values: collections::HashMap<String, Ast>) -> Node {
        let mut evaled_ast = self;
        for (var_id, replacement) in variable_values {
            evaled_ast = evaled_ast.replace_variable(&var_id, replacement);
        }

        evaled_ast
    }

    /// Do a depth-first-search on the AST, "calling" the function wherever we see a
    /// `Node::FunctionCall` with the matching name.  Calling a function can result in two main
    /// paths:
    fn call_function(&self, fn_id: &str, fn_ast: &Ast) -> Self {
        match self {
            Self::FunctionCall { args, name } if name == fn_id => {
                match (**fn_ast).clone() {
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
                    Self::Function {
                        args: resolved_args,
                        body,
                        ..
                    } => {
                        let mut evaled_body = body.into_inner();
                        for (i, resolved_arg) in resolved_args.iter().enumerate() {
                            evaled_body =
                                evaled_body.replace_variable(resolved_arg, args[i].clone());
                        }

                        evaled_body
                    }

                    // otherwise the function resolved to a non-function.  just treat that as
                    // terminal and return it.
                    node => node,
                }
            }

            // it's a function call but not the one we're looking for - recurse through the
            // arguments
            Self::FunctionCall { args, name } => {
                let mut called_args = vec![];
                for arg in args {
                    called_args.push(arg.call_function(fn_id, fn_ast));
                }

                Node::fn_call(name, &called_args)
            }

            // also recurse for infix functions
            Self::InfixFunctionCall {
                left,
                operator,
                right,
            } => Node::infix_fn_call(
                left.call_function(fn_id, fn_ast),
                operator,
                right.call_function(fn_id, fn_ast),
            ),

            // otherwise just don't modify it
            _ => self.clone(),
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
            }

            Node::InfixFunctionCall {
                left,
                operator,
                right,
            } => Node::infix_fn_call(
                left.replace_variable(var_id, replacement.clone()),
                operator,
                right.replace_variable(var_id, replacement.clone()),
            ),

            // a reference matching our variable - take the replacement
            Node::Reference(r) if var_id == r => replacement.into_inner(),

            // otherwise keep the Node unmodified
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::*;

    #[test]
    fn eval_functions_nomatch() {
        let ast = Ast::new(Node::reference("foo"));
        let functions = HashMap::new();

        assert_eq!(
            ast,
            Ast::new(
                ast.clone()
                    .into_inner()
                    .eval_functions(&["bar".to_owned(), "baz".to_owned()], &functions)
            )
        );
    }

    #[test]
    fn eval_functions_user_defined() {
        let ast = Ast::new(Node::fn_call("my_func", &[1.into(), 2.into()]));
        let mut functions = HashMap::new();
        functions.insert(
            "my_func".to_string(),
            Ast::new(Node::fn_def(
                "my_func",
                &["a", "b"],
                Node::infix_fn_call(Node::reference("a"), "+", Node::reference("b")),
            )),
        );

        assert_eq!(
            Ast::new(Node::infix_fn_call(1.into(), "+", 2.into())),
            Ast::new(
                ast.into_inner()
                    .eval_functions(&["my_func".to_owned()], &functions)
            )
        );
    }

    #[test]
    fn eval_variables_nomatch() {
        let ast = Ast::new(Node::reference("foo"));
        let mut values = collections::HashMap::new();
        values.insert("bar".to_string(), Ast::new(1.into()));

        assert_eq!(
            Ast::new(ast.clone().into_inner().eval_variables(values)),
            ast
        );
    }

    #[test]
    fn eval_variables_replaced() {
        let ast = Ast::new(Node::reference("foo"));
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Ast::new(1.into()));

        assert_eq!(
            Ast::new(ast.into_inner().eval_variables(values)),
            Ast::new(1.into())
        );
    }

    #[test]
    fn eval_variables_multiple() {
        let ast = Ast::new(Node::fn_call(
            "my_func",
            &[Node::reference("foo"), Node::reference("bar")],
        ));
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Ast::new(1.into()));
        values.insert("bar".to_string(), Ast::new(2.into()));

        assert_eq!(
            Ast::new(ast.into_inner().eval_variables(values)),
            Ast::new(Node::fn_call("my_func", &[1.into(), 2.into()]))
        );
    }

    #[test]
    fn eval_variables_nested_fn_call() {
        let ast = Ast::new(Node::fn_call(
            "outer_func",
            &[Node::fn_call(
                "my_func",
                &[Node::reference("foo"), Node::reference("bar")],
            )],
        ));
        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Ast::new(1.into()));
        values.insert("bar".to_string(), Ast::new(2.into()));

        assert_eq!(
            Ast::new(ast.into_inner().eval_variables(values)),
            Ast::new(Node::fn_call(
                "outer_func",
                &[Node::fn_call("my_func", &[1.into(), 2.into()])]
            ))
        );
    }

    #[test]
    fn eval_variables_nested_infix_fn_call() {
        let ast = Ast::new(Node::infix_fn_call(
            Node::fn_call("my_func", &[Node::reference("foo"), Node::reference("bar")]),
            "*",
            5.into(),
        ));

        let mut values = collections::HashMap::new();
        values.insert("foo".to_string(), Ast::new(3.into()));
        values.insert("bar".to_string(), Ast::new(4.into()));

        assert_eq!(
            Ast::new(ast.into_inner().eval_variables(values)),
            Ast::new(Node::infix_fn_call(
                Node::fn_call("my_func", &[3.into(), 4.into()]),
                "*",
                5.into()
            ))
        );
    }
}
