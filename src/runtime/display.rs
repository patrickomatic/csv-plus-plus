use std::fmt;
use super::Runtime;

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# CLI Options")?;
        writeln!(f, "{}", self.options)?;
        writeln!(f, "\n# Parsed Source Code")?;
        writeln!(f, "{}", self.source_code)?;
        writeln!(f, "\n# Output Target")?;
        writeln!(f, "{}", self.output)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::TestFile;
    use crate::*;

    #[test]
    fn display() {
        let test_file = TestFile::new("csv", "foo,bar,baz");
        let cli_args = CliArgs {
            input_filename: test_file.input_file.clone(),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        let runtime_str = Runtime::try_from(&cli_args).unwrap().to_string();

        assert!(runtime_str.contains("CLI Options"));
        assert!(runtime_str.contains("Parsed Source Code"));
        assert!(runtime_str.contains("Output Target"));
    }
}

