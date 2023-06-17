//! # OutputTarget
//!
use std::fmt;
use std::path::PathBuf;

use crate::{CliArgs, Error, Result, CompilerTarget};
use crate::target;

type GoogleSheetID = String;

#[derive(Clone, Debug, PartialEq)]
pub enum OutputTarget {
    Csv(PathBuf),
    Excel(PathBuf),
    OpenDocument(PathBuf),
    GoogleSheets(GoogleSheetID),
}

impl OutputTarget {
    pub fn from_cli_args(cli_args: &CliArgs) -> Result<Self> {
        if let Some(sheet_id) = &cli_args.google_sheet_id {
            Ok(Self::from_google_sheet_id(sheet_id.to_string()))?
        } else if let Some(filename) = &cli_args.output_filename {
            Ok(Self::from_filename(filename.to_path_buf()))?
        } else {
            Err(Error::InitError(
                    "Must specify either -g/--google-sheet-id or -o/--output-filename".to_string()))
        }
    }

    pub fn compiler_target(&self) -> Box<dyn CompilerTarget> {
        match self {
            Self::Csv(path) => 
                Box::new(target::Csv::new(path.to_path_buf())),
            Self::Excel(path) => 
                Box::new(target::Excel::new(path.to_path_buf())),
            Self::GoogleSheets(sheet_id) =>
                Box::new(target::GoogleSheets::new(sheet_id.clone())),
            Self::OpenDocument(path) =>
                Box::new(target::OpenDocument::new(path.to_path_buf())),
        }
    }

    fn from_filename(path: PathBuf) -> Result<Self> {
        match path.extension() {
            Some(ext) => {
                if target::Csv::supports_extension(ext) {
                    Ok(Self::Csv(path))
                } else if target::Excel::supports_extension(ext) {
                    Ok(Self::Excel(path))
                } else if target::OpenDocument::supports_extension(ext) {
                    Ok(Self::OpenDocument(path))
                } else {
                    Err(Error::InitError(
                        format!("{} is an unsupported extension: only .csv, .xlsx or .ods are supported.", 
                                ext.to_str().unwrap())))
                }
            },
            None => Err(Error::InitError("Output filename must end with .csv, .xlsx or .ods".to_string()))
        }
    }

    fn from_google_sheet_id(sheet_id: String) -> Result<Self> {
        if sheet_id.chars().all(char::is_alphanumeric) {
            Ok(Self::GoogleSheets(sheet_id))
        } else {
            Err(Error::InitError("The GOOGLE_SHEET_ID must be all letters and digits.".to_string()))
        }
    }
}

impl fmt::Display for OutputTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GoogleSheets(id) => 
                write!(f, "Google Sheets[{}]", id),
            Self::Csv(path) | Self::Excel(path) | Self::OpenDocument(path) => 
                write!(f, "{}", path.to_str().unwrap()),
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

    /*
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
    */
}

