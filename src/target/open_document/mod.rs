//! # OpenDocument
//!
//! Functions for writing to OpenDocument files
//!
use std::path::PathBuf;

use super::file_backer_upper;
use super::CompilationTarget;
use crate::{Result, Template};

pub struct OpenDocument {
    path: PathBuf,
}

impl CompilationTarget for OpenDocument {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(&self.path)?;
        Ok(())
    }

    fn write(&self, _template: &Template) -> Result<()> {
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
