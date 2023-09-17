//! # Csv
//!
//! Functions for writing to CSV files
//!
use crate::{Error, Output, Result, Runtime};
use std::path;
use std::ffi;
use std::fs;
use std::io;
use super::{ExistingCell, ExistingValues};

mod compilation_target;

pub struct Csv<'a> {
    path: path::PathBuf,
    runtime: &'a Runtime,
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
                    Ok(ExistingValues { cells: vec![] }),
                error => 
                    Err(Error::TargetWriteError {
                        message: format!("Error reading output: {}", error),
                        output: output.clone(),
                    }),
            },
        };
        let mut reader = csv::ReaderBuilder::new()
            .flexible(true)
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
