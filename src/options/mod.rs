//! # Options
//!
//! The options that a user can supply to the compiler.
//!
use crate::ast::Ast;
use crate::{CliArgs, Error, Result};
use std::collections;

mod default;
mod display;
mod try_from;

#[derive(Debug)]
pub struct Options {
    pub backup: bool,
    pub google_account_credentials: Option<String>,
    pub key_values: collections::HashMap<String, Ast>,
    pub offset: (u32, u32),
    pub overwrite_values: bool,
    pub sheet_name: String,
    pub use_cache: bool,
    pub verbose: bool,
}

impl Options {
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
                None => Err(Error::InitError(format!(
                    "Could not determine base filename from input filename: {}",
                    cli_args.input_filename.display()
                ))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CliArgs;
    use std::path;

    #[test]
    fn from_cli_args_no_sheet_name() {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            ..Default::default()
        };
        let options = Options::try_from(&cli_args).unwrap();

        assert_eq!(options.sheet_name, "foo");
    }

    #[test]
    fn from_cli_args_sheet_name() {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            sheet_name: Some("bar".to_string()),
            ..Default::default()
        };
        let options = Options::try_from(&cli_args).unwrap();

        assert_eq!(options.sheet_name, "bar");
    }
}
