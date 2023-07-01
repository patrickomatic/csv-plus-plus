//! # Excel
//!
//! Functions for writing compiled templates to Excel
//!
use std::ffi;
use std::path;

use crate::{Result, Runtime, Template};
use super::file_backer_upper;
use super::CompilationTarget;

pub struct Excel<'a> {
    path: path::PathBuf,
    runtime: &'a Runtime,
}

impl CompilationTarget for Excel<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(&self.path)?;
        Ok(())
    }

    fn write(&self, _template: &Template) -> Result<()> {
        todo!();
    }
}

impl<'a> Excel<'a> {
    pub fn new(runtime: &'a Runtime, path: path::PathBuf) -> Self {
        Self { path, runtime }
    }

    pub fn supports_extension(os_str: &ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("xlsx")
            || os_str.eq_ignore_ascii_case("xlsm")
            || os_str.eq_ignore_ascii_case("xltx")
            || os_str.eq_ignore_ascii_case("xltm")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_extension_true() {
        assert!(Excel::supports_extension(ffi::OsStr::new("xlsx")));
        assert!(Excel::supports_extension(ffi::OsStr::new("XLSX")));
        assert!(Excel::supports_extension(ffi::OsStr::new("xlsm")));
        assert!(Excel::supports_extension(ffi::OsStr::new("xltm")));
        assert!(Excel::supports_extension(ffi::OsStr::new("xltx")));
    }

    #[test]
    fn supports_extension_false() {
        assert!(!Excel::supports_extension(ffi::OsStr::new("foo")));
        assert!(!Excel::supports_extension(ffi::OsStr::new("csv")));
    }
}
