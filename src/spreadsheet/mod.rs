//! # Spreadsheet
//!
use crate::ast::{Ast, Node, VariableValue, Variables};
use crate::{csv_reader, ArcSourceCode, Result, Row};
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections;

mod display;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Spreadsheet {
    pub rows: Vec<Row>,
}

impl Spreadsheet {
    /// Parse the spreadsheet section of a csv++ source file.
    pub(crate) fn parse(source_code: ArcSourceCode) -> Result<Spreadsheet> {
        let mut csv_reader = csv_reader()
            .trim(csv::Trim::None)
            .from_reader(source_code.csv_section.as_bytes());

        let mut rows: Vec<Row> = vec![];
        for (row_index, result) in csv_reader.records().enumerate() {
            trace!("Parsing row {row_index}");
            rows.push(Row::parse(result, row_index.into(), source_code.clone())?);
        }

        Ok(Spreadsheet { rows })
    }

    /// Extract all of the variables that were defined by cells contained in this spreadsheet
    //
    // NOTE: we could also store these in a HashMap on the Spreadsheet as we build it rather than
    // parsing them out at compiler
    pub(crate) fn variables(&self) -> Variables {
        let mut vars = collections::HashMap::new();

        for (row_index, row) in self.rows.iter().enumerate() {
            let row_a1: a1_notation::Row = row_index.into();

            // does the row itself have a var?
            if let Some(var_id) = &row.var {
                let reference = if let Some(fill) = row.fill {
                    // if there's also an fill it's relative to that
                    Node::Variable {
                        name: var_id.clone(),
                        value: VariableValue::RowRelative { fill, row: row_a1 },
                    }
                } else {
                    // otherwise it's just relative to the single row where it was defined
                    Node::Variable {
                        name: var_id.clone(),
                        value: VariableValue::Row(row_a1),
                    }
                };

                vars.insert(var_id.to_owned(), Ast::new(reference));
            };

            for (cell_index, cell) in row.cells.iter().enumerate() {
                let cell_a1 = a1_notation::Address::new(cell_index, row_index);

                if let Some(var_id) = &cell.var {
                    let reference = if let Some(fill) = row.fill {
                        Node::Variable {
                            name: var_id.clone(),
                            value: VariableValue::ColumnRelative {
                                fill,
                                column: cell_a1.column,
                            },
                        }
                    } else {
                        Node::Variable {
                            name: var_id.clone(),
                            value: VariableValue::Absolute(cell_a1),
                        }
                    };

                    vars.insert(var_id.to_owned(), Ast::new(reference));
                }
            }
        }

        vars
    }

    pub(crate) fn widest_row(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::*;
    use a1_notation::Address;

    fn build_source_code(input: &str) -> ArcSourceCode {
        ArcSourceCode::new((&TestSourceCode::new("csv", input)).into())
    }

    #[test]
    fn parse_simple() {
        let source_code = build_source_code("foo,bar,baz\n1,2,3\n");
        let spreadsheet = Spreadsheet::parse(source_code).unwrap();

        // 2 rows
        assert_eq!(spreadsheet.rows.len(), 2);

        // each row has 3 cells
        assert_eq!(spreadsheet.rows[0].cells.len(), 3);
        assert_eq!(spreadsheet.rows[1].cells.len(), 3);

        // each row has a parsed value
        assert_eq!(spreadsheet.rows[0].cells[0].value, "foo");
        assert_eq!(spreadsheet.rows[0].cells[1].value, "bar");
        assert_eq!(spreadsheet.rows[0].cells[2].value, "baz");
        assert_eq!(spreadsheet.rows[1].cells[0].value, "1");
        assert_eq!(spreadsheet.rows[1].cells[1].value, "2");
        assert_eq!(spreadsheet.rows[1].cells[2].value, "3");

        // none have ASTs (didn't start with `=`)
        assert!(spreadsheet.rows[0].cells[0].ast.is_none());
        assert!(spreadsheet.rows[0].cells[1].ast.is_none());
        assert!(spreadsheet.rows[0].cells[2].ast.is_none());
        assert!(spreadsheet.rows[1].cells[0].ast.is_none());
        assert!(spreadsheet.rows[1].cells[1].ast.is_none());
        assert!(spreadsheet.rows[1].cells[2].ast.is_none());
    }

    #[test]
    fn parse_with_asts() {
        let source_code = build_source_code("=1,=2 * 3,=foo\n");
        let spreadsheet = Spreadsheet::parse(source_code).unwrap();

        assert!(spreadsheet.rows[0].cells[0].ast.is_some());
        assert!(spreadsheet.rows[0].cells[1].ast.is_some());
        assert!(spreadsheet.rows[0].cells[2].ast.is_some());
    }

    #[test]
    fn parse_trim_spaces() {
        let source_code = build_source_code("   foo   , bar\n");
        let spreadsheet = Spreadsheet::parse(source_code).unwrap();

        assert_eq!(spreadsheet.rows[0].cells[0].value, "foo");
        assert_eq!(spreadsheet.rows[0].cells[1].value, "bar");
    }

    #[test]
    fn parse_with_options() {
        let source_code = build_source_code("[[t=b / fs=20]]foo");
        let spreadsheet = Spreadsheet::parse(source_code).unwrap();

        assert!(spreadsheet.rows[0].cells[0]
            .text_formats
            .contains(&TextFormat::Bold));
        assert_eq!(spreadsheet.rows[0].cells[0].font_size, Some(20))
    }

    #[test]
    fn parse_with_row_option() {
        let source_code = build_source_code("![[t=b]]foo,bar,baz");
        let spreadsheet = Spreadsheet::parse(source_code).unwrap();

        assert!(spreadsheet.rows[0].cells[0]
            .text_formats
            .contains(&TextFormat::Bold));
        assert!(spreadsheet.rows[0].cells[1]
            .text_formats
            .contains(&TextFormat::Bold));
        assert!(spreadsheet.rows[0].cells[2]
            .text_formats
            .contains(&TextFormat::Bold));
    }

    #[test]
    fn variables_unscoped() {
        let spreadsheet = Spreadsheet {
            rows: vec![Row {
                cells: vec![
                    Cell {
                        var: Some("foo".to_string()),
                        ..Default::default()
                    },
                    Cell {
                        var: Some("bar".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
        };

        let variables = spreadsheet.variables();
        assert_eq!(
            **variables.get("foo").unwrap(),
            Node::var("foo", VariableValue::Absolute(Address::new(0, 0)))
        );
        assert_eq!(
            **variables.get("bar").unwrap(),
            Node::var("bar", VariableValue::Absolute(Address::new(1, 0)))
        );
    }

    #[test]
    fn variables_with_scope() {
        let spreadsheet = Spreadsheet {
            rows: vec![
                Row {
                    fill: Some(Fill::new(0, Some(10))),
                    cells: vec![Cell {
                        var: Some("foo".to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Row {
                    fill: Some(Fill::new(10, Some(100))),
                    var: Some("bar".to_string()),
                    ..Default::default()
                },
            ],
        };

        let variables = spreadsheet.variables();
        assert_eq!(
            **variables.get("foo").unwrap(),
            Node::var(
                "foo",
                VariableValue::ColumnRelative {
                    fill: Fill {
                        amount: Some(10),
                        start_row: 0.into()
                    },
                    column: 0.into(),
                }
            )
        );
        assert_eq!(
            **variables.get("bar").unwrap(),
            Node::var(
                "bar",
                VariableValue::RowRelative {
                    fill: Fill {
                        amount: Some(100),
                        start_row: 10.into()
                    },
                    row: 1.into(),
                }
            )
        );
    }

    #[test]
    fn widest_row() {
        let cell = Cell {
            value: "foo".to_string(),
            ..Default::default()
        };
        let spreadsheet = Spreadsheet {
            rows: vec![
                Row {
                    cells: vec![cell.clone()],
                    ..Default::default()
                },
                Row {
                    cells: vec![cell.clone(), cell.clone()],
                    ..Default::default()
                },
                Row {
                    cells: vec![cell.clone(), cell.clone(), cell.clone()],
                    ..Default::default()
                },
            ],
        };

        assert_eq!(spreadsheet.widest_row(), 3);
    }
}
