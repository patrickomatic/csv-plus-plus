use crate::error::{Error, Result};
use crate::parser::ast_lexer::TokenMatch;
use std::fmt;
use std::path;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ModuleName(pub String);

impl ModuleName {
    pub(crate) fn new<S: Into<String>>(name: S) -> Self {
        Self(name.into())
    }
}

#[allow(clippy::from_over_into)]
impl Into<path::PathBuf> for ModuleName {
    fn into(self) -> path::PathBuf {
        // XXX replace '.' with '/'
        todo!()
    }
}

impl TryFrom<path::PathBuf> for ModuleName {
    type Error = Error;

    fn try_from(p: path::PathBuf) -> Result<Self> {
        if let Some(f) = p.file_stem() {
            Ok(ModuleName::new(f.to_string_lossy()))
        } else {
            // TODO: throw a different error
            Err(Error::ObjectCodeError {
                filename: p.to_path_buf(),
                message: format!("Unable to get base filename for file: {}", p.display()),
            })
        }
    }
}

impl TryFrom<TokenMatch<'_>> for ModuleName {
    type Error = Error;

    fn try_from(tm: TokenMatch) -> Result<Self> {
        Ok(Self::new(tm.str_match))
    }
}

impl fmt::Display for ModuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
            ModuleName::try_from(token_match).unwrap(),
            ModuleName("foo".to_string())
        );
    }

    #[test]
    fn try_from_path_buf_just_file() {
        assert_eq!(
            ModuleName::try_from(path::PathBuf::from("test.csvpp")).unwrap(),
            ModuleName("test".to_string()),
        );
    }

    #[test]
    fn try_from_path_buf_with_path_separators() {
        assert_eq!(
            ModuleName::try_from(path::PathBuf::from("/home/foo/projects/test.csvpp")).unwrap(),
            ModuleName("test".to_string()),
        );
    }
}
