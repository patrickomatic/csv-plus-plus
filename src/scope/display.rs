use super::Scope;
use std::fmt;

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Required Modules")?;
        for m in &self.required_modules {
            writeln!(f, "{m}")?;
        }

        writeln!(f, "\n# Variables")?;
        for ast in self.variables.values() {
            writeln!(f, "{ast}")?;
        }

        writeln!(f, "\n# Functions")?;
        for ast in self.functions.values() {
            writeln!(f, "fn {ast}")?;
        }
        Ok(())
    }
}
