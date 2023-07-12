//! # OutputTarget
//!
use std::fmt;
use std::path;
use crate::{CliArgs, CompilationTarget, Error, Result, Runtime};
use crate::target;

type GoogleSheetID = String;

#[derive(Clone, Debug, PartialEq)]
pub enum OutputTarget {
    Csv(path::PathBuf),
    Excel(path::PathBuf),
    OpenDocument(path::PathBuf),
    GoogleSheets(GoogleSheetID),
}

impl OutputTarget {
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
    fn from_cli_args_csv() {
        let mut cli_args = CliArgs::default();
        cli_args.output_filename = Some(PathBuf::from("foo.csv"));

        let output_target = OutputTarget::from_cli_args(&cli_args).unwrap();
        assert_eq!(output_target, OutputTarget::Csv(PathBuf::from("foo.csv")))
    }

    #[test]
    fn from_cli_args_excel() {
        let mut cli_args = CliArgs::default();
        cli_args.output_filename = Some(PathBuf::from("foo.xlsx"));

        let output_target = OutputTarget::from_cli_args(&cli_args).unwrap();
        assert_eq!(output_target, OutputTarget::Excel(PathBuf::from("foo.xlsx")))
    }

    #[test]
    fn from_cli_args_google_sheets() {
        let mut cli_args = CliArgs::default();
        cli_args.google_sheet_id = Some("abc".to_string());

        let output_target = OutputTarget::from_cli_args(&cli_args).unwrap();
        assert_eq!(output_target, OutputTarget::GoogleSheets("abc".to_string()));
    }

    #[test]
    fn from_cli_args_open_document() {
        let mut cli_args = CliArgs::default();
        cli_args.output_filename = Some(PathBuf::from("foo.ods"));

        let output_target = OutputTarget::from_cli_args(&cli_args).unwrap();
        assert_eq!(output_target, OutputTarget::OpenDocument(PathBuf::from("foo.ods")))
    }

    #[test]
    fn from_cli_args_invalid() {
        let cli_args = CliArgs::default();

        let output_target = OutputTarget::from_cli_args(&cli_args);
        assert!(output_target.is_err());
    }
}
