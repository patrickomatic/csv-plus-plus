//! # `SourceCode`
//!
use std::path;

mod arc_source_code;
mod display;
mod errors;
mod try_from;

pub(crate) use arc_source_code::ArcSourceCode;

/// the line number - counts from `0` but renders the first line as `"1"`
pub type LineNumber = usize;

/// the amount of characters offset from the beginning of the line
pub type CharOffset = usize;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SourceCode {
    pub filename: path::PathBuf,
    pub(crate) lines: LineNumber,
    pub(crate) length_of_code_section: LineNumber,
    pub(crate) length_of_csv_section: LineNumber,
    pub(crate) code_section: Option<String>,
    pub(crate) csv_section: String,
    pub(crate) original: String,
}

impl SourceCode {
    /// Open the source code and do a rough first pass where we split the code section from the CSV
    /// section by looking for `---`.
    // TODO: make `filename` optional - we don't have it when reading from CLI key/values.  or if
    // we were ever to support from stdin
    pub(crate) fn new<S, P>(input: S, filename: P) -> SourceCode
    where
        S: Into<String>,
        P: Into<path::PathBuf>,
    {
        let str_input: String = input.into();

        if let Some(p) = str_input
            .lines()
            .position(|l| regex::Regex::new(r"^\s*---\s*$").unwrap().is_match(l))
        {
            let lines: Vec<_> = str_input.lines().collect();
            let csv_lines = &lines[(p + 1)..];
            let code_lines = &lines[..p];

            SourceCode {
                filename: filename.into(),
                lines: lines.len(),
                // +1 because `code_lines` will account for the separator `---`
                length_of_code_section: code_lines.len() + 1,
                length_of_csv_section: csv_lines.len(),
                csv_section: csv_lines.join("\n"),
                code_section: Some(code_lines.join("\n")),
                original: str_input.clone(),
            }
        } else {
            let csv_lines = str_input.lines().count();

            SourceCode {
                filename: filename.into(),
                lines: csv_lines,
                length_of_code_section: 0,
                length_of_csv_section: csv_lines,
                csv_section: str_input.clone(),
                code_section: None,
                original: str_input.clone(),
            }
        }
    }

    pub(crate) fn object_code_filename(&self) -> path::PathBuf {
        let mut f = self.filename.clone();
        f.set_extension("csvpo");
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    fn build_source_code() -> SourceCode {
        SourceCode {
            filename: path::PathBuf::from("test.csvpp".to_string()),
            lines: 25,
            length_of_code_section: 10,
            length_of_csv_section: 15,
            code_section: Some("\n".repeat(10)),
            csv_section: "foo,bar,baz".to_string(),
            original: "\n\n\n\n\n\n\n\n\n\n---\nfoo,bar,baz".to_string(),
        }
    }

    #[test]
    fn object_code_filename() {
        assert_eq!(
            path::PathBuf::from("test.csvpo"),
            build_source_code().object_code_filename()
        );
    }

    #[test]
    fn new() {
        let source_code = SourceCode::new(
            "foo := 2
bar := 3
---
foo,bar,baz
=foo",
            "foo.csvpp",
        );

        assert_eq!(source_code.lines, 5);
        assert_eq!(source_code.length_of_csv_section, 2);
        assert_eq!(source_code.length_of_code_section, 3);
    }

    #[test]
    fn new_no_scope() {
        let source_code = SourceCode::new("foo,bar,baz", "foo.csvpp");

        assert_eq!(source_code.lines, 1);
        assert_eq!(source_code.length_of_csv_section, 1);
        assert_eq!(source_code.length_of_code_section, 0);
        assert_eq!(source_code.code_section, None);
        assert_eq!(source_code.csv_section, "foo,bar,baz".to_string());
    }
}
