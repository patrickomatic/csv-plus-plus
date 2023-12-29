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

    /// Merge another `Scope` (only it's `exports`) into this one. if a var or function is already
    /// defined, we do not insert it
    pub(crate) fn merge(&mut self, other: &Self) {
        for e in other.exports.iter() {
            if let Some(f) = other.functions.get(e) {
                if !self.functions.contains_key(e) {
                    self.functions.insert(e.clone(), f.clone());
                }
            }
            if let Some(v) = other.variables.get(e) {
                if !self.variables.contains_key(e) {
                    self.variables.insert(e.clone(), v.clone());
                }
            }
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
        let mut scope2 = Scope {
            variables: HashMap::from([("foobs".to_string(), 1.into())]),
            functions: HashMap::from([(
                "func".to_string(),
                Node::fn_def("func", &["A", "B"], Ast::new(1.into())).into(),
            )]),
            ..Default::default()
        };
        scope2.merge(&scope1);

        assert_eq!(scope2.variables.len(), 3);
        assert_eq!(scope2.functions.len(), 1);
        assert_eq!(scope2.exports.len(), 0);
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
