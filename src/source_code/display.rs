use super::SourceCode;
use std::fmt;

impl fmt::Display for SourceCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: total_lines: {}, csv_section: {}, scope: {}",
            self.filename.display(),
            self.lines,
            self.length_of_csv_section,
            self.length_of_scope,
        )
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
            length_of_scope: 10,
            length_of_csv_section: 15,
            scope: Some("\n".repeat(10)),
            csv_section: "foo,bar,baz".to_string(),
            original: "\n\n\n\n\n\n\n\n\n\n---\nfoo,bar,baz".to_string(),
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            "test.csvpp: total_lines: 25, csv_section: 15, scope: 10",
            build_source_code().to_string(),
        );
    }
}
