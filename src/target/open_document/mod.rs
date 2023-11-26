//! # OpenDocument
//!
//! Functions for writing to OpenDocument files
//!
use super::file_backer_upper;
use super::CompilationTarget;
use crate::{Compiler, Module, Result};
use std::path::PathBuf;

pub struct OpenDocument<'a> {
    path: PathBuf,
    compiler: &'a Compiler,
}

impl CompilationTarget for OpenDocument<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(self.compiler, &self.path)?;
        Ok(())
    }

    fn write(&self, _module: &Module) -> Result<()> {
        todo!()
    }
}

impl<'a> OpenDocument<'a> {
    pub fn new(compiler: &'a Compiler, path: PathBuf) -> OpenDocument<'a> {
        Self { path, compiler }
    }

    pub(crate) fn supports_extension(os_str: &std::ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("ods")
    }
}
