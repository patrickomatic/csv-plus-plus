mod display;
mod try_from;

type ModulePathComponent = String;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ModulePath(pub(crate) Vec<ModulePathComponent>);

impl ModulePath {
    #[cfg(test)]
    pub(crate) fn new(name: &str) -> Self {
        // NOTE: only to make tests easy, actual code should use the TryFrom impls
        Self(
            name.split('/')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        )
    }
}
