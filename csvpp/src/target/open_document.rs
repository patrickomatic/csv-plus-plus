//! # OpenDocument
//!
//! Functions for writing to OpenDocument files
//!
use std::path::PathBuf;

use crate::{Options, Result, Template};
use super::CompilerTarget;

pub struct OpenDocument {
    path: PathBuf,
}

impl CompilerTarget for OpenDocument {
    fn write(&self, options: &Options, template: &Template) -> Result<()> {
        // TODO
        Ok(())
    }
}

impl OpenDocument {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
        }
    }

    pub fn supports_extension(os_str: &std::ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("ods")
    }
}
