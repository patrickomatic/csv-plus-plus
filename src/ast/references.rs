use super::{Ast, Node};
use crate::Scope;
use std::collections;

type References = collections::HashSet<String>;
pub(super) type ReferencesIter = collections::hash_set::IntoIter<String>;

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct AstReferences {
    pub(super) functions: References,
    pub(super) variables: References,
}

impl AstReferences {
    pub(super) fn is_empty(&self) -> bool {
        self.functions.is_empty() && self.variables.is_empty()
    }

    pub(super) fn extract_dfs(&mut self, ast: &Ast, scope: &Scope) {
        match &**ast {
            // `FunctionCall`s might be user-defined but we always need to recurse on them
            Node::FunctionCall { name, args } => {
                if scope.functions.contains_key(name) {
                    self.functions.insert(name.to_string());
                }

                for arg in args {
                    self.extract_dfs(arg, scope);
                }
            }

            Node::Function { body, .. } => {
                self.extract_dfs(body, scope);
            }

            // `InfixFunctionCall`s can't be defined by the user but we need to recurse on the left and
            // right sides
            Node::InfixFunctionCall { left, right, .. } => {
                self.extract_dfs(left, scope);
                self.extract_dfs(right, scope);
            }

            // take any references corresponding do a defined variable
            Node::Reference(r) if scope.variables.contains_key(r) => {
                self.variables.insert(r.to_string());
            }

            // anything else is terminal
            _ => (),
        }
    }
}

impl Node {
    /// Does a depth first search on `ast` and parses out all identifiers that might be able to be
    /// eval()ed
    pub(super) fn extract_references(&self, scope: &Scope) -> AstReferences {
        let mut references = AstReferences::default();
        references.extract_dfs(&Ast::new(self.clone()), scope);
        references
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
    fn extract_references_fn_call_user_defined() {
        let mut module = build_module();
        module.scope.functions.insert(
            "foo".to_string(),
            Node::fn_def("foo", &["a", "b"], Node::reference("return value")).into(),
        );

        let references = Node::extract_references(
            &Node::fn_call("foo", &[Node::reference("bar"), Node::reference("baz")]),
            &module.scope,
        );

        assert!(references.functions.contains("foo"));
    }

    #[test]
    fn extract_references_infix_fn() {
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

        assert!(references.functions.contains("foo"));
    }

    #[test]
    fn extract_references_fn_calls_nested() {
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

        assert!(references.functions.contains("foo"));
    }

    #[test]
    fn extract_references_fn_def_user_defined() {
        let mut module = build_module();
        module.scope.variables.insert("bar".to_string(), 1.into());

        let references = Node::extract_references(
            &Node::fn_def("foo", &["A", "B"], Node::reference("bar")),
            &module.scope,
        );

        assert!(references.variables.contains("bar"));
    }

    #[test]
    fn extract_references_vars() {
        let mut module = build_module();
        module
            .scope
            .variables
            .insert("foo".to_string(), Node::reference("return value").into());

        let references = Node::extract_references(&Node::reference("foo"), &module.scope);

        assert!(references.variables.contains("foo"));
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

        assert!(references.variables.contains("bar"));
    }
}
