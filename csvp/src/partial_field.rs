use super::{Field, SourcePosition};

#[derive(Debug, Default)]
pub(crate) struct PartialField {
    field: String,
    positions: Vec<SourcePosition>,
}

impl From<PartialField> for Field {
    fn from(pf: PartialField) -> Self {
        Field {
            value: pf.field.trim().to_string(),
            positions: pf.positions,
        }
    }
}

impl PartialField {
    pub(crate) fn push(&mut self, c: char, position: SourcePosition) {
        self.field.push(c);
        self.positions.push(position);
    }
}
