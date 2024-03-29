//! # Output
//!
use crate::target::{Csv, Excel, GoogleSheets, OpenDocument};
use crate::{CompilationTarget, Compiler, Error, Result};
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
    pub(crate) fn compilation_target<'a>(
        &'a self,
        compiler: &'a Compiler,
    ) -> Result<Box<dyn CompilationTarget + 'a>> {
        Ok(match self {
            Self::Csv(path) => Box::new(Csv::new(compiler, path)),
            Self::Excel(path) => Box::new(Excel::new(compiler, path)),
            Self::GoogleSheets(sheet_id) => Box::new(GoogleSheets::new(compiler, sheet_id)?),
            Self::OpenDocument(path) => Box::new(OpenDocument::new(compiler, path)),
        })
    }

    pub(crate) fn into_error<M: Into<String>>(self, message: M) -> Error {
        Error::TargetWriteError {
            message: message.into(),
            output: self,
        }
    }

    fn from_filename<P>(path: &P) -> Result<Self>
    where
        P: AsRef<path::Path> + Into<path::PathBuf>,
    {
        let path = path.as_ref();
        let ext = path.extension().ok_or(Error::InitError(
            "Output filename must end with .csv, .xlsx or .ods".to_string(),
        ))?;

        let path = path.into();
        if Csv::supports_extension(ext) {
            Ok(Self::Csv(path))
        } else if Excel::supports_extension(ext) {
            Ok(Self::Excel(path))
        } else if OpenDocument::supports_extension(ext) {
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
