use std::collections;
use std::fmt;

use crate::ast::Ast;

#[derive(Debug)]
pub struct Options {
    pub backup: bool,
    pub key_values: collections::HashMap<String, Ast>,
    pub offset: (u32, u32),
    pub overwrite_values: bool,
    pub verbose: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            backup: false,
            key_values: collections::HashMap::new(),
            offset: (0, 0),
            overwrite_values: true,
            verbose: false,
        }
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "backup: {}", self.backup)?;
        writeln!(f, "key_values: {:?}", self.key_values)?;
        writeln!(f, "offset: ({}, {})", self.offset.0, self.offset.1)?;
        writeln!(f, "overwrite_values: {}", self.overwrite_values)?;
        write!(f, "verbose: {}", self.verbose)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let options = Options::default();

        assert_eq!(r#"backup: false
key_values: {}
offset: (0, 0)
overwrite_values: true
verbose: false"#, options.to_string());
    }
}
