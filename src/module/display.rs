use super::Module;
use std::fmt;

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n# Variables")?;
        for ast in self.scope.variables.values() {
            writeln!(f, "{ast}")?;
        }

        writeln!(f, "\n# Functions")?;
        for ast in self.scope.functions.values() {
            writeln!(f, "fn {ast}")?;
        }

        writeln!(f, "\n# Spreadsheet")?;
        write!(f, "{}", self.spreadsheet.borrow())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    #[test]
    fn display() {
        let module_str = build_module().to_string();

        assert!(module_str.contains("# Variables"));
        assert!(module_str.contains("# Functions"));
        assert!(module_str.contains("# Spreadsheet"));
    }
}
