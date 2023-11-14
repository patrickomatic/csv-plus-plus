//!
use super::super::{file_backer_upper, CompilationTarget};
use super::Excel;
use crate::{Result, Template};
use umya_spreadsheet as u;

impl CompilationTarget for Excel<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(self.runtime, &self.path)?;
        Ok(())
    }

    fn write(&self, template: &Template) -> Result<()> {
        let mut spreadsheet = self.open_spreadsheet()?;

        self.create_worksheet(&mut spreadsheet)?;

        // TODO: it would be nice to just return the worksheet rather than having a separate method
        // to get it but I couldn't get the mutable references to work out.
        let worksheet = self.get_worksheet_mut(&mut spreadsheet)?;

        self.build_worksheet(template, worksheet)?;

        u::writer::xlsx::write(&spreadsheet, self.path.clone()).map_err(|e| {
            self.runtime.output_error(format!(
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
        let runtime = test_file.into();
        let target = Excel::new(&runtime, test_file.output_file.clone());
        let template = Template::compile(&runtime).unwrap();

        assert!(target.write(&template).is_ok());
    }
}
