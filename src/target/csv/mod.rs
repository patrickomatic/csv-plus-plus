//! # Csv
//!
//! Functions for writing to CSV files
//!
use super::{ExistingCell, ExistingValues};
use crate::{csv_reader, Output, Result, Runtime};
use std::ffi;
use std::fs;
use std::io;
use std::path;

mod compilation_target;

pub(crate) struct Csv<'a> {
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
            Err(e) => {
                return match e.kind() {
                    io::ErrorKind::NotFound => Ok(ExistingValues { cells: vec![] }),
                    error => Err(output
                        .clone()
                        .into_error(format!("Error reading output: {error}"))),
                }
            }
        };

        let mut reader = csv_reader().from_reader(io::BufReader::new(file));

        let mut cells = vec![];
        for result in reader.records() {
            let row = result.or(Err(output.clone().into_error("Error reading CSV row")))?;
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
