use super::Module;
use std::fmt;

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n# Variables")?;
        for ast in self.variables.values() {
            writeln!(f, "{ast}")?;
        }

        writeln!(f, "\n# Functions")?;
        for ast in self.functions.values() {
            writeln!(f, "fn {ast}")?;
        }

        writeln!(f, "\n# Spreadsheet")?;
        write!(f, "{}", self.spreadsheet.borrow())
    }
}

#[cfg(test)]
mod tests {
    use super::super::ModuleName;
    use crate::{Module, Spreadsheet};
    use std::cell;
    use std::collections;

    fn build_module() -> Module {
        Module {
            compiler_version: "v0.0.1".to_string(),
            functions: collections::HashMap::new(),
            module_name: ModuleName("main".to_string()),
            spreadsheet: cell::RefCell::new(Spreadsheet::default()),
            variables: collections::HashMap::new(),
        }
    }

    #[test]
    fn display() {
        let module_str = build_module().to_string();

        assert!(module_str.contains("# Variables"));
        assert!(module_str.contains("# Functions"));
        assert!(module_str.contains("# Spreadsheet"));
    }
}
