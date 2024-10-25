//! # `SourceCode`
//!
use crate::parser::ast_lexer::CODE_SECTION_SEPARATOR;
use crate::{compiler_error, csv_reader};
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

        if let Some((code_section, csv_section)) = str_input.split_once(CODE_SECTION_SEPARATOR) {
            let csv_lines = csv_section.lines().count();
            let code_lines = code_section.lines().count();

            SourceCode {
                filename: filename.into(),
                lines: csv_lines + code_lines,
                // +1 because `code_lines` will account for the separator `---`
                length_of_code_section: code_lines + 1,
                length_of_csv_section: csv_lines,
                csv_section: csv_section.to_string(),
                code_section: Some(code_section.to_string()),
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

    // TODO: store the lines split so I don't have to do this more than once?
    pub(crate) fn get_line(&self, line_number: LineNumber) -> Option<String> {
        self.original
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .get(line_number)
            .map(std::string::ToString::to_string)
    }

    pub(crate) fn object_code_filename(&self) -> path::PathBuf {
        let mut f = self.filename.clone();
        f.set_extension("csvpo");
        f
    }

    pub(crate) fn csv_line_number(&self, position: a1::Address) -> LineNumber {
        let row = position.row.y;
        self.length_of_code_section + row
    }

    pub(crate) fn line_offset_for_cell(
        &self,
        position: a1::Address,
        add_leading_whitespace: bool,
    ) -> CharOffset {
        let line_number = self.csv_line_number(position);
        let Some(line) = self.get_line(line_number) else {
            compiler_error(format!(
                "Unable to find line for `line_number` = {line_number}"
            ));
        };

        let mut reader = csv_reader().from_reader(line.as_bytes());

        if let Some(result) = reader.records().next() {
            let record = result.unwrap();
            let x = position.column.x;

            if x > record.len() {
                compiler_error(format!(
                    "CSV contained more cells than expected: `x` = {x}, `record.len()` = {}",
                    record.len()
                ));
            }

            // TODO: this doesn't work if the input takes advantage of CSV's weird double-quote
            // escaping rules. but tbh I dunno if it matters much
            //
            let leading_count = if add_leading_whitespace {
                record[x].chars().take_while(|x| x.is_whitespace()).count() + 1
            } else {
                0
            };
            // the length of all the cells (since spaces is preserved), plus how many commas (x) would have joined them
            (0..x).fold(0, |acc, i| acc + record[i].len()) + x + leading_count
        } else {
            compiler_error("Unable to read CSV results to generate error");
        }
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
    fn csv_line_number() {
        let source_code = SourceCode::new(
            "# A comment
var := 1

other_var := 42

---
foo,bar,baz
foo1,bar1,baz1
            ",
            "test.csvpp",
        );

        assert_eq!(7, source_code.csv_line_number(a1::Address::new(1, 1)));
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
            "test.csvpp",
        );

        assert_eq!(source_code.line_offset_for_cell((0, 0).into(), false), 0);
        assert_eq!(source_code.line_offset_for_cell((1, 0).into(), false), 4);
        assert_eq!(source_code.line_offset_for_cell((2, 0).into(), false), 8);
        assert_eq!(source_code.line_offset_for_cell((1, 1).into(), false), 5);
    }

    #[test]
    fn line_offset_for_cell_with_spaces() {
        let source_code = SourceCode::new(
            "# A comment
---
  foo,  bar,baz
            ",
            "test.csvpp",
        );

        assert_eq!(source_code.line_offset_for_cell((1, 0).into(), false), 6);
    }

    #[ignore]
    #[test]
    fn line_offset_for_cell_with_quotes() {
        let source_code = SourceCode::new(
            "# A comment
---
\" hmmm, this is all one cell\",baz
            ",
            "test.csvpp",
        );

        assert_eq!(source_code.line_offset_for_cell((1, 0).into(), false), 34);
    }

    #[test]
    fn object_code_filename() {
        assert_eq!(
            path::PathBuf::from("test.csvpo"),
            build_source_code().object_code_filename()
        );
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
