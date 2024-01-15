use super::super::CompilationTarget;
use super::GoogleSheets;
use crate::{Module, Result};

impl CompilationTarget for GoogleSheets<'_> {
    fn write_backup(&self) -> Result<()> {
        self.async_runtime
            .block_on(async { self.backup_sheet().await })
    }

    fn write(&self, module: &Module) -> Result<()> {
        self.async_runtime
            .block_on(async { self.write_sheet(module).await })
    }
}
