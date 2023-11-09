use super::Template;
use std::fmt;

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Variables")?;
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
    use crate::{Spreadsheet, Template};
    use std::cell;
    use std::collections;

    fn build_template() -> Template {
        Template {
            compiler_version: "v0.0.1".to_string(),
            functions: collections::HashMap::new(),
            module: "main".to_string(),
            spreadsheet: cell::RefCell::new(Spreadsheet::default()),
            variables: collections::HashMap::new(),
        }
    }

    #[test]
    fn display() {
        let template_str = build_template().to_string();

        assert!(template_str.contains("# Variables"));
        assert!(template_str.contains("# Functions"));
        assert!(template_str.contains("# Spreadsheet"));
    }
}
