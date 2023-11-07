//! # FileBackerUpper
use crate::{Result, Runtime};
use chrono::prelude::Local;
use std::fs;
use std::path;

const BACKUP_FORMATS: &[&str] = &[
    // filename.csv -> filename-2023-04-25.csv
    "%Y_%m_%d",
    // filename.csv -> filename-2023-04-25-1_00AM.csv
    "%Y_%m_%d-%I_%M%p",
    // filename.csv -> filename-2023-04-25-1_00_00AM.csv
    "%Y_%m_%d-%I_%M_%S%p",
    // filename.csv -> filename-2023-04-25-1_00_00_0000AM.csv
    "%Y_%m_%d-%I_%M_%S_%f%p",
];

/// Makes a copy of a file like foo.xlsx to foo-2023-04-25.xlsx
///
// NOTE:
// this operation is not technically atomic - to do so we'd need to create a tempfile, write to it
// then move it in place.  (but for this use case I don't think it matters)
pub(crate) fn backup_file(runtime: &Runtime, filename: &path::PathBuf) -> Result<path::PathBuf> {
    runtime.progress(format!("Backing up file: {}", filename.display()));

    let now = Local::now();

    let filename_str = filename.to_str().ok_or(
        runtime
            .output
            .clone()
            .into_error("Unable to format output filename"),
    )?;

    let file_stem = filename.file_stem().ok_or(
        runtime
            .output
            .clone()
            .into_error(format!("Unable to get base file for: {filename_str}",)),
    )?;

    let file_parent = filename
        .parent()
        .ok_or(runtime.output.clone().into_error(format!(
            "Unable to get parent base file for: {filename_str}",
        )))?;

    let file_extension = filename.extension().ok_or(
        runtime
            .output
            .clone()
            .into_error(format!("Unable to get extension for: {filename_str}",)),
    )?;

    for time_format in BACKUP_FORMATS.iter() {
        let timestamp = now.format(time_format);

        let mut new_file = file_parent.to_path_buf();
        new_file.push(format!("{}-{timestamp}", file_stem.to_str().unwrap()));
        new_file.set_extension(file_extension);

        if new_file.exists() {
            continue;
        }

        if let Err(e) = fs::copy(filename, &new_file) {
            return Err(runtime
                .output
                .clone()
                .into_error(format!("Error making backup of {filename_str}: {e}",)));
        }

        return Ok(new_file);
    }

    Err(runtime.output.clone().into_error(format!(
        "Unable to make backup of output file: {filename_str}",
    )))
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn backup_file() {
        // TODO
    }
}
