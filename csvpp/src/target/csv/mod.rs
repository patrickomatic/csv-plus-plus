//! # Csv
//!
//! Functions for writing to CSV files
//!
use std::path;
use std::ffi;
use std::fs;
use std::io;

use crate::{Error, Output, Result, Runtime, Template};
use super::{CompilationTarget, ExistingCell, ExistingValues, MergeResult};
use super::{file_backer_upper, merge_rows};

pub struct Csv<'a> {
    path: path::PathBuf,
    runtime: &'a Runtime,
}

impl CompilationTarget for Csv<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(&self.path)?;
        Ok(())
    }

    fn write(&self, template: &Template) -> Result<()> {
        // TODO rather than passing target, let it throw a different error and catch it and attach
        // target
        let existing_values = Self::read(&self.path, &self.runtime.output)?;
        let new_values = template.spreadsheet.borrow();
        let mut writer = csv::Writer::from_path(&self.path).map_err(|e|
            Error::TargetWriteError {
                message: format!("Unable to open output file for writing: {:?}", e),
                output: self.runtime.output.clone(),
            })?;

        for (index, row) in new_values.cells.iter().enumerate() {
            // let empty = vec![];
            let output_row: Vec<String> = merge_rows(
                    existing_values.cells.get(index).unwrap_or(&vec![].to_owned()), 
                    row, 
                    &self.runtime.options,
                )
                .iter()
                .map(|cell| {
                    match cell {
                        MergeResult::New(v) => v.to_string(),
                        MergeResult::Existing(v) => v.to_string(),
                        MergeResult::Empty => "".to_owned(),
                    }
                })
                .collect();
            
            writer.write_record(output_row).unwrap();
        }

        writer.flush().map_err(|e|
            Error::TargetWriteError {
                message: format!("Unable to finish writing to output: {}", e),
                output: self.runtime.output.clone(),
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

    fn read(path: &path::PathBuf, output: &Output) -> Result<ExistingValues<String>> {
        let file = match fs::File::open(path) {
            Ok(f) => f,
            Err(e) => return match e.kind() {
                io::ErrorKind::NotFound => 
                    Ok(ExistingValues::default()),
                error => 
                    Err(Error::TargetWriteError {
                        message: format!("Error reading output: {}", error),
                        output: output.clone(),
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
                output: output.clone(),
            }))?;
            let existing_row = row
                .iter()
                .map(|cell| {
                    if cell.is_empty() {
                        ExistingCell::Empty
                    } else {
                        ExistingCell::Value(cell.to_string())
                    }
                })
                .collect();

            cells.push(existing_row);
        }
        Ok(ExistingValues { cells })
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

    // TODO more tests
}
