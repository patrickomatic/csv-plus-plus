use super::Output;
use std::fmt;

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GoogleSheets(id) => write!(f, "Google Sheets: {id}"),
            Self::Csv(path) => write!(f, "CSV: {}", path.display()),
            Self::Excel(path) => write!(f, "Excel: {}", path.display()),
            Self::OpenDocument(path) => write!(f, "OpenDocument: {}", path.display()),
        }
    }
}

#[cfg(test)]
mod tests {
    // use crate::test_utils::TestFile;
    // use super::*;

    #[test]
    fn display() {
        // TODO
    }
}

