use super::SourcePosition;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Field {
    pub value: String,
    // TODO: currently this stores a `SourcePosition` for every character in the field.  If the
    // field is 80 characters wide, this will have 80 entries.  We could probably instead split
    // it up into ranges of offsets per the line they occur on and make this much more compact
    // (maybe a trie structure)
    pub(crate) positions: Vec<SourcePosition>,
}

#[cfg(test)]
mod tests {}
