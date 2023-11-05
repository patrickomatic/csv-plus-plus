//! # OpenDocument
//!
//! Functions for writing to OpenDocument files
//!
use super::file_backer_upper;
use super::CompilationTarget;
use crate::{Result, Runtime, Template};
use std::path::PathBuf;

pub struct OpenDocument<'a> {
    path: PathBuf,
    runtime: &'a Runtime,
}

impl CompilationTarget for OpenDocument<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(self.runtime, &self.path)?;
        Ok(())
    }

    fn write(&self, _template: &Template) -> Result<()> {
        todo!()
    }
}

impl<'a> OpenDocument<'a> {
    pub fn new(runtime: &'a Runtime, path: PathBuf) -> OpenDocument<'a> {
        Self { path, runtime }
    }

    pub(crate) fn supports_extension(os_str: &std::ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("ods")
    }
}
