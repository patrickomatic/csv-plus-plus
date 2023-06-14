use std::collections::HashMap;
use std::fmt;

use crate::Node;

#[derive(Debug)]
pub struct Options {
    pub backup: bool,
    pub key_values: HashMap<String, Box<dyn Node>>,
    pub offset: (u32, u32),
    pub overwrite_values: bool,
    pub verbose: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            backup: false,
            key_values: HashMap::new(),
            offset: (0, 0),
            overwrite_values: true,
            verbose: false,
        }
    }
}

// TODO: do I really need this or just use Debug
impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            r#"
    backup: {}
    key_values: {:?}
    offset: ({}, {})
    overwrite_values: {}
    verbose: {}
            "#,
            self.backup,
            self.key_values,
            self.offset.0,
            self.offset.1,
            self.overwrite_values,
            self.verbose,
        )
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn display() {
    // TODO
    // }
}
