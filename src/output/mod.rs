//! # Output
//!
use crate::target;
use crate::{CompilationTarget, Error, Result, Runtime};
use std::path;

mod display;
mod try_from;

type GoogleSheetID = String;

#[derive(Clone, Debug, PartialEq)]
pub enum Output {
    Csv(path::PathBuf),
    Excel(path::PathBuf),
    OpenDocument(path::PathBuf),
    GoogleSheets(GoogleSheetID),
}

impl Output {
    pub fn compilation_target<'a>(
        &'a self,
        runtime: &'a Runtime,
    ) -> Result<Box<dyn CompilationTarget + 'a>> {
        Ok(match self {
            Self::Csv(path) => Box::new(target::Csv::new(runtime, path.to_path_buf())),
            Self::Excel(path) => Box::new(target::Excel::new(runtime, path.to_path_buf())),
            Self::GoogleSheets(sheet_id) => Box::new(target::GoogleSheets::new(runtime, sheet_id)?),
            Self::OpenDocument(path) => Box::new(target::OpenDocument::new(path.to_path_buf())),
        })
    }

    fn from_filename(path: path::PathBuf) -> Result<Self> {
        let ext = path.extension().ok_or(Error::InitError(
            "Output filename must end with .csv, .xlsx or .ods".to_string(),
        ))?;

        if target::Csv::supports_extension(ext) {
            Ok(Self::Csv(path))
        } else if target::Excel::supports_extension(ext) {
            Ok(Self::Excel(path))
        } else if target::OpenDocument::supports_extension(ext) {
            Ok(Self::OpenDocument(path))
        } else {
            Err(Error::InitError(format!(
                "{} is an unsupported extension: only .csv, .xlsx or .ods are supported.",
                ext.to_str().unwrap()
            )))
        }
    }

    fn from_google_sheet_id(sheet_id: String) -> Result<Self> {
        if sheet_id.chars().all(char::is_alphanumeric) {
            Ok(Self::GoogleSheets(sheet_id))
        } else {
            Err(Error::InitError(
                "The GOOGLE_SHEET_ID must be all letters and digits.".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO
}
