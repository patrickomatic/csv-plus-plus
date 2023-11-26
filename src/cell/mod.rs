//! # Cell
//!
use crate::ast::Ast;
use crate::parser::ast_parser::AstParser;
use crate::parser::cell_parser::CellParser;
use crate::{
    BorderSide, BorderStyle, Compiler, DataValidation, HorizontalAlign, NumberFormat, Result, Rgb,
    Row, TextFormat, VerticalAlign,
};
use a1_notation::Address;
use std::collections::HashSet;

mod display;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Cell {
    pub ast: Option<Ast>,
    pub value: String,
    pub border_color: Option<Rgb>,
    pub border_style: Option<BorderStyle>,
    pub borders: HashSet<BorderSide>,
    pub color: Option<Rgb>,
    pub data_validation: Option<DataValidation>,
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

fn parse_ast(input: &str, compiler: &Compiler) -> Result<Option<Ast>> {
    Ok(if let Some(without_equals) = input.strip_prefix('=') {
        Some(AstParser::parse(without_equals, false, compiler)?)
    } else {
        None
    })
}

impl Cell {
    pub fn parse(
        input: &str,
        position: Address,
        row: &mut Row,
        compiler: &Compiler,
    ) -> Result<Self> {
        let mut cell = CellParser::parse(input, position, row, compiler)?;
        cell.ast = parse_ast(&cell.value, compiler)?;

        Ok(cell)
    }

    /// Copy all of the values from `row` which are relevant for a `Cell` to inherit
    pub(crate) fn default_from(row: &Row) -> Self {
        Self {
            border_color: row.border_color.clone(),
            border_style: row.border_style,
            borders: row.borders.clone(),
            color: row.color.clone(),
            data_validation: row.data_validation.clone(),
            font_color: row.font_color.clone(),
            font_family: row.font_family.clone(),
            font_size: row.font_size,
            horizontal_align: row.horizontal_align,
            lock: row.lock,
            note: row.note.clone(),
            number_format: row.number_format,
            text_formats: row.text_formats.clone(),
            vertical_align: row.vertical_align,
            ..Default::default()
        }
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
        let cell =
            Cell::parse("foo", Address::new(0, 4), &mut Row::default(), &source_code).unwrap();

        assert_eq!(cell.value, "foo");
        assert_eq!(cell.ast, None);
    }

    #[test]
    fn parse_ast() {
        let test_file = &TestSourceCode::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let cell = Cell::parse(
            "=1 + foo",
            Address::new(0, 4),
            &mut Row::default(),
            &source_code,
        )
        .unwrap();

        assert!(cell.ast.is_some());
    }
}
