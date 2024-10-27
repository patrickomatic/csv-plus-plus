use super::Scope;
use std::fmt;

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Variables")?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn display() {
        let mut scope = Scope::default();
        scope.variables.insert("foo".to_string(), 420.into());
        scope.functions.insert(
            "bar".to_string(),
            Node::fn_def("foo", &["a", "b"], Ast::new(1)).into(),
        );

        assert_eq!(
            "# Variables\n420\n\n# Functions\nfn foo(a, b) 1\n",
            scope.to_string()
        );
    }
}
