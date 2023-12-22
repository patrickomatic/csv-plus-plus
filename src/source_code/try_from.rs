use super::SourceCode;
use crate::{Error, Result};
use std::fs;
use std::path;

impl TryFrom<path::PathBuf> for SourceCode {
    type Error = Error;

    fn try_from(p: path::PathBuf) -> Result<Self> {
        let input = fs::read_to_string(&p).map_err(|e| Error::SourceCodeError {
            filename: p.clone(),
            message: format!("Error reading source code {}: {e}", p.display()),
        })?;

        Self::new(input.as_str(), p)
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

        dbg!(&source_code);
        assert_eq!(source_code.lines, 5);
        // TODO: the csv_section should not include an additional newline
        assert_eq!(source_code.length_of_csv_section, 2);
        assert_eq!(source_code.length_of_code_section, 4);
        assert_eq!(source_code.code_section, Some("\nfoo := 1\n\n".to_string()));
        assert_eq!(source_code.csv_section, "\nfoo,bar,baz,=foo\n".to_string());
    }
}
