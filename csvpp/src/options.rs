use std::collections;
use std::fmt;
use crate::{CliArgs, Result};
use crate::ast::Ast;
use crate::compiler::ast_parser::AstParser;

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
    pub fn from_cli_args(cli_args: &CliArgs) -> Result<Self> {
        let key_values_as_str = cli_args
            .key_values
            .iter()
            .map(|s| s.as_str())
            .collect();

        Ok(Options {
            backup: cli_args.backup,
            google_account_credentials: cli_args.google_account_credentials.clone(),
            key_values: AstParser::parse_key_value_str(key_values_as_str)?,
            offset: (cli_args.x_offset, cli_args.y_offset),
            overwrite_values: !cli_args.safe,
            verbose: cli_args.verbose,
        })
    }
    
    pub fn redacted_google_account_credentials(&self) -> String {
        if self.google_account_credentials.is_some() { 
            "...redacted...".to_owned()
        } else {
            "none".to_string()
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
    use crate::CliArgs;
    use super::*;

    #[test]
    fn default() {
        let options = Options::default();

        assert_eq!(options.backup, false);
        assert_eq!(options.google_account_credentials, None);
        assert_eq!(options.key_values, collections::HashMap::new());
        assert_eq!(options.offset, (0, 0));
        assert_eq!(options.overwrite_values, true);
        assert_eq!(options.verbose, false);
    }

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

    #[test]
    fn from_cli_args() {
        let cli_args = CliArgs::default();
        let options = Options::from_cli_args(&cli_args);

        assert!(options.is_ok());
    }
}
