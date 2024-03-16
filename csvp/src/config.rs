#[derive(Debug)]
pub struct Config {
    pub(super) separator: char,
}

impl Default for Config {
    fn default() -> Self {
        Self { separator: ',' }
    }
}


