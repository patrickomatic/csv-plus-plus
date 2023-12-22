use std::ffi;
use std::path;

mod display;
mod try_from;

type ModulePathComponent = String;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ModulePath(pub(crate) Vec<ModulePathComponent>);

impl ModulePath {
    pub(crate) fn filename_relative_to(self, module_path: &Self) -> path::PathBuf {
        let mut relative_to: path::PathBuf = module_path.clone().into();
        relative_to = relative_to
            .parent()
            .unwrap_or(path::Path::new(ffi::OsStr::new("")))
            .to_path_buf();

        let self_path: path::PathBuf = self.into();
        path::Path::new(&relative_to).join(self_path)
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use std::path;

    #[test]
    fn filename_relative_to() {
        // XXX
    }
}
