use std::fmt;
use super::Spreadsheet;

impl fmt::Display for Spreadsheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "total_rows: {}", self.rows.len())
    }
}
