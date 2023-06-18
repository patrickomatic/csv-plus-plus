//! # GoogleSheets
// use sheets4::api::
use crate::{Options, Result, Template};
use super::CompilationTarget;

pub struct GoogleSheets {
    pub sheet_id: String,
}

impl CompilationTarget for GoogleSheets {
    fn write_backup(&self) -> Result<()> {
        todo!();
    }

    fn write(&self, options: &Options, template: &Template) -> Result<()> {
        todo!();
    }
}

impl GoogleSheets {
    pub fn new(sheet_id: String) -> Self {
        Self { sheet_id }
    }
}
