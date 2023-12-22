use super::{Ast, FunctionName, Node, VariableName};
use crate::Scope;

// TODO: turn into HashSet
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct AstReferences {
    pub(crate) functions: Vec<FunctionName>,
    pub(crate) variables: Vec<VariableName>,
}

impl AstReferences {
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty() && self.variables.is_empty()
    }
}

impl Node {
    /// Does a depth first search on `ast` and parses out all identifiers that might be able to be
    /// eval()ed
    pub(crate) fn extract_references(&self, scope: &Scope) -> AstReferences {
        let mut fns = vec![];
        let mut vars = vec![];

        extract_dfs(&Ast::new(self.clone()), scope, &mut fns, &mut vars);

        AstReferences {
            functions: fns,
            variables: vars,
        }
    }
}

fn extract_dfs(
    ast: &Ast,
    scope: &Scope,
    acc_fns: &mut Vec<FunctionName>,
    acc_vars: &mut Vec<VariableName>,
) {
    match &**ast {
        // `FunctionCall`s might be user-defined but we always need to recurse on them
        Node::FunctionCall { name, args } => {
            if scope.functions.contains_key(name) {
                acc_fns.push(name.to_string());
            }

            for arg in args {
                extract_dfs(arg, scope, acc_fns, acc_vars);
            }
        }

        Node::Function { body, .. } => {
            extract_dfs(body, scope, acc_fns, acc_vars);
        }

        // `InfixFunctionCall`s can't be defined by the user but we need to recurse on the left and
        // right sides
        Node::InfixFunctionCall { left, right, .. } => {
            extract_dfs(left, scope, acc_fns, acc_vars);
            extract_dfs(right, scope, acc_fns, acc_vars);
        }

        // take any references corresponding do a defined variable
        Node::Reference(r) if scope.variables.contains_key(r) => acc_vars.push(r.to_string()),

        // anything else is terminal
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn extract_references_empty() {
        let module = build_module();
        let references = Node::extract_references(&Ast::new(5.into()), &module.scope);

        assert!(references.is_empty());
    }

    #[test]
    fn extract_references_fns_user_defined() {
        let mut module = build_module();
        module.scope.functions.insert(
            "foo".to_string(),
            Node::fn_def("foo", &["a", "b"], Node::reference("return value")).into(),
        );

        let references = Node::extract_references(
            &Node::fn_call("foo", &[Node::reference("bar"), Node::reference("baz")]),
            &module.scope,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_fns_infix() {
        let mut module = build_module();
        module.scope.functions.insert(
            "foo".to_string(),
            Node::fn_def("foo", &["a", "b"], Node::reference("return value")).into(),
        );

        let references = Node::extract_references(
            &Node::infix_fn_call(
                Node::fn_call("foo", &[Node::reference("bar"), Node::reference("baz")]),
                "+",
                Node::fn_call("bar", &[Node::reference("bar"), Node::reference("baz")]),
            ),
            &module.scope,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_fns_nested() {
        let mut module = build_module();
        module.scope.functions.insert(
            "foo".to_string(),
            Node::fn_def("foo", &["a", "b"], Node::reference("return value")).into(),
        );

        let references = Node::extract_references(
            &Node::fn_call(
                "foo_outer",
                &[Node::fn_call(
                    "foo",
                    &[Node::reference("bar"), Node::reference("baz")],
                )],
            ),
            &module.scope,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_vars() {
        let mut module = build_module();
        module
            .scope
            .variables
            .insert("foo".to_string(), Node::reference("return value").into());

        let references = Node::extract_references(&Node::reference("foo"), &module.scope);

        assert_eq!(references.variables.len(), 1);
        assert_eq!(&references.variables[0], "foo");
    }

    #[test]
    fn extract_references_vars_nested() {
        let mut module = build_module();
        module
            .scope
            .variables
            .insert("bar".to_string(), Node::reference("return value").into());

        let references = Node::extract_references(
            &Node::fn_call(
                "foo_outer",
                &[Node::fn_call(
                    "foo",
                    &[Node::reference("bar"), Node::reference("baz")],
                )],
            ),
            &module.scope,
        );

        assert_eq!(references.variables.len(), 1);
        assert_eq!(&references.variables[0], "bar");
    }
}
