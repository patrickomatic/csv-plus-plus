//! # OpenDocument
//!
//! Functions for writing to OpenDocument files
//!
use std::path::PathBuf;

use crate::{Options, Result, Template};
use super::CompilationTarget;

pub struct OpenDocument {
    path: PathBuf,
}

impl CompilationTarget for OpenDocument {
    fn write_backup(&self) -> Result<()> {
        todo!()
    }

    fn write(&self, _options: &Options, _template: &Template) -> Result<()> {
        todo!()
    }
}

impl OpenDocument {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn supports_extension(os_str: &std::ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("ods")
    }
}
