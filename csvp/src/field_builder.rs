use super::{Field, SourcePosition};

#[derive(Debug)]
pub(crate) struct FieldBuilder {
    address: a1::Address,
    value: String,
    positions: Vec<SourcePosition>,
}

impl From<FieldBuilder> for Field {
    fn from(pf: FieldBuilder) -> Self {
        Field {
            address: pf.address,
            value: pf.value.trim().to_string(),
            positions: pf.positions,
        }
    }
}

impl FieldBuilder {
    pub(crate) fn new<A: Into<a1::Address>>(address: A) -> Self {
        Self {
            address: address.into(),
            value: String::default(),
            positions: Vec::default(),
        }
    }

    pub(crate) fn push<P: Into<SourcePosition>>(&mut self, c: char, position: P) {
        self.value.push(c);
        self.positions.push(position.into());
    }
}
