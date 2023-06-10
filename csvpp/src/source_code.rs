//!
//!
use std::fmt;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;

use crate::Error;

type LineCount = u16;

#[derive(Debug)]
pub struct SourceCode {
    pub filename: PathBuf,
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
    pub fn open(filename: PathBuf) -> Result<SourceCode, Error>  {
        let mut total_lines = 0;
        let mut separator_line: Option<LineCount> = None;
        let mut code_section_str = String::from("");
        let mut csv_section = String::from("");

        let file = match File::open(&filename) {
            Ok(file) => file,
            Err(error) => 
                return Err(Error::InitError(
                    format!("Error opening {}: {}", &filename.display(), error.to_string()),
                )),
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(l) => {
                    if l.trim() == "---" {
                        separator_line = Some(total_lines + 1);
                        continue;
                    } 

                    if separator_line == None {
                        code_section_str.push_str(&l);
                        code_section_str.push_str("\n");
                    } else {
                        csv_section.push_str(&l);
                        csv_section.push_str("\n");
                    }

                    total_lines += 1;
                },
                Err(message) => 
                    return Err(Error::InitError(
                        format!("Error reading line {}: {}", total_lines, message),
                    )),
            }
        }

        let length_of_code_section = separator_line.unwrap_or(0);

        let code_section = if separator_line == None { None } else { Some(code_section_str) };

        Ok(SourceCode {
            filename,
            lines: total_lines,
            length_of_code_section,
            length_of_csv_section: total_lines - length_of_code_section,
            csv_section,
            code_section, 
        })
    }

    pub fn object_code_filename(&self) -> PathBuf {
        let mut f = self.filename.clone();
        f.set_extension("csvpo");
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_source_code() -> SourceCode {
        SourceCode {
            filename: PathBuf::from("test.csvpp".to_string()),
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
            test_source_code().to_string(),
        );
    }

    #[test]
    fn object_code_filename() {
        assert_eq!(PathBuf::from("test.csvpo"), test_source_code().object_code_filename());
    }
}
