//! # Spreadsheet
//!
//!
use a1_notation;
use serde::{Deserialize, Serialize};
use std::collections;
use std::fmt;
use csv;
use crate::{Modifier, Result, SourceCode};
use crate::ast::{Node, Variables, VariableValue};
use super::spreadsheet_cell::SpreadsheetCell;

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
    // NOTE: we could also store these in a HashMap on the Spreadsheet as we build it rather than
    // parsing them out at runtime
    pub fn variables(&self) -> Variables {
        let mut vars = collections::HashMap::new();

        self.cells.iter().flatten().for_each(|c| {
            if let Some(var_id) = &c.modifier.var {
                let reference = if let Some(scope) = c.row_modifier.clone().and_then(|rm| rm.expand) {
                    Node::Variable {
                        name: var_id.clone(),
                        value: VariableValue::Relative {
                            scope,
                            column: c.position.column,
                        },
                    }
                } else {
                    Node::Variable {
                        name: var_id.clone(),
                        value: VariableValue::Absolute(c.position),
                    }
                };

                vars.insert(var_id.to_owned(), Box::new(reference));
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
        let mut row_modifier = Modifier::default();
        let csv_parsed_row = &record_result.unwrap_or(csv::StringRecord::new());

        for (cell_index, unparsed_value) in csv_parsed_row.into_iter().enumerate() {
            let a1 = a1_notation::Address::new(cell_index, row_index);
            let cell = SpreadsheetCell::parse(unparsed_value, a1, &row_modifier, source_code)?;

            // a row modifier was defined, so make sure it applies to cells going forward
            if let Some(rm) = &cell.row_modifier {
                row_modifier = rm.clone();
            }

            row.push(cell);
        }

        Ok(row)
    }

    pub fn widest_row(&self) -> usize {
        self.cells.iter().map(|row| row.len()).max().unwrap_or(0)
    }
}

impl fmt::Display for Spreadsheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "total_rows: {}", self.cells.len())
    }
}

#[cfg(test)]
mod tests {
    use a1_notation::Address;
    use crate::modifier::TextFormat;
    use crate::{Expand, SourceCode};
    use std::path;
    use super::*;

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

        // the cells have the correct positions
        assert_eq!(spreadsheet.cells[0][0].position.to_string(), "A1");
        assert_eq!(spreadsheet.cells[0][1].position.to_string(), "B1");
        assert_eq!(spreadsheet.cells[0][2].position.to_string(), "C1");
        assert_eq!(spreadsheet.cells[1][0].position.to_string(), "A2");
        assert_eq!(spreadsheet.cells[1][1].position.to_string(), "B2");
        assert_eq!(spreadsheet.cells[1][2].position.to_string(), "C2");

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
        let source_code = build_source_code("![[f=b]]foo,bar,baz");
        let spreadsheet = Spreadsheet::parse(&source_code).unwrap();

        assert!(spreadsheet.cells[0][0].modifier.formats.contains(&TextFormat::Bold));
        assert!(spreadsheet.cells[0][1].modifier.formats.contains(&TextFormat::Bold));
        assert!(spreadsheet.cells[0][2].modifier.formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn variables_unscoped() {
        let spreadsheet = Spreadsheet {
            cells: vec![
                vec![
                    SpreadsheetCell {
                        ast: None,
                        position: Address::new(0, 0),
                        modifier: Modifier {
                            var: Some("foo".to_string()),
                            ..Default::default()
                        },
                        row_modifier: None,
                        value: "".to_string(),
                    },
                    SpreadsheetCell {
                        ast: None,
                        position: Address::new(1, 1),
                        modifier: Modifier {
                            var: Some("bar".to_string()),
                            ..Default::default()
                        },
                        row_modifier: None,
                        value: "".to_string(),
                    },
                ]],
        };

        let variables = spreadsheet.variables();
        assert_eq!(**variables.get("foo").unwrap(), 
                   Node::var("foo", VariableValue::Absolute(Address::new(0, 0))));
        assert_eq!(**variables.get("bar").unwrap(), 
                   Node::var("bar", VariableValue::Absolute(Address::new(1, 1))));
    }

    #[test]
    fn variables_with_scope() {
        let spreadsheet = Spreadsheet {
            cells: vec![
                vec![
                    SpreadsheetCell {
                        ast: None,
                        position: (0, 0).into(),
                        modifier: Modifier {
                            var: Some("foo".to_string()),
                            ..Default::default()
                        },
                        row_modifier: Some(Modifier {
                            expand: Some(Expand::new(0, Some(10))),
                            ..Default::default()
                        }),
                        value: "".to_string(),
                    },
                    SpreadsheetCell {
                        ast: None,
                        position: (1, 1).into(),
                        modifier: Modifier {
                            var: Some("bar".to_string()),
                            ..Default::default()
                        },
                        row_modifier: Some(Modifier {
                            expand: Some(Expand::new(10, Some(100))),
                            ..Default::default()
                        }),
                        value: "".to_string(),
                    },
                ]],
        };

        let variables = spreadsheet.variables();
        assert_eq!(**variables.get("foo").unwrap(), 
                   Node::var("foo", VariableValue::Relative {
                       scope: Expand { amount: Some(10), start_row: 0.into() },
                       column: 0.into(),
                   }));
                   // Node::var("foo", Address::new(0, 0).into(), 
                             // Some()));
        assert_eq!(**variables.get("bar").unwrap(), 
                   Node::var("bar", VariableValue::Relative {
                       scope: Expand { amount: Some(100), start_row: 10.into() },
                       column: 1.into(),
                   }));
    }

    #[test]
    fn widest_row() {
        let cell = SpreadsheetCell {
            ast: None,
            position: Address::new(0, 0),
            modifier: Modifier::default(),
            row_modifier: None,
            value: "foo".to_string(),
        };
        let spreadsheet = Spreadsheet {
            cells: vec![
                vec![cell.clone()],
                vec![cell.clone(), cell.clone()],
                vec![cell.clone(), cell.clone(), cell.clone()],
            ],
        };

        assert_eq!(spreadsheet.widest_row(), 3);
    }
}
