use crate::ast::{Ast, Functions, Variables};
use std::collections;

mod display;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Scope {
    pub(crate) functions: Functions,
    pub(crate) variables: Variables,
    pub(crate) exports: collections::HashSet<String>,
}

impl Scope {
    pub(crate) fn merge_variables(self, vars: Variables) -> Self {
        Self {
            variables: vars.into_iter().chain(self.variables).collect(),
            ..self
        }
    }

    pub(crate) fn define_function<S, A>(&mut self, name: S, ast: A)
    where
        S: Into<String>,
        A: Into<Ast>,
    {
        let name = name.into();
        self.exports.insert(name.clone());
        self.functions.insert(name, ast.into());
    }

    pub(crate) fn define_variable<S, A>(&mut self, name: S, ast: A)
    where
        S: Into<String>,
        A: Into<Ast>,
    {
        let name = name.into();
        self.exports.insert(name.clone());
        self.variables.insert(name, ast.into());
    }

    pub(crate) fn merge(self, other: Self) -> Self {
        Self {
            // functions: self.functions.into_iter().chain(other.functions).collect(),
            functions: other.functions.into_iter().chain(self.functions).collect(),
            ..self
        }
        .merge_variables(other.variables)
    }

    // merge two scopes, but only the exports from `other`
    pub(crate) fn merge_into_main(self, other: Self) -> Self {
        let mut exports = self.exports;
        let mut functions = self.functions;
        let mut variables = self.variables;
        for e in other.exports {
            if let Some(f) = other.functions.get(&e) {
                if !functions.contains_key(&e) {
                    functions.insert(e.clone(), f.clone());
                }
            }
            if let Some(v) = other.variables.get(&e) {
                if !variables.contains_key(&e) {
                    variables.insert(e.clone(), v.clone());
                }
            }
            exports.insert(e);
        }

        Self {
            exports,
            functions,
            variables,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use std::collections::*;

    #[test]
    fn merge() {
        let scope1 = Scope {
            variables: HashMap::from([
                ("foo".to_string(), 1.into()),
                ("bar".to_string(), 2.into()),
            ]),
            exports: HashSet::from(["foo".to_string(), "bar".to_string()]),
            ..Default::default()
        };
        let scope2 = Scope {
            variables: HashMap::from([("foobs".to_string(), 1.into())]),
            functions: HashMap::from([(
                "func".to_string(),
                Node::fn_def("func", &["A", "B"], Ast::new(1.into())).into(),
            )]),
            ..Default::default()
        };
        let merged = scope1.merge(scope2);

        assert_eq!(merged.variables.len(), 3);
        assert_eq!(merged.functions.len(), 1);
        assert_eq!(merged.exports.len(), 2);
    }

    #[test]
    fn merge_variables() {
        let mut scope = Scope::default();
        scope = scope.merge_variables(HashMap::from([
            ("foo".to_string(), 1.into()),
            ("bar".to_string(), 2.into()),
        ]));

        assert_eq!(scope.variables.len(), 2);
        assert_eq!(scope.functions.len(), 0);
        assert_eq!(scope.exports.len(), 0);
    }
}
