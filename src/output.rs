//! # Output
//!
use std::fmt;
use std::path;
use crate::{CliArgs, CompilationTarget, Error, Result, Runtime};
use crate::target;

type GoogleSheetID = String;

#[derive(Clone, Debug, PartialEq)]
pub enum Output {
    Csv(path::PathBuf),
    Excel(path::PathBuf),
    OpenDocument(path::PathBuf),
    GoogleSheets(GoogleSheetID),
}

impl Output {
    pub fn from_cli_args(cli_args: &CliArgs) -> Result<Self> {
        if let Some(sheet_id) = &cli_args.google_sheet_id {
            Ok(Self::from_google_sheet_id(sheet_id.to_string())?)
        } else if let Some(filename) = &cli_args.output_filename {
            Ok(Self::from_filename(filename.to_path_buf())?)
        } else {
            Err(Error::InitError(
                    "Must specify either -g/--google-sheet-id or -o/--output-filename".to_string()))
        }
    }

    pub fn compilation_target<'a>(&'a self, runtime: &'a Runtime) -> Result<Box<dyn CompilationTarget + 'a>> {
        Ok(match self {
            Self::Csv(path) => 
                Box::new(target::Csv::new(runtime, path.to_path_buf())),
            Self::Excel(path) => 
                Box::new(target::Excel::new(runtime, path.to_path_buf())),
            Self::GoogleSheets(sheet_id) =>
                Box::new(target::GoogleSheets::new(runtime, sheet_id)?),
            Self::OpenDocument(path) =>
                Box::new(target::OpenDocument::new(path.to_path_buf())),
        })
    }

    fn from_filename(path: path::PathBuf) -> Result<Self> {
        let ext = path.extension().ok_or(
            Error::InitError("Output filename must end with .csv, .xlsx or .ods".to_string()))?;

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
    }

    fn from_google_sheet_id(sheet_id: String) -> Result<Self> {
        if sheet_id.chars().all(char::is_alphanumeric) {
            Ok(Self::GoogleSheets(sheet_id))
        } else {
            Err(Error::InitError("The GOOGLE_SHEET_ID must be all letters and digits.".to_string()))
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GoogleSheets(id) => write!(f, "Google Sheets: {id}"),
            Self::Csv(path) => write!(f, "CSV: {}", path.display()),
            Self::Excel(path) => write!(f, "Excel: {}", path.display()),
            Self::OpenDocument(path) => write!(f, "OpenDocument: {}", path.display()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn from_cli_args_csv() {
        let cli_args = CliArgs {
            output_filename: Some(PathBuf::from("foo.csv")),
            ..Default::default()
        };
        let output_target = Output::from_cli_args(&cli_args).unwrap();

        assert_eq!(output_target, Output::Csv(PathBuf::from("foo.csv")))
    }

    #[test]
    fn from_cli_args_excel() {
        let cli_args = CliArgs {
            output_filename: Some(PathBuf::from("foo.xlsx")),
            ..Default::default()
        };
        let output_target = Output::from_cli_args(&cli_args).unwrap();

        assert_eq!(output_target, Output::Excel(PathBuf::from("foo.xlsx")))
    }

    #[test]
    fn from_cli_args_google_sheets() {
        let cli_args = CliArgs {
            google_sheet_id: Some("abc".to_string()),
            ..Default::default()
        };
        let output_target = Output::from_cli_args(&cli_args).unwrap();

        assert_eq!(output_target, Output::GoogleSheets("abc".to_string()));
    }

    #[test]
    fn from_cli_args_open_document() {
        let cli_args = CliArgs {
            output_filename: Some(PathBuf::from("foo.ods")),
            ..Default::default()
        };
        let output_target = Output::from_cli_args(&cli_args).unwrap();

        assert_eq!(output_target, Output::OpenDocument(PathBuf::from("foo.ods")))
    }

    #[test]
    fn from_cli_args_invalid() {
        let cli_args = CliArgs::default();
        let output_target = Output::from_cli_args(&cli_args);

        assert!(output_target.is_err());
    }
}
