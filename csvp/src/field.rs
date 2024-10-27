use super::{Offset, SourcePosition};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Field {
    pub address: a1::Address,

    /// The value of the field, trimmed of leading and trailing spaces
    pub value: String,

    // TODO: currently this stores a `SourcePosition` for every character in the field.  If the
    // field is 80 characters wide, this will have 80 entries.  We could probably instead split
    // it up into ranges of offsets per the line they occur on and make this much more compact
    // (maybe a trie structure)
    pub positions: Vec<SourcePosition>,
}

impl Field {
    /// When a cell spans multiple lines, the newlines are removed and the content of the cell if
    /// flattened as if they weren't there.  Given an offset into the cell in this format, find the
    /// position of the original location in source.
    #[must_use]
    pub fn position_for_offset(&self, offset: Offset) -> Option<SourcePosition> {
        if offset >= self.positions.len() {
            None
        } else {
            Some(self.positions[offset])
        }
    }

    #[must_use]
    pub fn eof_position(&self) -> Option<SourcePosition> {
        self.positions.last().map(|p| SourcePosition {
            line_offset: p.line_offset + 1,
            ..*p
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_field() -> Field {
        Field {
            address: (0, 0).into(),
            value: "foo".into(),
            positions: vec![(0, 0).into(), (1, 0).into(), (2, 0).into()],
        }
    }

    #[test]
    fn position_for_offset() {
        assert_eq!(build_field().position_for_offset(1), Some((1, 0).into()));
    }

    #[test]
    fn position_for_offset_out_of_bounds() {
        assert_eq!(build_field().position_for_offset(100), None);
    }
}
