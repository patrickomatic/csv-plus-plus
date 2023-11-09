use super::{Ast, FunctionName, Node, VariableName};
use crate::{Runtime, Template};

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
    pub(crate) fn extract_references(
        &self,
        runtime: &Runtime,
        template: &Template,
    ) -> AstReferences {
        let mut fns = vec![];
        let mut vars = vec![];

        extract_dfs(
            runtime,
            &Box::new(self.clone()),
            template,
            &mut fns,
            &mut vars,
        );

        AstReferences {
            functions: fns,
            variables: vars,
        }
    }
}

fn extract_dfs(
    runtime: &Runtime,
    ast: &Ast,
    template: &Template,
    fns: &mut Vec<FunctionName>,
    vars: &mut Vec<VariableName>,
) {
    match &**ast {
        // `FunctionCall`s might be user-defined but we always need to recurse on them
        Node::FunctionCall { name, args } => {
            if template.is_function_defined(runtime, name) {
                fns.push(name.to_string());
            }

            for arg in args {
                extract_dfs(runtime, arg, template, fns, vars);
            }
        }

        // `InfixFunctionCall`s can't be defined by the user but we need to recurse on the left and
        // right sides
        Node::InfixFunctionCall { left, right, .. } => {
            extract_dfs(runtime, left, template, fns, vars);
            extract_dfs(runtime, right, template, fns, vars);
        }

        // take any references corresponding do a defined variable
        Node::Reference(r) if template.is_variable_defined(runtime, r) => vars.push(r.to_string()),

        // anything else is terminal
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BuiltinFunction;
    use crate::test_utils::*;
    use crate::{Runtime, Spreadsheet};

    fn build_template(runtime: &Runtime) -> Template {
        Template::new(Spreadsheet::default(), None, runtime)
    }

    #[test]
    fn extract_references_empty() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();

        let references =
            Node::extract_references(&Box::new(5.into()), &runtime, &build_template(&runtime));

        assert!(references.is_empty());
    }

    #[test]
    fn extract_references_fns_builtin() {
        let test_file = TestFile::new("csv", "");
        let mut runtime: Runtime = test_file.into();
        runtime.builtin_functions.insert(
            "foo".to_string(),
            BuiltinFunction {
                eval: Box::new(|_, _| Ok(Node::reference("return value"))),
                name: "foo".to_string(),
            },
        );
        let template = build_template(&runtime);

        let references = Node::extract_references(
            &Box::new(Node::fn_call(
                "foo",
                &[Node::reference("bar"), Node::reference("baz")],
            )),
            &runtime,
            &template,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_fns_user_defined() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut template = build_template(&runtime);
        template.functions.insert(
            "foo".to_string(),
            Box::new(Node::fn_def(
                "foo",
                &["a", "b"],
                Node::reference("return value"),
            )),
        );

        let references = Node::extract_references(
            &Box::new(Node::fn_call(
                "foo",
                &[Node::reference("bar"), Node::reference("baz")],
            )),
            &runtime,
            &template,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_fns_infix() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut template = build_template(&runtime);
        template.functions.insert(
            "foo".to_string(),
            Box::new(Node::fn_def(
                "foo",
                &["a", "b"],
                Node::reference("return value"),
            )),
        );

        let references = Node::extract_references(
            &Box::new(Node::infix_fn_call(
                Node::fn_call("foo", &[Node::reference("bar"), Node::reference("baz")]),
                "+",
                Node::fn_call("bar", &[Node::reference("bar"), Node::reference("baz")]),
            )),
            &runtime,
            &template,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_fns_nested() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut template = build_template(&runtime);
        template.functions.insert(
            "foo".to_string(),
            Box::new(Node::fn_def(
                "foo",
                &["a", "b"],
                Node::reference("return value"),
            )),
        );

        let references = Node::extract_references(
            &Box::new(Node::fn_call(
                "foo_outer",
                &[Node::fn_call(
                    "foo",
                    &[Node::reference("bar"), Node::reference("baz")],
                )],
            )),
            &runtime,
            &template,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_vars() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut template = build_template(&runtime);
        template
            .variables
            .insert("foo".to_string(), Box::new(Node::reference("return value")));

        let references =
            Node::extract_references(&Box::new(Node::reference("foo")), &runtime, &template);

        assert_eq!(references.variables.len(), 1);
        assert_eq!(&references.variables[0], "foo");
    }

    #[test]
    fn extract_references_vars_nested() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let mut template = build_template(&runtime);
        template
            .variables
            .insert("bar".to_string(), Box::new(Node::reference("return value")));

        let references = Node::extract_references(
            &Box::new(Node::fn_call(
                "foo_outer",
                &[Node::fn_call(
                    "foo",
                    &[Node::reference("bar"), Node::reference("baz")],
                )],
            )),
            &runtime,
            &template,
        );

        assert_eq!(references.variables.len(), 1);
        assert_eq!(&references.variables[0], "bar");
    }
}
