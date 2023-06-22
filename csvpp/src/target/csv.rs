//! # Csv
//!
//! Functions for writing to CSV files
//!
use csv::WriterBuilder;
use std::path::PathBuf;
use std::ffi::OsStr;

use crate::{Options, Result, Template};
use super::CompilationTarget;

pub struct Csv {
    path: PathBuf,
    builder: WriterBuilder,
}

impl CompilationTarget for Csv {
    fn write_backup(&self) -> Result<()> {
        todo!()
    }

    fn write(&self, _options: &Options, _template: &Template) -> Result<()> {
        todo!()
    }
}

impl Csv {
    pub fn new(path: PathBuf) -> Self {
        let builder = WriterBuilder::new();

        Self { builder, path }
    }

    pub fn supports_extension(os_str: &OsStr) -> bool {
        os_str.eq_ignore_ascii_case("csv")
    }
}
