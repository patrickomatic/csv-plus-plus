mod csv;
mod excel;
mod google_sheets;
mod open_document;

pub use crate::target::csv::Csv;
pub use excel::Excel;
pub use google_sheets::GoogleSheets;
pub use open_document::OpenDocument;

use crate::{Options, Result, Template};

pub trait CompilationTarget {
    // TODO: create a FileCompilationTarget trait with a defaut implementation of write_backup
    fn write_backup(&self) -> Result<()>;

    fn write(&self, options: &Options, template: &Template) -> Result<()>;
}

#[cfg(test)]
mod tests {
    // TODO
}
