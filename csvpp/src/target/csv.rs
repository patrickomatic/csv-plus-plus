//! Functions for writing to CSV files
//!
use csv;

use crate::error::Error;

struct CsvTarget<'a> {
    writer: &'a mut csv::Writer,
}

impl CompilerTarget for CsvWriter {
    fn write(&self, options: &Options, template: &Template) -> Result<(), Error> {
        let mut writer = csv::Writer::from_writer(file_writer()?);
        let mut csv_target = CsvTarget { writer };

        Ok(())
    }

    fn file_writer(&self, options: &Options) -> Result<(), Error> {
        Ok(())
    }
}
