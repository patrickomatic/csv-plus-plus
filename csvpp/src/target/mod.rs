mod csv;
mod excel;
mod file_backer_upper;
mod google_sheets;
mod open_document;

pub use crate::target::csv::Csv;
pub use excel::Excel;
pub use google_sheets::GoogleSheets;
pub use open_document::OpenDocument;

use crate::{Result, Template};

pub trait CompilationTarget {
    fn write_backup(&self) -> Result<()>;

    fn write(&self, template: &Template) -> Result<()>;
}
