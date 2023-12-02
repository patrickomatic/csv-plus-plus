use super::Compiler;
use std::fmt;

impl fmt::Display for Compiler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# CLI Options")?;
        writeln!(f, "{}", self.options)?;
        writeln!(f, "\n# Parsed Source Code")?;
        writeln!(f, "{}", *self.source_code)?;
        writeln!(f, "\n# Output Target")?;
        writeln!(f, "{}", self.output)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn display() {
        let test_file = TestSourceCode::new("csv", "foo,bar,baz");
        let cli_args = CliArgs {
            input_filename: test_file.input_file.clone(),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        let compiler_str = Compiler::try_from(&cli_args).unwrap().to_string();

        assert!(compiler_str.contains("CLI Options"));
        assert!(compiler_str.contains("Parsed Source Code"));
        assert!(compiler_str.contains("Output Target"));
    }
}
