use super::Spreadsheet;
use std::fmt;

impl fmt::Display for Spreadsheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "total_rows: {}", self.rows.len())
    }
}
