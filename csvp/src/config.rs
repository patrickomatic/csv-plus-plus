//! # Config
use super::Offset;

#[derive(Debug)]
pub struct Config {
    pub separator: char,
    pub lines_above: Offset,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            separator: ',',
            lines_above: 0,
        }
    }
}
