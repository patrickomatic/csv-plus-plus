use super::{Ast, FunctionName, Functions, Node, VariableName, Variables};
use crate::Compiler;

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
        compiler: &Compiler,
        module_fns: &Functions,
        module_vars: &Variables,
    ) -> AstReferences {
        let mut fns = vec![];
        let mut vars = vec![];

        extract_dfs(
            compiler,
            &Box::new(self.clone()),
            module_fns,
            module_vars,
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
    compiler: &Compiler,
    ast: &Ast,
    module_fns: &Functions,
    module_vars: &Variables,
    fns: &mut Vec<FunctionName>,
    vars: &mut Vec<VariableName>,
) {
    match &**ast {
        // `FunctionCall`s might be user-defined but we always need to recurse on them
        Node::FunctionCall { name, args } => {
            if compiler.is_function_defined(module_fns, name) {
                fns.push(name.to_string());
            }

            for arg in args {
                extract_dfs(compiler, arg, module_fns, module_vars, fns, vars);
            }
        }

        // `InfixFunctionCall`s can't be defined by the user but we need to recurse on the left and
        // right sides
        Node::InfixFunctionCall { left, right, .. } => {
            extract_dfs(compiler, left, module_fns, module_vars, fns, vars);
            extract_dfs(compiler, right, module_fns, module_vars, fns, vars);
        }

        // take any references corresponding do a defined variable
        Node::Reference(r) if compiler.is_variable_defined(module_vars, r) => {
            vars.push(r.to_string())
        }

        // anything else is terminal
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BuiltinFunction;
    use crate::test_utils::*;
    use crate::*;
    use crate::{Compiler, Spreadsheet};

    fn build_module() -> Module {
        Module::new(Spreadsheet::default(), None, ModuleName::new("foo"))
    }

    #[test]
    fn extract_references_empty() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler = test_file.into();
        let module = build_module();

        let references = Node::extract_references(
            &Box::new(5.into()),
            &compiler,
            &module.functions,
            &module.variables,
        );

        assert!(references.is_empty());
    }

    #[test]
    fn extract_references_fns_builtin() {
        let test_file = &TestSourceCode::new("csv", "");
        let mut compiler: Compiler = test_file.into();
        compiler.builtin_functions.insert(
            "foo".to_string(),
            BuiltinFunction {
                eval: Box::new(|_, _| Ok(Node::reference("return value"))),
                name: "foo".to_string(),
            },
        );
        let module = build_module();

        let references = Node::extract_references(
            &Box::new(Node::fn_call(
                "foo",
                &[Node::reference("bar"), Node::reference("baz")],
            )),
            &compiler,
            &module.functions,
            &module.variables,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_fns_user_defined() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler = test_file.into();
        let mut module = build_module();
        module.functions.insert(
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
            &compiler,
            &module.functions,
            &module.variables,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_fns_infix() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler = test_file.into();
        let mut module = build_module();
        module.functions.insert(
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
            &compiler,
            &module.functions,
            &module.variables,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_fns_nested() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler = test_file.into();
        let mut module = build_module();
        module.functions.insert(
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
            &compiler,
            &module.functions,
            &module.variables,
        );

        assert_eq!(references.functions.len(), 1);
        assert_eq!(&references.functions[0], "foo");
    }

    #[test]
    fn extract_references_vars() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler = test_file.into();
        let mut module = build_module();
        module
            .variables
            .insert("foo".to_string(), Box::new(Node::reference("return value")));

        let references = Node::extract_references(
            &Box::new(Node::reference("foo")),
            &compiler,
            &module.functions,
            &module.variables,
        );

        assert_eq!(references.variables.len(), 1);
        assert_eq!(&references.variables[0], "foo");
    }

    #[test]
    fn extract_references_vars_nested() {
        let test_file = &TestSourceCode::new("csv", "");
        let compiler = test_file.into();
        let mut module = build_module();
        module
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
            &compiler,
            &module.functions,
            &module.variables,
        );

        assert_eq!(references.variables.len(), 1);
        assert_eq!(&references.variables[0], "bar");
    }
}
