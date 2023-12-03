use super::ModulePath;
use crate::error::{Error, Result};
use crate::parser::ast_lexer::TokenMatch;
use std::path;

impl From<ModulePath> for path::PathBuf {
    fn from(_mn: ModulePath) -> path::PathBuf {
        // XXX replace '.' with '/'
        path::Path::new("foo").to_path_buf()
    }
}

impl TryFrom<path::PathBuf> for ModulePath {
    type Error = Error;

    fn try_from(p: path::PathBuf) -> Result<Self> {
        if let Some(f) = p.file_stem() {
            // XXX we to join the paths to be relative
            Ok(ModulePath(vec![f.to_string_lossy().to_string()]))
        } else {
            Err(Error::ModuleLoadError(format!(
                "Unable to get base filename for file: {}",
                p.display()
            )))
        }
    }
}

impl TryFrom<TokenMatch<'_>> for ModulePath {
    type Error = Error;

    fn try_from(tm: TokenMatch) -> Result<Self> {
        // XXX need to do more validation
        Ok(Self(
            tm.str_match.split('/').map(|s| s.to_string()).collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from_token_match() {
        let token_match = build_ast_token_match("foo", build_source_code());

        assert_eq!(
            ModulePath::try_from(token_match).unwrap(),
            ModulePath(vec!["foo".to_string()])
        );
    }

    #[test]
    fn try_from_path_buf_just_file() {
        assert_eq!(
            ModulePath::try_from(path::PathBuf::from("test.csvpp")).unwrap(),
            ModulePath(vec!["test".to_string()]),
        );
    }

    #[test]
    fn try_from_path_buf_with_path_separators() {
        assert_eq!(
            ModulePath::try_from(path::PathBuf::from("/home/foo/projects/test.csvpp")).unwrap(),
            ModulePath(vec!["test".to_string()]),
        );
    }
}
