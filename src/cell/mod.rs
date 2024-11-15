//! # Cell
//!
use crate::ast::Ast;
use crate::cell_options::{
    BorderSide, BorderStyle, DataValidation, HorizontalAlign, NumberFormat, TextFormat, TextWrap,
    VerticalAlign,
};
use crate::parser::ast_parser::AstParser;
use crate::parser::cell_parser::CellParser;
use crate::{ArcSourceCode, Result, Rgb, Row};
use csvp::Field;
use std::collections;

mod display;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Cell {
    pub ast: Option<Ast>,
    pub field: Field,
    pub parsed_value: String,
    pub border_color: Option<Rgb>,
    pub border_style: BorderStyle,
    pub borders: collections::HashSet<BorderSide>,
    pub color: Option<Rgb>,
    pub data_validation: Option<DataValidation>,
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

fn parse_ast(input: &str, field: &Field, source_code: &ArcSourceCode) -> Result<Option<Ast>> {
    Ok(if input.starts_with('=') {
        Some(
            AstParser::parse(input, false, Some(field.clone()), source_code.clone())
                .map_err(|e| source_code.cell_syntax_error(e, field.address))?,
        )
    } else {
        None
    })
}

impl Cell {
    #[cfg(test)]
    pub(crate) fn new(field: Field) -> Self {
        Self {
            field,
            ast: Option::default(),
            border_color: Option::default(),
            border_style: BorderStyle::default(),
            borders: collections::HashSet::default(),
            color: Option::default(),
            data_validation: Option::default(),
            font_color: Option::default(),
            font_family: Option::default(),
            font_size: Option::default(),
            horizontal_align: HorizontalAlign::default(),
            lock: Default::default(),
            note: Option::default(),
            number_format: Option::default(),
            parsed_value: String::default(),
            text_formats: collections::HashSet::default(),
            text_wrap: TextWrap::default(),
            var: Option::default(),
            vertical_align: VerticalAlign::default(),
        }
    }

    pub(crate) fn parse(field: &Field, row: &mut Row, source_code: &ArcSourceCode) -> Result<Self> {
        let mut cell = CellParser::parse(field, row, source_code.clone())?;
        cell.ast = parse_ast(&cell.parsed_value, &cell.field.clone(), source_code)?;

        Ok(cell)
    }

    /// Copy all of the values from `row` which are relevant for a `Cell` to inherit
    pub(crate) fn default_from(row: Row, field: Field) -> Self {
        Self {
            ast: None,
            border_color: row.border_color,
            border_style: row.border_style,
            borders: row.borders,
            color: row.color,
            data_validation: row.data_validation,
            field,
            font_color: row.font_color,
            font_family: row.font_family,
            font_size: row.font_size,
            horizontal_align: row.horizontal_align,
            lock: row.lock,
            note: row.note,
            number_format: row.number_format,
            parsed_value: String::default(),
            text_formats: row.text_formats,
            text_wrap: row.text_wrap,
            var: None,
            vertical_align: row.vertical_align,
        }
    }

    pub(crate) fn side_has_border(&self, border_side: BorderSide) -> bool {
        self.borders.contains(&BorderSide::All) || self.borders.contains(&border_side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn parse_no_ast() {
        let test_file = &TestSourceCode::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let cell = Cell::parse(
            &build_field("foo", (0, 4)),
            &mut Row::default(),
            &ArcSourceCode::new(source_code),
        )
        .unwrap();

        assert_eq!(cell.field.value, "foo");
        assert_eq!(cell.ast, None);
    }

    #[test]
    fn parse_ast() {
        let test_file = &TestSourceCode::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let cell = Cell::parse(
            &build_field("=1 + foo", (0, 4)),
            &mut Row::default(),
            &ArcSourceCode::new(source_code),
        )
        .unwrap();

        assert!(cell.ast.is_some());
    }
}
