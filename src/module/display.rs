use super::Module;
use std::fmt;

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.scope.variables.is_empty() {
            writeln!(f, "\n# Variables")?;
            for ast in self.scope.variables.values() {
                writeln!(f, "{ast}")?;
            }
        }

        if !self.scope.functions.is_empty() {
            writeln!(f, "\n# Functions")?;
            for ast in self.scope.functions.values() {
                writeln!(f, "fn {ast}")?;
            }
        }

        writeln!(f, "\n# Spreadsheet")?;
        write!(f, "{}", self.spreadsheet)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::test_utils::*;

    #[test]
    fn display() {
        let mut module = build_module();
        module.scope.define_variable("foo", Ast::new(1.into()));
        module.scope.define_function("bar", Ast::new(1.into()));
        let module_str = module.to_string();

        assert!(module_str.contains("# Variables"));
        assert!(module_str.contains("# Functions"));
        assert!(module_str.contains("# Spreadsheet"));
    }
}
