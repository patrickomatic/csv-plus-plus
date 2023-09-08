//! # SourceCode
//!
//! The original source code being compiled.  When csv++ is first initialized the source code will
//! be read and a very rough parse will be done which reads line-by-line and splits the CSV section
//! from the code section by looking for the `---` token.
//!
//! After this both the code section and CSV section will be lexed and parsed using separate
//! algorithms.
//!
use std::cmp;
use std::fmt;
use std::fs;
use std::path;
use crate::{Error, Result};
use crate::compiler::token_library::CODE_SECTION_SEPARATOR;

// how many lines above (and below) we'll show as context when highlighting error messages
const LINES_IN_ERROR_CONTEXT: usize = 3;

type LineCount = usize;

#[derive(Debug)]
pub struct SourceCode {
    pub filename: path::PathBuf,
    pub lines: LineCount,
    pub length_of_code_section: LineCount,
    pub length_of_csv_section: LineCount,
    pub code_section: Option<String>,
    pub csv_section: String,
    pub original: String,
}

impl fmt::Display for SourceCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{}: total_lines: {}, csv_section: {}, code_section: {}", 
            self.filename.display(),
            self.lines,
            self.length_of_csv_section,
            self.length_of_code_section,
        )
    }
}

impl SourceCode {
    /// Open the source code and do a rough first pass where we split the code section from the CSV
    /// section by looking for `---`.
    pub fn open(filename: path::PathBuf) -> Result<SourceCode>  {
        let input = fs::read_to_string(&filename).map_err(|e| {
            Error::SourceCodeError {
                filename: filename.clone(),
                message: format!("Error reading source code {}: {e}", filename.display()),
            }
        })?;

        Self::new(input.as_str(), filename)
    }

    pub fn new(input: &str, filename: path::PathBuf) -> Result<SourceCode> {
        if let Some((code_section, csv_section)) = input.split_once(CODE_SECTION_SEPARATOR) {
            let csv_lines = csv_section.lines().count();
            let code_lines = code_section.lines().count();

            Ok(SourceCode {
                filename,
                lines: csv_lines + code_lines,
                length_of_code_section: code_lines,
                length_of_csv_section: csv_lines,
                csv_section: csv_section.to_string(),
                code_section: Some(code_section.to_string()), 
                original: input.to_owned(),
            })
        } else {
            let csv_lines = input.lines().count();
            Ok(SourceCode {
                filename,
                lines: csv_lines,
                length_of_code_section: 0,
                length_of_csv_section: csv_lines,
                csv_section: input.to_owned(),
                code_section: None, 
                original: input.to_owned(),
            })
        }
    }

    pub fn get_line(&self, line_number: usize) -> Option<String> {
        self.original
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .get(line_number - 1)
            .map(|s| s.to_string())
    }

    pub fn highlight_line(
        &self,
        line_number: usize,
        position: usize
    ) -> Vec<String> {
        let lines = self.original
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        // we present the line number as 1-based to the user, but index the array starting at 0
        let i = line_number - 1;

        // are they requesting a line totally outside of the range?
        if i > lines.len() {
            return vec![];
        }

        let start_index = i.saturating_sub(LINES_IN_ERROR_CONTEXT);
        let end_index = cmp::min(lines.len(), i + LINES_IN_ERROR_CONTEXT + 1);

        // start with 3 lines before, and also include our highlight line
        let mut lines_out = lines[start_index..(i + 1)].to_vec();

        // save the number of this line because we want to skip line-numbering it below
        let skip_numbering_on = lines_out.len();

        // draw something like this to highlight it:
        // ```
        //      foo!
        // --------^
        // ```
        lines_out.push(format!("{}^", "-".repeat(position - 1)));

        // and 3 lines after
        lines_out.append(&mut lines[(i + 1)..end_index].to_vec());

        // now format each line with line numbers
        let longest_line_number = (line_number + LINES_IN_ERROR_CONTEXT).to_string().len();
        let mut line_count = line_number.saturating_sub(LINES_IN_ERROR_CONTEXT).saturating_sub(1);

        // now iterate over it and apply lines numbers like `XX: some_code( ...` where XX is the
        // line number
        lines_out
            .iter()
            .enumerate()
            .map(|(i, line)| {
                // don't increment the line *after* the line we're highlighting.  because it's the 
                // ----^ thing and it doesn't correspond to a source code row, it's highlighting the
                // text above it
                if i == skip_numbering_on {
                    format!(" {: <width$}: {}", " ", line, width = longest_line_number)
                } else {
                    line_count += 1;
                    format!(" {: <width$}: {}", line_count, line, width = longest_line_number)
                }
            })
            .collect()
    }

    pub fn object_code_filename(&self) -> path::PathBuf {
        let mut f = self.filename.clone();
        f.set_extension("csvpo");
        f
    }

    pub fn csv_line_number(&self, position: a1_notation::Address) -> usize {
        let row = position.row.y;
        self.length_of_code_section + 2 + row
    }
}

#[cfg(test)]
mod tests {
    use std::path;
    use crate::test_utils::TestFile;
    use super::*;

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
    fn display() {
        assert_eq!(
            "test.csvpp: total_lines: 25, csv_section: 15, code_section: 10", 
            build_source_code().to_string(),
        );
    }

    #[test]
    fn get_line_none() {
        let source_code = build_source_code();
        assert_eq!(source_code.get_line(100), None);
    }

    #[test]
    fn get_line_some() {
        let source_code = build_source_code();
        assert_eq!(source_code.get_line(11), Some("---".to_string()));
    }

    #[test]
    fn highlight_line() {
        let source_code = SourceCode::new(
            "
# A comment

var := 1
other_var := 42

something {
    foo: bar
}
---
foo,bar,baz
            ",
            path::PathBuf::from("test.csvpp"),
        ).unwrap();

        assert_eq!(
            source_code.highlight_line(8, 6), 
            vec![
                " 5 : other_var := 42",
                " 6 : ",
                " 7 : something {",
                " 8 :     foo: bar",
                "   : -----^",
                " 9 : }",
                " 10: ---",
                " 11: foo,bar,baz",
            ]);
    }

    #[test]
    fn highlight_line_at_top() {
        let source_code = SourceCode::new(
            "# A comment

var := 1
other_var := 42

something {
    foo: bar
}
---
foo,bar,baz
            ",
            path::PathBuf::from("test.csvpp"),
        ).unwrap();

        assert_eq!(
            source_code.highlight_line(1, 6), 
            vec![
                " 1: # A comment",
                "  : -----^",
                " 2: ",
                " 3: var := 1",
                " 4: other_var := 42",
            ]);
    }

    #[test]
    fn object_code_filename() {
        assert_eq!(
            path::PathBuf::from("test.csvpo"), 
            build_source_code().object_code_filename());
    }

    #[test]
    fn open_no_code_section() {
        let source_code = SourceCode::new(
            "foo,bar,baz", 
            std::path::PathBuf::from("foo.csvpp")).unwrap();

        assert_eq!(source_code.lines, 1);
        assert_eq!(source_code.length_of_csv_section, 1);
        assert_eq!(source_code.length_of_code_section, 0);
        assert_eq!(source_code.code_section, None);
        assert_eq!(source_code.csv_section, "foo,bar,baz".to_string());
    }

    #[test]
    fn open_code_section() {
        let s = TestFile::new("csv", "
foo := 1

---
foo,bar,baz,=foo
");
        let source_code = SourceCode::open(s.input_file.clone()).unwrap();

        assert_eq!(source_code.lines, 5);
        assert_eq!(source_code.length_of_csv_section, 2);
        assert_eq!(source_code.length_of_code_section, 3);
        assert_eq!(source_code.code_section, Some("\nfoo := 1\n\n".to_string()));
        assert_eq!(source_code.csv_section, "\nfoo,bar,baz,=foo\n".to_string());
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
        ).unwrap();

        assert_eq!(8, source_code.csv_line_number(a1_notation::Address::new(1, 1)));
    }
}
