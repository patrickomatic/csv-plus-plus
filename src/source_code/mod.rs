//! # SourceCode
//!
//! The original source code being compiled.  When csv++ is first initialized the source code will
//! be read and a very rough parse will be done which reads line-by-line and splits the CSV section
//! from the code section by looking for the `---` token.
//!
//! After this both the code section and CSV section will be lexed and parsed using separate
//! algorithms.
//!
use crate::parser::ast_lexer::CODE_SECTION_SEPARATOR;
use crate::{csv_reader, Error, Result};
use std::fs;
use std::path;

mod display;
mod errors;

/// the line number - counts from `0` but renders the first line as `"1"`
pub type LineNumber = usize;

/// the amount of characters offset from the beginning of the line
pub type CharOffset = usize;

#[derive(Debug)]
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
    pub fn open(filename: &path::PathBuf) -> Result<SourceCode> {
        let input = fs::read_to_string(filename).map_err(|e| Error::SourceCodeError {
            filename: filename.clone(),
            message: format!("Error reading source code {}: {e}", filename.display()),
        })?;

        Self::new(input.as_str(), filename.clone())
    }

    pub fn new<S: Into<String>>(input: S, filename: path::PathBuf) -> Result<SourceCode> {
        let str_input: String = input.into();

        if let Some((code_section, csv_section)) = str_input.split_once(CODE_SECTION_SEPARATOR) {
            let csv_lines = csv_section.lines().count();
            let code_lines = code_section.lines().count();

            Ok(SourceCode {
                filename,
                lines: csv_lines + code_lines,
                length_of_code_section: code_lines,
                length_of_csv_section: csv_lines,
                csv_section: csv_section.to_string(),
                code_section: Some(code_section.to_string()),
                original: str_input.to_owned(),
            })
        } else {
            let csv_lines = str_input.lines().count();

            Ok(SourceCode {
                filename,
                lines: csv_lines,
                length_of_code_section: 0,
                length_of_csv_section: csv_lines,
                csv_section: str_input.to_owned(),
                code_section: None,
                original: str_input.to_owned(),
            })
        }
    }

    // TODO: store the lines split so I don't have to do this more than once?
    pub(crate) fn get_line(&self, line_number: LineNumber) -> Option<String> {
        self.original
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .get(line_number)
            .map(|s| s.to_string())
    }

    pub(crate) fn object_code_filename(&self) -> path::PathBuf {
        let mut f = self.filename.clone();
        f.set_extension("csvpo");
        f
    }

    pub(crate) fn csv_line_number(&self, position: a1_notation::Address) -> LineNumber {
        let row = position.row.y;
        self.length_of_code_section + 1 + row
    }

    pub(crate) fn line_offset_for_cell(&self, position: a1_notation::Address) -> CharOffset {
        let line_number = self.csv_line_number(position);
        let Some(line) = self.get_line(line_number) else {
            // TODO: Err
            return 0;
        };

        let mut reader = csv_reader().from_reader(line.as_bytes());

        if let Some(result) = reader.records().next() {
            let record = result.unwrap();
            let x = position.column.x;

            if x > record.len() || x == 0 {
                // TODO: err
                return 0;
            }

            // TODO: this doesn't work if the input takes advantage of CSV's weird double-quote
            // escaping rules. but tbh I dunno if it matters much
            //
            // the length of all the cells (since spaces is preserved), plus how many commas would have joined them
            (0..x).fold(0, |acc, i| acc + record[i].len()) + x
        } else {
            // TODO: Err instead?
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
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
    fn csv_line_number() {
        let source_code = SourceCode::new(
            "# A comment
var := 1

other_var := 42

---
foo,bar,baz
foo1,bar1,baz1
            ",
            path::PathBuf::from("test.csvpp"),
        )
        .unwrap();

        assert_eq!(
            7,
            source_code.csv_line_number(a1_notation::Address::new(1, 1))
        );
    }

    #[test]
    fn get_line_none() {
        let source_code = build_source_code();
        assert_eq!(source_code.get_line(99), None);
    }

    #[test]
    fn get_line_some() {
        let source_code = build_source_code();
        assert_eq!(source_code.get_line(10), Some("---".to_string()));
    }

    #[test]
    fn line_offset_for_cell() {
        let source_code = SourceCode::new(
            "# A comment
---
foo,bar,baz
foo1,bar1,baz1
            ",
            path::PathBuf::from("test.csvpp"),
        )
        .unwrap();

        assert_eq!(source_code.line_offset_for_cell((0, 0).into()), 0);
        assert_eq!(source_code.line_offset_for_cell((1, 0).into()), 4);
        assert_eq!(source_code.line_offset_for_cell((2, 0).into()), 8);
        assert_eq!(source_code.line_offset_for_cell((1, 1).into()), 5);
    }

    #[test]
    fn line_offset_for_cell_with_spaces() {
        let source_code = SourceCode::new(
            "# A comment
---
  foo,  bar,baz
            ",
            path::PathBuf::from("test.csvpp"),
        )
        .unwrap();

        assert_eq!(source_code.line_offset_for_cell((1, 0).into()), 6);
    }

    #[ignore]
    #[test]
    fn line_offset_for_cell_with_quotes() {
        let source_code = SourceCode::new(
            "# A comment
---
\" hmmm, this is all one cell\",baz
            ",
            path::PathBuf::from("test.csvpp"),
        )
        .unwrap();

        assert_eq!(source_code.line_offset_for_cell((1, 0).into()), 34);
    }

    #[test]
    fn object_code_filename() {
        assert_eq!(
            path::PathBuf::from("test.csvpo"),
            build_source_code().object_code_filename()
        );
    }

    #[test]
    fn open_no_code_section() {
        let source_code =
            SourceCode::new("foo,bar,baz", std::path::PathBuf::from("foo.csvpp")).unwrap();

        assert_eq!(source_code.lines, 1);
        assert_eq!(source_code.length_of_csv_section, 1);
        assert_eq!(source_code.length_of_code_section, 0);
        assert_eq!(source_code.code_section, None);
        assert_eq!(source_code.csv_section, "foo,bar,baz".to_string());
    }

    #[test]
    fn open_code_section() {
        let s = TestSourceCode::new(
            "csv",
            "
foo := 1

---
foo,bar,baz,=foo
",
        );
        let source_code = SourceCode::open(&s.input_file).unwrap();

        assert_eq!(source_code.lines, 5);
        assert_eq!(source_code.length_of_csv_section, 2);
        assert_eq!(source_code.length_of_code_section, 3);
        assert_eq!(source_code.code_section, Some("\nfoo := 1\n\n".to_string()));
        assert_eq!(source_code.csv_section, "\nfoo,bar,baz,=foo\n".to_string());
    }
}
