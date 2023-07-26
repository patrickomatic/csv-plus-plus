//! # Spreadsheet
//!
//!
use a1_notation;
use serde::{Deserialize, Serialize};
use std::collections;
use std::fmt;
use csv;
use crate::{Modifier, Result, SourceCode};
use crate::ast::{Ast, Node, Variables};
use super::ast_parser::AstParser;
use super::modifier_parser::ModifierParser;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpreadsheetCell {
    pub ast: Option<Ast>,
    pub position: a1_notation::A1,
    pub modifier: Modifier,
    pub value: String,
}

impl SpreadsheetCell {
    pub fn parse(
        input: &str,
        position: a1_notation::A1,
        row_modifier: Modifier,
        source_code: &SourceCode,
    ) -> Result<(SpreadsheetCell, Modifier)> {
        let parsed_modifiers = ModifierParser::parse(input, source_code, position, row_modifier)?;

        Ok((SpreadsheetCell {
            ast: Self::parse_ast(&parsed_modifiers.value, source_code)?,
            position: parsed_modifiers.position,
            modifier: parsed_modifiers.modifier,
            value: parsed_modifiers.value,
        }, parsed_modifiers.row_modifier))
    }

    fn parse_ast(input: &str, source_code: &SourceCode) -> Result<Option<Ast>> {
        if let Some(without_equals) = input.strip_prefix('=') {
            Ok(Some(AstParser::parse(without_equals, false, Some(source_code))?))
        } else {
            Ok(None)
        }
    }
}

// TODO we might want a more dedicated function like to_formula
impl fmt::Display for SpreadsheetCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ast.clone().map(|a| a.to_string()).unwrap_or_else(|| self.value.clone()))
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Spreadsheet {
    pub cells: Vec<Vec<SpreadsheetCell>>,
}

impl Spreadsheet {
    /// Parse the spreadsheet section of a csv++ source file.
    pub fn parse(source_code: &SourceCode) -> Result<Spreadsheet> {
        let mut csv_reader = Self::csv_reader(source_code);
        let mut cells: Vec<Vec<SpreadsheetCell>> = vec![];

        for (row_index, result) in csv_reader.records().enumerate() {
            let row = Self::parse_row(result, row_index, source_code)?;
            cells.push(row);
        }

        Ok(Spreadsheet { cells })
    }

    /// Extract all of the variables that were defined by cells contained in this spreadsheet
    // 
    // NOTE: we could also store these in a HashMap on the Spreadsheet as we build it
    pub fn variables(&self) -> Variables {
        let mut vars = collections::HashMap::new();
        self.cells.iter().flatten().for_each(|c| {
            if let Some(var_id) = &c.modifier.var {
                let reference: Ast = Box::new(Node::Reference(c.position.to_string()));
                vars.insert(var_id.to_owned(), reference);
            }
        });

        vars
    }

    fn csv_reader(source_code: &SourceCode) -> csv::Reader<&[u8]> {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(source_code.csv_section.as_bytes())
    }

    fn parse_row(
        record_result: std::result::Result<csv::StringRecord, csv::Error>,
        row_index: usize,
        source_code: &SourceCode,
    ) -> Result<Vec<SpreadsheetCell>> {
        let mut row: Vec<SpreadsheetCell> = vec![];
        let mut row_modifier = Modifier::new(true);
        let csv_parsed_row = &record_result.unwrap_or(csv::StringRecord::new());

        for (cell_index, unparsed_value) in csv_parsed_row.into_iter().enumerate() {
            let a1 = a1_notation::A1::builder().xy(cell_index, row_index).build().unwrap();
            let (cell, rm) = SpreadsheetCell::parse(unparsed_value, a1, row_modifier, source_code)?;

            row_modifier = rm;
            row.push(cell);
        }

        Ok(row)
    }
}

impl fmt::Display for Spreadsheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: do something better
        write!(f, "{}", self.cells.len())
    }
}

#[cfg(test)]
mod tests {
    use std::path;
    use super::*;
    use crate::SourceCode;
    use crate::modifier::TextFormat;

    fn build_source_code(input: &str) -> SourceCode {
        SourceCode::new(input, path::PathBuf::from("foo.csvpp")).unwrap()
    }

    #[test]
    fn parse_simple() {
        let source_code = build_source_code("foo,bar,baz\n1,2,3\n");
        let spreadsheet = Spreadsheet::parse(&source_code).unwrap();

        // 2 rows
        assert_eq!(spreadsheet.cells.len(), 2);

        // each row has 3 cells
        assert_eq!(spreadsheet.cells[0].len(), 3);
        assert_eq!(spreadsheet.cells[1].len(), 3);

        // each row has a parsed value
        assert_eq!(spreadsheet.cells[0][0].value, "foo");
        assert_eq!(spreadsheet.cells[0][1].value, "bar");
        assert_eq!(spreadsheet.cells[0][2].value, "baz");
        assert_eq!(spreadsheet.cells[1][0].value, "1");
        assert_eq!(spreadsheet.cells[1][1].value, "2");
        assert_eq!(spreadsheet.cells[1][2].value, "3");
        
        // none have ASTs (didn't start with `=`)
        assert!(spreadsheet.cells[0][0].ast.is_none());
        assert!(spreadsheet.cells[0][1].ast.is_none());
        assert!(spreadsheet.cells[0][2].ast.is_none());
        assert!(spreadsheet.cells[1][0].ast.is_none());
        assert!(spreadsheet.cells[1][1].ast.is_none());
        assert!(spreadsheet.cells[1][2].ast.is_none());
    }

    #[test]
    fn parse_with_asts() {
        let source_code = build_source_code("=1,=2 * 3,=foo\n");
        let spreadsheet = Spreadsheet::parse(&source_code).unwrap();

        assert!(spreadsheet.cells[0][0].ast.is_some());
        assert!(spreadsheet.cells[0][1].ast.is_some());
        assert!(spreadsheet.cells[0][2].ast.is_some());
    }

    #[test]
    fn parse_with_modifiers() {
        let source_code = build_source_code("[[f=b / fs=20]]foo");
        let spreadsheet = Spreadsheet::parse(&source_code).unwrap();

        assert!(spreadsheet.cells[0][0].modifier.formats.contains(&TextFormat::Bold));
        assert_eq!(spreadsheet.cells[0][0].modifier.font_size, Some(20))
    }

    #[test]
    fn parse_with_row_modifier() {
        // TODO
    }
}
