use super::super::CompilationTarget;
use super::GoogleSheets;
use crate::{Result, Template};

impl CompilationTarget for GoogleSheets<'_> {
    fn write_backup(&self) -> Result<()> {
        // TODO note to myself: you use a drive client to do this, not a sheets client
        todo!();
    }

    fn write(&self, template: &Template) -> Result<()> {
        self.async_runtime
            .block_on(async { self.write_sheet(template).await })
    }
}
