use super::Row;
use std::fmt;

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cells.is_empty() {
            write!(f, "-- empty --")
        } else {
            write!(
                f,
                "{}: |\t{}\t|",
                self.row,
                &self
                    .cells
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("\t|\t")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modifier::*;
    use crate::*;

    #[test]
    fn display() {
        let row = Row {
            cells: vec![
                Cell {
                    ast: None,
                    position: a1_notation::Address::new(0, 0),
                    modifier: Modifier::default(),
                    value: "foo".to_string(),
                },
                Cell {
                    ast: None,
                    position: a1_notation::Address::new(0, 1),
                    modifier: Modifier::default(),
                    value: "bar".to_string(),
                },
                Cell {
                    ast: None,
                    position: a1_notation::Address::new(0, 2),
                    modifier: Modifier::default(),
                    value: "baz".to_string(),
                },
            ],
            modifier: RowModifier::default(),
            row: 0.into(),
        };

        assert_eq!(row.to_string(), "1: |\tfoo\t|\tbar\t|\tbaz\t|");
    }
}
