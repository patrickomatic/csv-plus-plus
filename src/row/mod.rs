mod display;

use crate::cell_options::{
    BorderSide, BorderStyle, DataValidation, Fill, HorizontalAlign, NumberFormat, TextFormat,
    TextWrap, VerticalAlign,
};
use crate::{ArcSourceCode, Cell, Result, Rgb, Scope};
use csvp::Field;
use std::collections;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub border_color: Option<Rgb>,
    pub border_style: BorderStyle,
    pub borders: collections::HashSet<BorderSide>,
    pub color: Option<Rgb>,
    pub data_validation: Option<DataValidation>,
    pub fill: Option<Fill>,
    pub font_color: Option<Rgb>,
    pub font_family: Option<String>,
    pub font_size: Option<u8>,
    pub horizontal_align: HorizontalAlign,
    pub lock: bool,
    pub note: Option<String>,
    pub number_format: Option<NumberFormat>,
    pub text_formats: collections::HashSet<TextFormat>,
    pub text_wrap: TextWrap,
    pub var: Option<String>,
    pub vertical_align: VerticalAlign,
}

impl Row {
    pub(crate) fn eval(
        self,
        source_code: &ArcSourceCode,
        scope: &Scope,
        row_a1: a1::Row,
    ) -> Result<Self> {
        let mut cells = Vec::with_capacity(self.cells.len());
        for (cell_index, cell) in self.cells.into_iter().enumerate() {
            let cell_a1 = a1::Address::new(cell_index, row_a1.y);
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

    pub(crate) fn parse(record_result: &[Field], source_code: &ArcSourceCode) -> Result<Self> {
        let mut row = Self::default();
        Ok(Self {
            cells: record_result
                .iter()
                .map(|field| Cell::parse(field, &mut row, source_code))
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
        let mut cell = Cell::new(build_field("", (0, 0)));
        cell.ast = Some(1.into());
        assert!(Row {
            cells: vec![cell],
            ..Default::default()
        }
        .eval(&build_source_code(), &Scope::default(), 0.into())
        .is_ok());
    }

    #[test]
    fn parse() {
        let row = Row::parse(
            &vec![
                build_field("a", (0, 0)),
                build_field("b", (1, 0)),
                build_field("c", (2, 0)),
            ],
            &build_source_code(),
        )
        .unwrap();

        assert_eq!(row.cells.len(), 3);
    }
}
