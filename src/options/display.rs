use super::Options;
use std::fmt;

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "backup: {}", self.backup)?;
        writeln!(
            f,
            "google_account_credentials: {}",
            self.redacted_google_account_credentials()
        )?;
        writeln!(f, "key_values: {:?}", self.key_values)?;
        writeln!(f, "offset: ({}, {})", self.offset.0, self.offset.1)?;
        writeln!(f, "overwrite_values: {}", self.overwrite_values)?;
        writeln!(f, "sheet_name: {}", self.sheet_name)?;
        write!(f, "verbosity: {}", self.verbosity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let options = Options::default();

        assert_eq!(
            r#"backup: false
google_account_credentials: none
key_values: {}
offset: (0, 0)
overwrite_values: true
sheet_name: empty
verbosity: INFO"#,
            options.to_string()
        );
    }
}
