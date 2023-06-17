//! # GoogleSheets
// use sheets4::api::
use crate::{Options, Result, Template};
use super::CompilerTarget;

pub struct GoogleSheets {
    pub sheet_id: String,
}

impl CompilerTarget for GoogleSheets {
    fn write(&self, options: &Options, template: &Template) -> Result<()> {
        todo!();
    }
}

impl GoogleSheets {
    pub fn new(sheet_id: String) -> Self {
        Self { sheet_id }
    }
}
