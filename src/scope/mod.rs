use crate::ast::{Functions, Variables};
use crate::ModulePath;

mod display;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Scope {
    pub(crate) functions: Functions,
    pub(crate) required_modules: Vec<ModulePath>,
    pub(crate) variables: Variables,
}

impl Scope {
    pub(crate) fn merge_variables(self, vars: Variables) -> Self {
        Self {
            variables: self.variables.into_iter().chain(vars).collect(),
            ..self
        }
    }

    pub(crate) fn merge(self, other: Self) -> Self {
        Self {
            functions: self.functions.into_iter().chain(other.functions).collect(),
            variables: self.variables.into_iter().chain(other.variables).collect(),
            ..self
        }
    }
}
