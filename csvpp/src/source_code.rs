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
use std::fs;
use std::path;
use crate::{Error, Result};
use crate::compiler::token_library::CODE_SECTION_SEPARATOR;

type LineCount = usize;

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
        let input = fs::read_to_string(&filename).map_err(|e| {
            Error::SourceCodeError {
                filename: filename.clone(),
                message: format!("Error reading source code {}: {}", filename.display(), e),
            }
        })?;

        if let Some((code_section, csv_section)) = input.split_once(CODE_SECTION_SEPARATOR) {
            let csv_lines = csv_section.lines().count();
            let code_lines = code_section.lines().count();

            Ok(SourceCode {
                filename,
                lines: csv_lines + code_lines,
                length_of_code_section: code_lines,
                length_of_csv_section: csv_lines,
                csv_section: csv_section.trim().to_string(),
                code_section: Some(code_section.trim().to_string()), 
            })
        } else {
            let csv_lines = input.lines().count();
            Ok(SourceCode {
                filename,
                lines: csv_lines,
                length_of_code_section: 0,
                length_of_csv_section: csv_lines,
                csv_section: input.trim().to_owned(),
                code_section: None, 
            })
        }
    }

    pub fn object_code_filename(&self) -> path::PathBuf {
        let mut f = self.filename.clone();
        f.set_extension("csvpo");
        f
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::fs;
    use std::path;
    use super::*;

    struct Setup {
        source: path::PathBuf,
    }

    impl Setup {
        fn new(input: &str) -> Self {
            let mut rng = rand::thread_rng();

            let random_filename = format!("source_code_test_input{}.csvpp", rng.gen::<u64>());
            let source_path = path::Path::new(&random_filename);
            fs::write(source_path, input).unwrap();

            Self { source: source_path.to_path_buf() }
        }
    }

    impl Drop for Setup {
        fn drop(&mut self) {
            fs::remove_file(&self.source).unwrap();
        }
    }

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

    #[test]
    fn open_no_code_section() {
        let s = Setup::new("foo,bar,baz");
        let source_code = SourceCode::open(s.source.clone()).unwrap();

        assert_eq!(source_code.lines, 1);
        assert_eq!(source_code.length_of_csv_section, 1);
        assert_eq!(source_code.length_of_code_section, 0);
        assert_eq!(source_code.code_section, None);
        assert_eq!(source_code.csv_section, "foo,bar,baz".to_string());
    }

    #[test]
    fn open_code_section() {
        let s = Setup::new(r#"
foo := 1

---
foo,bar,baz,=foo
"#);
        let source_code = SourceCode::open(s.source.clone()).unwrap();

        assert_eq!(source_code.lines, 5);
        assert_eq!(source_code.length_of_csv_section, 2);
        assert_eq!(source_code.length_of_code_section, 3);
        assert_eq!(source_code.code_section, Some("foo := 1".to_string()));
        assert_eq!(source_code.csv_section, "foo,bar,baz,=foo".to_string());
    }
}
