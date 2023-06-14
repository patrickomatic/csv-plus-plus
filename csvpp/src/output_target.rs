//! # OutputTarget
//!
use std::fmt;
use std::path::PathBuf;

use crate::{CliArgs, Error, Result};

type GoogleSheetID = String;

#[derive(Clone, Debug, PartialEq)]
pub enum OutputTarget {
    File(PathBuf),
    GoogleSheets(GoogleSheetID),
}

impl OutputTarget {
    pub fn from_cli_args(cli_args: &CliArgs) -> Result<Self> {
        let output = if cli_args.google_sheet_id.is_some() {
            OutputTarget::GoogleSheets(cli_args.google_sheet_id.clone().unwrap().to_string())
        } else if cli_args.output_filename.is_some() {
            OutputTarget::File(cli_args.output_filename.clone().unwrap())
        } else {
            return Err(Error::InitError(
                    "Must specify either -g/--google-sheet-id or -o/--output-filename".to_string()))
        };

        output.validate()?;

        Ok(output)
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::File(f) => self.validate_file(f),        
            Self::GoogleSheets(sheet_id) => self.validate_google_sheets(sheet_id),        
        }
    }

    fn validate_google_sheets(&self, sheet_id: &GoogleSheetID) -> Result<()> {
        if sheet_id.chars().all(char::is_alphanumeric) {
            Ok(())
        } else {
            Err(Error::InitError("The GOOGLE_SHEET_ID must be all letters and digits.".to_string()))
        }
    }
    
    fn validate_file(&self, path: &PathBuf) -> Result<()> {
        if let Some(ext) = path.extension() {
            if ext.eq_ignore_ascii_case("csv") || ext.eq_ignore_ascii_case("xlsx") || ext.eq_ignore_ascii_case("ods") {
                return Ok(())
            } else {
                Err(Error::InitError(
                    format!("{} is an unsupported extension: only .csv, .xlsx or .ods are supported.", 
                            ext.to_str().unwrap())))
            }
        } else {
            Err(Error::InitError("Output filename must end with .csv, .xlsx or .ods".to_string()))
        }
    }
}

impl fmt::Display for OutputTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GoogleSheets(id) => write!(f, "Google Sheets[{}]", id),
            Self::File(path) => write!(f, "{}", path.to_str().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn from_cli_args() {
        // TODO
    }

    #[test]
    fn from_cli_args_invalid() {
        // TODO
    }

    #[test]
    fn validate_google_sheets_valid() {
        assert!(OutputTarget::GoogleSheets("abc123".to_string()).validate().is_ok());
    }
    
    #[test]
    fn validate_google_sheets_invalid() {
        assert!(OutputTarget::GoogleSheets("abc #!*)! 123".to_string()).validate().is_err());
    }

    #[test]
    fn validate_file_csv() {
        assert!(OutputTarget::File(PathBuf::from("/foo/bar/file.csv")).validate().is_ok());
    }

    #[test]
    fn validate_file_xlsx() {
        assert!(OutputTarget::File(PathBuf::from("FileName.xlsx")).validate().is_ok());
    }

    #[test]
    fn validate_file_ods() {
        assert!(OutputTarget::File(PathBuf::from("TEST.ODS")).validate().is_ok());
    }

    #[test]
    fn validate_file_invalid() {
        assert!(OutputTarget::File(PathBuf::from("not_a_valid_file")).validate().is_err());
    }
}

