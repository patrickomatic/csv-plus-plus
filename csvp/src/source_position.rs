use super::Offset;
use std::fmt;

#[derive(Copy, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SourcePosition {
    pub line_number: Offset,
    pub line_offset: Offset,
}

impl SourcePosition {
    pub(crate) fn new(line_offset: Offset, line_number: Offset) -> Self {
        Self {
            line_number,
            line_offset,
        }
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line_number + 1, self.line_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(SourcePosition::new(0, 0).to_string(), "1:0");
    }
}
