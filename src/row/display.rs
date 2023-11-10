use super::Row;
use std::fmt;

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cells.is_empty() {
            write!(f, "-- empty --")
        } else {
            write!(
                f,
                "|\t{}\t|",
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
                    modifier: Modifier::default(),
                    value: "foo".to_string(),
                },
                Cell {
                    ast: None,
                    modifier: Modifier::default(),
                    value: "bar".to_string(),
                },
                Cell {
                    ast: None,
                    modifier: Modifier::default(),
                    value: "baz".to_string(),
                },
            ],
            modifier: RowModifier::default(),
        };

        assert_eq!(row.to_string(), "|\tfoo\t|\tbar\t|\tbaz\t|");
    }
}
