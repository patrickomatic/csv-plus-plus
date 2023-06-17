mod csv;
mod excel;
mod google_sheets;
mod open_document;

pub use crate::target::csv::Csv;
pub use excel::Excel;
pub use google_sheets::GoogleSheets;
pub use open_document::OpenDocument;

use crate::{Options, Result, Template};

pub trait CompilerTarget {
    fn write(&self, options: &Options, template: &Template) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_output_target_google_sheets() {
    }

    #[test]
    fn from_output_target_csv() {
    }

    #[test]
    fn from_output_target_excel() {
    }
}
