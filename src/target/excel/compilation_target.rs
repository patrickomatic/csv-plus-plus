//!
use super::super::{file_backer_upper, CompilationTarget};
use super::Excel;
use crate::{Module, Result};
use umya_spreadsheet as u;

impl CompilationTarget for Excel<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(self.compiler, &self.path)?;
        Ok(())
    }

    fn write(&self, module: &Module) -> Result<()> {
        let mut spreadsheet = self.open_spreadsheet()?;

        self.create_worksheet(&mut spreadsheet)?;

        // TODO: it would be nice to just return the worksheet rather than having a separate method
        // to get it but I couldn't get the mutable references to work out.
        let worksheet = self.get_worksheet_mut(&mut spreadsheet)?;

        self.build_worksheet(module, worksheet);

        u::writer::xlsx::write(&spreadsheet, self.path.clone()).map_err(|e| {
            self.compiler.output_error(format!(
                "Unable to write target file {}: {e}",
                self.path.display()
            ))
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn write() {
        let test_file = &TestSourceCode::new("xlsx", "foo,bar,baz");
        let compiler = test_file.into();
        let target = Excel::new(&compiler, test_file.output_file.clone());
        let module = compiler.compile().unwrap();

        assert!(target.write(&module).is_ok());
    }
}
