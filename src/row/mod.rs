mod display;

use crate::{
    ArcSourceCode, BorderSide, BorderStyle, Cell, DataValidation, Fill, HorizontalAlign,
    NumberFormat, Result, Rgb, Scope, TextFormat, VerticalAlign,
};
use std::collections::HashSet;

type CsvRowResult = std::result::Result<csv::StringRecord, csv::Error>;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub border_color: Option<Rgb>,
    pub border_style: Option<BorderStyle>,
    pub borders: HashSet<BorderSide>,
    pub color: Option<Rgb>,
    pub data_validation: Option<DataValidation>,
    pub fill: Option<Fill>,
    pub font_color: Option<Rgb>,
    pub font_family: Option<String>,
    pub font_size: Option<u8>,
    pub horizontal_align: Option<HorizontalAlign>,
    pub lock: bool,
    pub note: Option<String>,
    pub number_format: Option<NumberFormat>,
    pub text_formats: HashSet<TextFormat>,
    pub var: Option<String>,
    pub vertical_align: Option<VerticalAlign>,
}

impl Row {
    pub(crate) fn eval(
        self,
        source_code: &ArcSourceCode,
        scope: &Scope,
        row_a1: a1_notation::Row,
    ) -> Result<Self> {
        let mut cells = vec![];
        for (cell_index, cell) in self.cells.into_iter().enumerate() {
            let cell_a1 = a1_notation::Address::new(cell_index, row_a1.y);
            let ast = if let Some(a) = cell.ast {
                Some(
                    a.eval(scope, Some(cell_a1))
                        .map_err(|e| source_code.eval_error(e, Some(cell_a1)))?,
                )
            } else {
                None
            };
            cells.push(Cell { ast, ..cell });
        }

        Ok(Self { cells, ..self })
    }

    pub(crate) fn parse(
        record_result: CsvRowResult,
        row_a1: a1_notation::Row,
        source_code: &ArcSourceCode,
    ) -> Result<Self> {
        // handle if the row is blank or an error or something. (maybe we should warn here if it's
        // an error?)
        let csv_parsed_row = &record_result.unwrap_or_default();

        let mut row = Self::default();
        Ok(Self {
            cells: csv_parsed_row
                .into_iter()
                .enumerate()
                .map(|(cell_index, unparsed_value)| {
                    Cell::parse(
                        unparsed_value,
                        a1_notation::Address::new(cell_index, row_a1.y),
                        &mut row,
                        source_code,
                    )
                })
                .collect::<Result<Vec<_>>>()?,
            ..row
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn eval_simple_ast() {
        assert!(Row {
            cells: vec![Cell {
                ast: Some(1.into()),
                ..Default::default()
            }],
            ..Default::default()
        }
        .eval(&build_source_code(), &Scope::default(), 0.into())
        .is_ok());
    }

    #[test]
    fn parse() {
        let row = Row::parse(
            Ok(csv::StringRecord::from(vec!["a", "b", "c"])),
            0.into(),
            &build_source_code(),
        )
        .unwrap();

        assert_eq!(row.cells.len(), 3);
    }
}
