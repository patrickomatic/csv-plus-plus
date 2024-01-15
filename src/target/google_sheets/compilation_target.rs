use super::super::CompilationTarget;
use super::GoogleSheets;
use crate::{Module, Result};
use log::info;

impl CompilationTarget for GoogleSheets<'_> {
    fn write_backup(&self) -> Result<()> {
        info!("Making backup of spreadsheet via Google Drive API");
        self.async_runtime
            .block_on(async { self.backup_sheet().await })
    }

    fn write(&self, module: &Module) -> Result<()> {
        info!("Writing compiled spreadsheet to Google Sheets API");
        self.async_runtime
            .block_on(async { self.write_sheet(module).await })
    }
}
