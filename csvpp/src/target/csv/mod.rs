//! # Csv
//!
//! Functions for writing to CSV files
//!
use std::cmp;
use std::path;
use std::ffi;
use std::fmt;
use std::fs;
use std::io;

use crate::{Error, OutputTarget, Result, Runtime, SpreadsheetCell, Template};
use super::CompilationTarget;
use super::file_backer_upper;

pub struct Csv<'a> {
    path: path::PathBuf,
    runtime: &'a Runtime,
}

#[derive(Debug)]
pub struct ExistingValue(String);

impl ExistingValue {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn empty_value() -> String {
        "".to_string()
    }

    /*
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    */
}

impl fmt::Display for ExistingValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default)]
pub struct ExistingValues {
    pub cells: Vec<Vec<ExistingValue>>,
}

impl ExistingValues {
    pub fn read(path: &path::PathBuf, target: &OutputTarget) -> Result<Self> {
        let file = match fs::File::open(path) {
            Ok(f) => f,
            Err(e) => return match e.kind() {
                io::ErrorKind::NotFound => 
                    Ok(ExistingValues::default()),
                error => 
                    Err(Error::TargetWriteError {
                        message: format!("Error reading output target: {}", error),
                        target: target.clone(),
                    }),
            },
        };
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(io::BufReader::new(file));

        let mut cells = vec![];
        for result in reader.records() {
            let row = result.or(Err(Error::TargetWriteError {
                message: "Error reading CSV row".to_owned(),
                target: target.clone(),
            }))?;

            cells.push(row.iter().map(ExistingValue::new).collect());
        }
        
        Ok(ExistingValues { cells })
    }
}

impl CompilationTarget for Csv<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(&self.path)?;
        Ok(())
    }

    fn write(&self, template: &Template) -> Result<()> {
        // TODO rather than passing target, let it throw a different error and catch it and attach
        // target
        let existing_values = ExistingValues::read(&self.path, &self.runtime.target)?;
        let new_values = template.spreadsheet.borrow();
        let mut writer = csv::Writer::from_path(&self.path).map_err(|e|
            Error::TargetWriteError {
                message: format!("Unable to open target file for writing: {:?}", e),
                target: self.runtime.target.clone(),
            })?;

        for (index, row) in existing_values.cells.iter().enumerate() {
            writer.write_record(self.merge_rows(row, &new_values.cells[index])).unwrap();
        }

        writer.flush().map_err(|e|
            Error::TargetWriteError {
                message: format!("Unable to finish writing to target: {}", e),
                target: self.runtime.target.clone(),
            })?;

        Ok(())
    }
}

impl<'a> Csv<'a> {
    pub fn new(runtime: &'a Runtime, path: path::PathBuf) -> Self {
        Self { path, runtime }
    }

    pub fn supports_extension(os_str: &ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("csv")
    }

    // TODO make this reusable in some way - maybe generic over existing_row's type
    fn merge_rows(&'a self, existing_row: &'a [ExistingValue], template_row: &[SpreadsheetCell]) -> Vec<String> {
        let overwrite_values = self.runtime.options.overwrite_values;
        let mut row = vec![];

        for i in 0..cmp::max(existing_row.len(), template_row.len()) {
            let new_val = template_row.get(i).map(|v| v.to_string());
            let existing_val = existing_row.get(i).map(|v| v.to_string());

            let merged_value = if existing_val.is_none() {
                new_val
            } else if overwrite_values {
                new_val.or(existing_val)
            } else {
                existing_val.or(new_val)
            };

            row.push(merged_value.unwrap_or(ExistingValue::empty_value()));
        }

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_extension_true() {
        assert!(Csv::supports_extension(ffi::OsStr::new("csv")));
        assert!(Csv::supports_extension(ffi::OsStr::new("CSV")));
    }

    #[test]
    fn supports_extension_false() {
        assert!(!Csv::supports_extension(ffi::OsStr::new("foo")));
        assert!(!Csv::supports_extension(ffi::OsStr::new("XLSX")));
    }
}
