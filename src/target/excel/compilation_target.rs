//!
use super::super::{file_backer_upper, CompilationTarget};
use super::Excel;
use crate::{Error, Result, Template};

impl CompilationTarget for Excel<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(&self.path)?;
        Ok(())
    }

    fn write(&self, template: &Template) -> Result<()> {
        let mut spreadsheet = self.open_spreadsheet()?;

        self.create_worksheet(&mut spreadsheet)?;

        // TODO: it would be nice to just return the worksheet rather than having a separate method
        // to get it but I couldn't get the mutable references to work out.
        let worksheet = self.get_worksheet_mut(&mut spreadsheet)?;

        self.build_worksheet(template, worksheet)?;

        umya_spreadsheet::writer::xlsx::write(&spreadsheet, self.path.clone()).map_err(|e| {
            Error::TargetWriteError {
                message: format!("Unable to write target file {}: {}", self.path.display(), e),
                output: self.runtime.output.clone(),
            }
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestFile;

    #[test]
    fn write() {
        let test_file = TestFile::new("xlsx", "foo,bar,baz");
        let runtime = test_file.clone().into();
        let target = Excel::new(&runtime, test_file.output_file.clone());
        let template = Template::compile(&runtime).unwrap();

        assert!(target.write(&template).is_ok());
    }
}
