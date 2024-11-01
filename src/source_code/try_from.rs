use crate::{Error, Result, SourceCode};
use std::{fs, path};

impl TryFrom<path::PathBuf> for SourceCode {
    type Error = Error;

    fn try_from(p: path::PathBuf) -> Result<Self> {
        let input = fs::read_to_string(&p).map_err(|e| Error::SourceCodeError {
            filename: p.clone(),
            message: format!("Error reading source code {}: {e}", p.display()),
        })?;

        Ok(Self::new(input.as_str(), p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from() {
        let s = TestSourceCode::new(
            "csv",
            "
foo := 1

---
foo,bar,baz,=foo
",
        );
        let source_code = SourceCode::try_from(s.input_file.clone()).unwrap();

        assert_eq!(source_code.lines, 5);
        assert_eq!(source_code.length_of_csv_section, 1);
        assert_eq!(source_code.length_of_code_section, 4);
        assert_eq!(source_code.code_section, Some("\nfoo := 1\n".to_string()));
        assert_eq!(source_code.csv_section, "foo,bar,baz,=foo".to_string());
    }
}
