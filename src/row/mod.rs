use crate::{Cell, Result, RowModifier, SourceCode};
use serde::{Deserialize, Serialize};

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
        source_code: &SourceCode,
    ) -> Result<Self> {
        let mut cells: Vec<Cell> = vec![];
        let mut row_modifier = RowModifier::default();

        // handle if the row is blank or an error or something. (maybe we should warn here?)
        let csv_parsed_row = &record_result.unwrap_or(csv::StringRecord::new());

        for (cell_index, unparsed_value) in csv_parsed_row.into_iter().enumerate() {
            let a1 = a1_notation::Address::new(cell_index, row_index);
            let (cell, rm) = Cell::parse(unparsed_value, a1, &row_modifier, source_code)?;

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
}
