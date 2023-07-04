use std::collections;
use std::fmt;
use crate::ast::Ast;

#[derive(Debug)]
pub struct Options {
    pub backup: bool,
    pub google_account_credentials: Option<String>,
    pub key_values: collections::HashMap<String, Ast>,
    pub offset: (u32, u32),
    pub overwrite_values: bool,
    pub verbose: bool,
}

impl Options {
    pub fn redacted_google_account_credentials(&self) -> String {
        if self.google_account_credentials.is_some() { 
            "...set but redacted...".to_owned()
        } else {
            "none".to_owned()
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            backup: false,
            google_account_credentials: None,
            key_values: collections::HashMap::new(),
            offset: (0, 0),
            overwrite_values: true,
            verbose: false,
        }
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "backup: {}", self.backup)?;
        writeln!(f, "google_account_credentials: {}", self.redacted_google_account_credentials())?;
        writeln!(f, "key_values: {:?}", self.key_values)?;
        writeln!(f, "offset: ({}, {})", self.offset.0, self.offset.1)?;
        writeln!(f, "overwrite_values: {}", self.overwrite_values)?;
        write!(f, "verbose: {}", self.verbose)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let options = Options::default();

        assert_eq!(r#"backup: false
google_account_credentials: none
key_values: {}
offset: (0, 0)
overwrite_values: true
verbose: false"#, options.to_string());
    }
}
