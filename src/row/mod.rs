use crate::{Cell, Result, RowModifier, Runtime};
use serde::{Deserialize, Serialize};

mod display;

type CsvRowResult = std::result::Result<csv::StringRecord, csv::Error>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub modifier: RowModifier,
    pub row: a1_notation::Row,
}

impl Row {
    pub(crate) fn parse(
        record_result: CsvRowResult,
        // TODO: maybe make this be a Row... but the naming gets weird
        row_index: usize,
        runtime: &Runtime,
    ) -> Result<Self> {
        let mut cells: Vec<Cell> = vec![];
        let mut row_modifier = RowModifier::default();

        // handle if the row is blank or an error or something. (maybe we should warn here if it's
        // an error?)
        let csv_parsed_row = &record_result.unwrap_or_default();

        for (cell_index, unparsed_value) in csv_parsed_row.into_iter().enumerate() {
            let a1 = a1_notation::Address::new(cell_index, row_index);
            let (cell, rm) = Cell::parse(unparsed_value, a1, &row_modifier, runtime)?;

            // a row modifier was defined, so make sure it applies to cells going forward
            if let Some(r) = rm {
                row_modifier = r;
            }

            cells.push(cell);
        }

        Ok(Self {
            cells,
            modifier: row_modifier,
            row: row_index.into(),
        })
    }

    pub(crate) fn clone_to_row(&self, new_row: a1_notation::Row) -> Self {
        Self {
            row: new_row,
            modifier: RowModifier {
                expand: self.modifier.expand.map(|e| e.clone_to_row(new_row)),
                ..self.modifier.clone()
            },
            cells: self.cells.iter().map(|c| c.clone_to_row(new_row)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn parse() {
        // TODO
    }

    #[test]
    fn clone_to_row() {
        let row = Row {
            row: 22.into(),
            cells: vec![Cell {
                ast: None,
                position: a1_notation::Address::new(0, 22),
                modifier: Modifier::default(),
                value: "foo".to_string(),
            }],
            modifier: RowModifier {
                expand: Some(Expand::new(22, Some(100))),
                ..Default::default()
            },
        };

        assert_eq!(
            row.clone_to_row(5.into()),
            Row {
                row: 5.into(),
                cells: vec![Cell {
                    ast: None,
                    position: a1_notation::Address::new(0, 5),
                    modifier: Modifier::default(),
                    value: "foo".to_string(),
                }],
                modifier: RowModifier {
                    expand: Some(Expand::new(5, Some(100))),
                    ..Default::default()
                },
            }
        );
    }
}
