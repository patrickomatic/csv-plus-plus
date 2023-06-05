//!
//!
use std::fmt;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;

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

// fn split_template(reader: &BufReader) -> (Option<String>, String) {
    // TODO
// }

impl SourceCode {
    pub fn open(filename: PathBuf) -> Result<SourceCode, String>  {
        let mut total_lines = 0;
        let mut separator_line: Option<LineCount> = None;
        let mut code_section_str = String::from("");
        let mut csv_section = String::from("");

        let file = match File::open(&filename) {
            Ok(file) => file,
            Err(error) => return Err(format!("Error opening {}: {}", &filename.display(), error.to_string())),
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
                Err(message) => return Err(format!("Error reading line {}: {}", total_lines, message)),
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
}
