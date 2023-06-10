//! Functions for writing to CSV files
//!
use csv;

use crate::Error;

struct CsvTarget<'a> {
    writer: &'a mut csv::Writer,
}

impl CompilerTarget for CsvWriter {
    fn write(&self, options: &Options, template: &Template) -> Result<(), &dyn Error> {
        let mut writer = csv::Writer::from_writer(file_writer()?);
        let mut csv_target = CsvTarget { writer };

        Ok(())
    }

    fn file_writer(&self, options: &Options) -> Result<(), &dyn Error> {
        Ok(())
    }
}
