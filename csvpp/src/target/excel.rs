//! # Excel
//!
//! Functions for writing compiled templates to Excel
//!
use std::path::PathBuf;

use crate::{Options, Result, Template};
use super::CompilationTarget;

pub struct Excel {
    path: PathBuf,
}

impl CompilationTarget for Excel {
    fn write_backup(&self) -> Result<()> {
        todo!();
    }

    fn write(&self, _options: &Options, _template: &Template) -> Result<()> {
        todo!();
    }
}

impl Excel {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn supports_extension(os_str: &std::ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("xlsx")
            || os_str.eq_ignore_ascii_case("xlsm")
            || os_str.eq_ignore_ascii_case("xltx")
            || os_str.eq_ignore_ascii_case("xltm")
    }
}
