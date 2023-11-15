use super::super::CompilationTarget;
use super::GoogleSheets;
use crate::{Module, Result};

impl CompilationTarget for GoogleSheets<'_> {
    fn write_backup(&self) -> Result<()> {
        // TODO note to myself: you use a drive client to do this, not a sheets client
        todo!();
    }

    fn write(&self, module: &Module) -> Result<()> {
        self.async_runtime
            .block_on(async { self.write_sheet(module).await })
    }
}
