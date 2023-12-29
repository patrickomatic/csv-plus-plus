use super::ModulePath;
use crate::error::{Error, Result};
use crate::parser::ast_lexer::TokenMatch;
use std::path;

impl From<ModulePath> for path::PathBuf {
    fn from(mn: ModulePath) -> path::PathBuf {
        let mut p: path::PathBuf = mn.0.iter().collect();
        p.set_extension("csvpp");
        p
    }
}

impl TryFrom<path::PathBuf> for ModulePath {
    type Error = Error;

    // TODO: do some validation
    fn try_from(mut p: path::PathBuf) -> Result<Self> {
        p.set_extension("");

        Ok(ModulePath(
            p.components()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .collect(),
        ))
    }
}

impl TryFrom<TokenMatch<'_>> for ModulePath {
    type Error = Error;

    // TODO do more validation (can only be [\w_/])
    fn try_from(tm: TokenMatch) -> Result<Self> {
        Ok(Self(
            tm.str_match
                .split('/')
                .map(std::string::ToString::to_string)
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use std::path;

    #[test]
    fn path_buf_from_module_path() {
        assert_eq!(
            path::PathBuf::from(ModulePath::new("foo/bar/baz")),
            path::Path::new("foo/bar/baz.csvpp").to_path_buf()
        );
    }

    #[test]
    fn try_from_token_match() {
        let token_match = build_ast_token_match("foo", build_source_code());

        assert_eq!(
            ModulePath::try_from(token_match).unwrap(),
            ModulePath::new("foo"),
        );
    }

    #[test]
    fn try_from_path_buf_just_file() {
        assert_eq!(
            ModulePath::try_from(path::PathBuf::from("test.csvpp")).unwrap(),
            ModulePath::new("test"),
        );
    }

    #[test]
    fn try_from_path_buf_with_path_separators() {
        assert_eq!(
            ModulePath::try_from(path::PathBuf::from("projects/test.csvpp")).unwrap(),
            ModulePath::new("projects/test"),
        );
    }
}
