use crate::Template;
use super::{Ast, FunctionName, Node, VariableName};

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
    pub(crate) fn extract_references(&self, template: &Template) -> AstReferences {
        let mut fns = vec![];
        let mut vars = vec![];

        extract_dfs(&Box::new(self.clone()), template, &mut fns, &mut vars);

        AstReferences { functions: fns, variables: vars }
    }
}

fn extract_dfs(
    ast: &Ast, 
    template: &Template, 
    fns: &mut Vec<FunctionName>, 
    vars: &mut Vec<VariableName>,
) {
    match &**ast {
        // `FunctionCall`s are what make our AST tree-like.  Each function call can have many
        // arguments each of which is an AST
        Node::FunctionCall { name, args } if template.is_function_defined(name) => {
            fns.push(name.to_string());

            for arg in args.iter() {
                extract_dfs(arg, template, fns, vars);
            }
        },

        // take any references corresponding do a defined variable
        Node::Reference(r) if template.is_variable_defined(r) => 
            vars.push(r.to_string()),

        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Runtime, Spreadsheet};
    use crate::test_utils::*;
    use super::*;

    fn build_template(runtime: &Runtime) -> Template {
        Template::new(Spreadsheet::default(), None, runtime)
    }

    #[test]
    fn extract_references_empty() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();

        let references = Node::extract_references(
            &Box::new(Node::Integer(5)), 
            &build_template(&runtime));

        assert!(references.is_empty());
    }

    #[test]
    fn extract_references_functions() {
        // TODO
    }

    #[test]
    fn extract_references_variables() {
        // TODO
    }
}
