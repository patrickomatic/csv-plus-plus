//! # Options
//!
//! The options that a user can supply to the compiler.
//!
use std::collections;
use std::fmt;
use crate::{CliArgs, Error, Result};
use crate::ast::Ast;
use crate::compiler::ast_parser::AstParser;

#[derive(Debug)]
pub struct Options {
    pub backup: bool,
    pub google_account_credentials: Option<String>,
    pub key_values: collections::HashMap<String, Ast>,
    pub offset: (u32, u32),
    pub overwrite_values: bool,
    pub sheet_name: String,
    pub verbose: bool,
}

impl Options {
    // TODO: implement this with the From trait? but it throws using ?
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
            sheet_name: Self::sheet_name(cli_args)?,
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

    fn sheet_name(cli_args: &CliArgs) -> Result<String> {
        if let Some(sheet_name) = &cli_args.sheet_name {
            Ok(sheet_name.clone())
        } else {
            match cli_args.input_filename.file_stem() {
                Some(fs) => Ok(fs.to_string_lossy().to_string()),
                None => Err(Error::InitError(
                    format!(
                        "Could not determine base filename from input filename: {}",
                        cli_args.input_filename.display()))),
            }
        }
    }
}

// TODO: this is only used for tests, maybe we don't need it
impl Default for Options {
    fn default() -> Self {
        Self {
            backup: false,
            google_account_credentials: None,
            key_values: collections::HashMap::new(),
            offset: (0, 0),
            overwrite_values: true,
            sheet_name: "empty".to_string(),
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
        writeln!(f, "sheet_name: {}", self.sheet_name)?;
        write!(f, "verbose: {}", self.verbose)
    }
}

#[cfg(test)]
mod tests {
    use std::path;
    use crate::CliArgs;
    use super::*;

    #[test]
    fn default() {
        let options = Options::default();

        assert!(!options.backup);
        assert!(options.overwrite_values);
        assert!(!options.verbose);
        assert_eq!(options.google_account_credentials, None);
        assert_eq!(options.key_values, collections::HashMap::new());
        assert_eq!(options.offset, (0, 0));
    }

    #[test]
    fn display() {
        let options = Options::default();

        assert_eq!(r#"backup: false
google_account_credentials: none
key_values: {}
offset: (0, 0)
overwrite_values: true
sheet_name: empty
verbose: false"#, options.to_string());
    }

    #[test]
    fn from_cli_args_no_sheet_name() {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            ..Default::default()
        };
        let options = Options::from_cli_args(&cli_args).unwrap();

        assert_eq!(options.sheet_name, "foo");
    }

    #[test]
    fn from_cli_args_sheet_name() {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            sheet_name: Some("bar".to_string()),
            ..Default::default()
        };
        let options = Options::from_cli_args(&cli_args).unwrap();

        assert_eq!(options.sheet_name, "bar");
    }
}
