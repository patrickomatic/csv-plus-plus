//! # SourceCode
//!
//! The original source code being compiled.  When csv++ is first initialized the source code will
//! be read and a very rough parse will be done which reads line-by-line and splits the CSV section
//! from the code section by looking for the `---` token.
//!
//! After this both the code section and CSV section will be lexed and parsed using separate
//! algorithms.
//!
use std::fmt;
use std::io;
use std::io::BufRead;
use std::fs;
use std::path;
use crate::{Error, Result};
use crate::compiler::token_library::CODE_SECTION_SEPARATOR;

type LineCount = u16;

#[derive(Debug)]
pub struct SourceCode {
    pub filename: path::PathBuf,
    pub lines: LineCount,
    pub length_of_code_section: LineCount,
    pub length_of_csv_section: LineCount,
    pub code_section: Option<String>,
    pub csv_section: String,
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
        let mut total_lines = 0;
        let mut separator_line: Option<LineCount> = None;
        let mut code_section_str = String::from("");
        let mut csv_section = String::from("");

        let file = Self::open_file(&filename)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(l) => {
                    if l.trim() == CODE_SECTION_SEPARATOR {
                        separator_line = Some(total_lines + 1);
                        continue;
                    } 

                    if separator_line.is_none() {
                        code_section_str.push_str(&l);
                        code_section_str.push('\n');
                    } else {
                        csv_section.push_str(&l);
                        csv_section.push('\n');
                    }

                    total_lines += 1;
                },
                Err(message) => 
                    return Err(Error::SourceCodeError {
                        filename,
                        message: format!("Error reading line {}: {}", total_lines, message),
                    }),
            }
        }

        let length_of_code_section = separator_line.unwrap_or(0);

        let code_section = if separator_line.is_none() { None } else { Some(code_section_str) };

        Ok(SourceCode {
            filename,
            lines: total_lines,
            length_of_code_section,
            length_of_csv_section: total_lines - length_of_code_section,
            csv_section,
            code_section, 
        })
    }

    pub fn object_code_filename(&self) -> path::PathBuf {
        let mut f = self.filename.clone();
        f.set_extension("csvpo");
        f
    }

    fn open_file(filename: &path::PathBuf) -> Result<fs::File> {
        fs::File::open(filename).map_err(|error| Error::SourceCodeError {
                filename: filename.to_path_buf(),
                message: format!("Error opening file: {}", error),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_source_code() -> SourceCode {
        SourceCode {
            filename: path::PathBuf::from("test.csvpp".to_string()),
            lines: 25,
            length_of_code_section: 10,
            length_of_csv_section: 15,
            code_section: Some("\n".repeat(10)),
            csv_section: "foo,bar,baz".to_string(),
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
    fn object_code_filename() {
        assert_eq!(
            path::PathBuf::from("test.csvpo"), 
            build_source_code().object_code_filename());
    }
}
