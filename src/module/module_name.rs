use crate::error::{Error, Result};
use crate::parser::ast_lexer::TokenMatch;
use std::fmt;
use std::path;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ModuleName(pub String);

impl TryFrom<path::PathBuf> for ModuleName {
    type Error = Error;

    fn try_from(p: path::PathBuf) -> Result<Self> {
        if let Some(f) = p.file_stem() {
            Ok(ModuleName(f.to_string_lossy().to_string()))
        } else {
            // TODO: throw a different error
            Err(Error::ObjectCodeError {
                filename: p.to_path_buf(),
                message: "Unable to get base filename".to_string(),
            })
        }
    }
}

impl TryFrom<TokenMatch<'_>> for ModuleName {
    type Error = Error;

    fn try_from(tm: TokenMatch) -> Result<Self> {
        Ok(Self(tm.str_match.to_string()))
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
        let source_code = build_source_code();
        let token_match = build_ast_token_match("foo", &source_code);

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
