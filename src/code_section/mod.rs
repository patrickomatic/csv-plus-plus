use crate::ast::{Functions, Variables};
use crate::ModuleName;

mod display;

#[derive(Debug, Default)]
pub struct CodeSection {
    pub(crate) functions: Functions,
    pub(crate) required_modules: Vec<ModuleName>,
    pub(crate) variables: Variables,
}
