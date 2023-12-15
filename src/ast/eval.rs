//! # Eval
//!
//! The main functions for evaluating a function or variable.
//!
use super::{Ast, AstReferences, FunctionName, Node};
use crate::{EvalError, EvalResult, Scope};
use std::collections;

impl Ast {
    /// The idea here is just to keep looping as long as we are making progress eval()ing. Where
    /// progress means that `.extract_references()` returns a different, non-empty result each
    /// time.
    pub(crate) fn eval(
        self,
        scope: &Scope,
        position: Option<a1_notation::Address>,
    ) -> EvalResult<Ast> {
        let mut evaled_ast = self;
        let mut last_round_refs = AstReferences::default();

        loop {
            let refs = evaled_ast.extract_references(scope);
            if refs.is_empty() || refs == last_round_refs {
                break;
            }
            last_round_refs = refs.clone();

            evaled_ast = evaled_ast
                .eval_variables(evaled_ast.resolve_variables(scope, &refs.variables, position))
                .eval_functions(&refs.functions, scope)?;
        }

        Ok(evaled_ast)
    }

    /// Variables can all be resolved in one go - we just loop them by name and resolve the ones
    /// that we can and leave the rest alone.
    fn resolve_variables(
        &self,
        scope: &Scope,
        var_names: &[String],
        position: Option<a1_notation::Address>,
    ) -> collections::HashMap<String, Ast> {
        let mut resolved_vars = collections::HashMap::new();
        for var_name in var_names {
            if let Some(value) = scope.variables.get(var_name) {
                let value_from_var = match &**value {
                    Node::Variable { value, .. } => value.clone().into_ast(position),
                    n => n.clone().into(),
                };

                resolved_vars.insert(var_name.to_string(), value_from_var);
            }
        }

        resolved_vars
    }

    /// Evaluate the given `functions` calling `resolve_fn` upon each occurence to render a
    /// replacement.  Unlike variable resolution, we can't produce the values up front because the
    /// resolution function requires being called with the `arguments` at the call site.
    fn eval_functions(self, fns_to_resolve: &[FunctionName], scope: &Scope) -> EvalResult<Ast> {
        let mut evaled_ast = self;
        for fn_name in fns_to_resolve {
            if let Some(fn_ast) = scope.functions.get(fn_name) {
                evaled_ast = evaled_ast.call_function(fn_name, fn_ast)?;
            } else {
                // TODO: log a warning that we tried to resolve an unknown function
                // this is one of those things that should never happen (since `fns_to_resolve`
                // is only comprised of functions we know about)
            }
        }

        Ok(evaled_ast)
    }

    /// Use the mapping in `variable_values` to replace each variable referenced in the AST with
    /// it's given replacement.
    fn eval_variables(&self, variable_values: collections::HashMap<String, Ast>) -> Ast {
        let mut evaled_ast = self.clone();
        for (var_id, replacement) in variable_values {
            evaled_ast = evaled_ast.replace_variable(&var_id, replacement);
        }

        evaled_ast
    }

    /// Do a depth-first-search on the AST, "calling" the function wherever we see a
    /// `Node::FunctionCall` with the matching name.  Calling a function can result in two main
    /// paths:
    fn call_function(self, fn_id: &str, fn_ast: &Ast) -> EvalResult<Self> {
        let inner = self.into_inner();
        Ok(match inner {
            Node::FunctionCall { args, name } if name == fn_id => {
                match (*fn_ast).clone().into_inner() {
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
                    Node::Function {
                        args: resolved_args,
                        body,
                        ..
                    } => {
                        if args.len() != resolved_args.len() {
                            return Err(EvalError::new(
                                fn_ast.to_string(),
                                format!(
                                    "Expected {} arguments but received {}",
                                    args.len(),
                                    resolved_args.len()
                                ),
                            ));
                        }

                        let mut evaled_body = body;
                        for (i, resolved_arg) in resolved_args.iter().enumerate() {
                            evaled_body =
                                evaled_body.replace_variable(resolved_arg, args[i].clone());
                        }

                        evaled_body
                    }

                    // otherwise the function resolved to a non-function.  just treat that as
                    // terminal and return it.
                    node => node.into(),
                }
            }

            // it's a function call but not the one we're looking for - recurse through the
            // arguments
            Node::FunctionCall { args, name } => {
                let mut called_args = vec![];
                for arg in args {
                    called_args.push(arg.call_function(fn_id, fn_ast)?);
                }

                Node::fn_call(name, &called_args).into()
            }

            // also recurse for infix functions
            Node::InfixFunctionCall {
                left,
                operator,
                right,
            } => Node::infix_fn_call(
                left.call_function(fn_id, fn_ast)?,
                operator,
                right.call_function(fn_id, fn_ast)?,
            )
            .into(),

            // otherwise just don't modify it
            _ => inner.clone().into(),
        })
    }

    /// Depth-first-search replacing `Node::Reference`s of `var_id` with `replacement`.
    fn replace_variable(&self, var_id: &str, replacement: Ast) -> Self {
        let inner = (**self).clone();
        Ast::new(match inner {
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
            _ => inner,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn eval_unknown_function() {
        let ast = Ast::new(Node::fn_call("foo", &[Ast::from(1)]));
        assert_eq!(ast.clone().eval(&Scope::default(), None).unwrap(), ast);
    }

    #[test]
    fn eval_known_function() {
        let ast = Ast::new(Node::fn_call("foo", &[Ast::from(1), 2.into()]));
        let mut scope = Scope::default();
        scope.functions.insert(
            "foo".to_string(),
            Node::fn_def("foo", &["a", "b"], Ast::from(1)).into(),
        );

        assert_eq!(ast.clone().eval(&scope, None).unwrap(), 1.into());
    }

    #[test]
    fn eval_known_function_wrong_number_of_args() {
        let ast = Ast::new(Node::fn_call("foo", &[Ast::from(1)]));
        let mut scope = Scope::default();
        scope.functions.insert(
            "foo".to_string(),
            Node::fn_def("foo", &["a", "b"], Ast::from(1)).into(),
        );

        assert!(ast.clone().eval(&scope, None).is_err());
    }

    /*
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
    */
}
