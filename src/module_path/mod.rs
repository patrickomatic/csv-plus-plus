mod display;
mod try_from;

type ModulePathComponent = String;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ModulePath(pub(crate) Vec<ModulePathComponent>);

impl ModulePath {}
