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
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("\t|\t")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn display() {
        let row = Row {
            cells: vec![
                Cell {
                    parsed_value: "foo".into(),
                    ..Cell::new(build_field("foo", (0, 0)))
                },
                Cell {
                    parsed_value: "bar".into(),
                    ..Cell::new(build_field("bar", (1, 0)))
                },
                Cell {
                    parsed_value: "baz".into(),
                    ..Cell::new(build_field("baz", (2, 0)))
                },
            ],
            ..Default::default()
        };

        assert_eq!(row.to_string(), "|\tfoo\t|\tbar\t|\tbaz\t|");
    }
}
