use super::SourcePosition;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Cell {
    pub value: String,
    // TODO: currently this stores a `SourcePosition` for every character in the cell.  If the
    // cell is 80 characters wide, this will have 80 entries.  We could probably instead split
    // it up into ranges of offsets per the line they occur on and make this much more compact
    // (maybe a trie structure)
    pub(crate) offset_positions: Vec<SourcePosition>,
}

#[cfg(test)]
mod tests {}
