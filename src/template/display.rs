use std::fmt;
use super::Template;

impl fmt::Display for Template<'_> {
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
    use crate::{Template, Runtime, Spreadsheet};
    use crate::test_utils::TestFile;
    use std::cell;
    use std::collections;

    fn build_template(runtime: &Runtime) -> Template {
        Template {
            csv_line_number: 5,
            functions: collections::HashMap::new(),
            variables: collections::HashMap::new(),
            runtime,
            spreadsheet: cell::RefCell::new(Spreadsheet::default()),
        }
    }


    #[test]
    fn display() {
        let test_file = TestFile::new("csv", "");
        let runtime = test_file.into();
        let template_str = build_template(&runtime).to_string();

        assert!(template_str.contains("# Variables"));
        assert!(template_str.contains("# Functions"));
        assert!(template_str.contains("# Spreadsheet"));
   }
}
