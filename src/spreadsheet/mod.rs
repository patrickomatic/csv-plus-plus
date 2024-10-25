//! # Spreadsheet
//!
use crate::ast::{Ast, Node, VariableValue, Variables};
use crate::fill::ROW_MAX;
use crate::{csv_reader, ArcSourceCode, EvalError, EvalResult, Result, Row};
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections;

mod display;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Spreadsheet {
    pub rows: Vec<Row>,
    pub(crate) fills_expanded: bool,
}

impl Spreadsheet {
    /// For each row of the spreadsheet, if it has a [[fill=]] then we need to actually fill it to
    /// that many rows.  
    ///
    /// This has to happen before eval()ing the cells because that process depends on them being in
    /// their final location.
    pub(crate) fn eval_fills(self) -> EvalResult<Self> {
        if self.fills_expanded {
            return Ok(self);
        } else if self.has_multiple_infinite_expands() {
            return Err(EvalError::new(
                "![[fill]]",
                "Multiple infinite fills - \
                    you can only have one without a fill amount and all others must supply an \
                    amount (i.e., `![[fill=10]]` rather than `![[fill]]`)",
            ));
        }

        let mut rows = Vec::with_capacity(self.rows.len());
        let mut row_num = 0;
        let rows_left_over = self.rows_left_over();

        for row in self.rows {
            if let Some(f) = row.fill {
                let new_fill = f.clone_to_row(row_num);
                let fill_amount = new_fill.amount.unwrap_or(rows_left_over);

                for _ in 0..fill_amount {
                    rows.push(Row {
                        fill: Some(new_fill),
                        ..row.clone()
                    });
                    row_num += 1;
                }
            } else {
                rows.push(row);
                row_num += 1;
            }
        }

        Ok(Self {
            rows,
            fills_expanded: true,
        })
    }

    /// Parse the spreadsheet section of a csv++ source file.
    pub(crate) fn parse(source_code: &ArcSourceCode) -> Result<Spreadsheet> {
        let mut csv_reader = csv_reader()
            .trim(csv::Trim::None)
            .from_reader(source_code.csv_section.as_bytes());

        let mut rows: Vec<Row> = vec![];
        for (row_index, result) in csv_reader.records().enumerate() {
            trace!("Parsing row {row_index}");
            rows.push(Row::parse(result, row_index.into(), source_code)?);
        }

        Ok(Spreadsheet {
            rows,
            fills_expanded: false,
        })
    }

    /// Extract all of the variables that were defined by cells contained in this spreadsheet
    //
    // NOTE: we could also store these in a HashMap on the Spreadsheet as we build it rather than
    // parsing them out at compile-time
    pub(crate) fn variables(&self) -> Variables {
        let mut vars = collections::HashMap::new();

        for (row_index, row) in self.rows.iter().enumerate() {
            let row_a1: a1::Row = row_index.into();

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
                let cell_a1 = a1::Address::new(cell_index, row_index);

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

    fn has_multiple_infinite_expands(&self) -> bool {
        let mut saw_infinite = false;
        for row in &self.rows {
            if let Some(fill) = row.fill {
                if fill.amount.is_none() {
                    if saw_infinite {
                        return true;
                    }
                    saw_infinite = true;
                }
            }
        }

        false
    }

    fn rows_left_over(&self) -> usize {
        let total_rows_minus_infinite = self.rows.iter().fold(0, |acc, row| {
            acc + row.fill.map_or(1, |f| f.amount.unwrap_or(0))
        });

        ROW_MAX.saturating_sub(total_rows_minus_infinite)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::*;
    use a1::Address;

    fn build_source_code(input: &str) -> ArcSourceCode {
        ArcSourceCode::new((&TestSourceCode::new("csv", input)).into())
    }

    #[test]
    fn eval_fills_already_expanded() {
        let spreadsheet = Spreadsheet {
            rows: vec![
                Row {
                    fill: Some(Fill::new(0, Some(10))),
                    ..Default::default()
                },
                Row {
                    fill: Some(Fill::new(10, Some(30))),
                    ..Default::default()
                },
            ],
            fills_expanded: true,
        }
        .eval_fills()
        .unwrap();

        assert_eq!(spreadsheet.rows.len(), 2);
    }

    #[test]
    fn eval_fills_finite() {
        let spreadsheet = Spreadsheet {
            rows: vec![
                Row {
                    fill: Some(Fill::new(0, Some(10))),
                    ..Default::default()
                },
                Row {
                    fill: Some(Fill::new(10, Some(30))),
                    ..Default::default()
                },
            ],
            fills_expanded: false,
        }
        .eval_fills()
        .unwrap();

        assert_eq!(spreadsheet.rows.len(), 40);
        // 0-9 should be Fill { amount: 10, start_row: 0 }
        assert_eq!(spreadsheet.rows[0].fill.unwrap().start_row, 0.into());
        assert_eq!(spreadsheet.rows[9].fill.unwrap().start_row, 0.into());
        // and 10-39 should be Fill { amount: 30, start_row: 10 }
        assert_eq!(spreadsheet.rows[10].fill.unwrap().start_row, 10.into());
        assert_eq!(spreadsheet.rows[39].fill.unwrap().start_row, 10.into());
    }

    #[test]
    fn eval_fills_infinite() {
        let spreadsheet = Spreadsheet {
            rows: vec![
                Row {
                    fill: Some(Fill::new(0, Some(10))),
                    ..Default::default()
                },
                Row {
                    fill: Some(Fill::new(10, None)),
                    ..Default::default()
                },
            ],
            fills_expanded: false,
        }
        .eval_fills()
        .unwrap();

        assert_eq!(spreadsheet.rows.len(), 1000);
        // 0-9 should be Fill { amount: 10, start_row: 0 }
        assert_eq!(spreadsheet.rows[0].fill.unwrap().start_row, 0.into());
        assert_eq!(spreadsheet.rows[9].fill.unwrap().start_row, 0.into());
        // and 10-999 should be Fill { amount: None, start_row: 10 }
        assert_eq!(spreadsheet.rows[10].fill.unwrap().start_row, 10.into());
        assert_eq!(spreadsheet.rows[999].fill.unwrap().start_row, 10.into());
    }

    #[test]
    fn eval_fills_multiple_and_infinite() {
        let spreadsheet = Spreadsheet {
            rows: vec![
                Row {
                    fill: Some(Fill::new(0, Some(10))),
                    ..Default::default()
                },
                Row {
                    fill: Some(Fill::new(10, None)),
                    ..Default::default()
                },
                Row {
                    fill: Some(Fill::new(990, Some(10))),
                    ..Default::default()
                },
            ],
            fills_expanded: false,
        }
        .eval_fills()
        .unwrap();

        assert_eq!(spreadsheet.rows.len(), 1000);
    }

    #[test]
    fn eval_fills_multiple_infinite() {
        assert!(Spreadsheet {
            rows: vec![
                Row {
                    fill: Some(Fill::new(0, None)),
                    ..Default::default()
                },
                Row {
                    fill: Some(Fill::new(10, None)),
                    ..Default::default()
                },
            ],
            fills_expanded: false,
        }
        .eval_fills()
        .is_err());
    }

    #[test]
    fn parse_simple() {
        let source_code = build_source_code("foo,bar,baz\n1,2,3\n");
        let spreadsheet = Spreadsheet::parse(&source_code).unwrap();

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
        let spreadsheet = Spreadsheet::parse(&source_code).unwrap();

        assert!(spreadsheet.rows[0].cells[0].ast.is_some());
        assert!(spreadsheet.rows[0].cells[1].ast.is_some());
        assert!(spreadsheet.rows[0].cells[2].ast.is_some());
    }

    #[test]
    fn parse_trim_spaces() {
        let source_code = build_source_code("   foo   , bar\n");
        let spreadsheet = Spreadsheet::parse(&source_code).unwrap();

        assert_eq!(spreadsheet.rows[0].cells[0].value, "foo");
        assert_eq!(spreadsheet.rows[0].cells[1].value, "bar");
    }

    #[test]
    fn parse_with_options() {
        let source_code = build_source_code("[[t=b / fs=20]]foo");
        let spreadsheet = Spreadsheet::parse(&source_code).unwrap();

        assert!(spreadsheet.rows[0].cells[0]
            .text_formats
            .contains(&TextFormat::Bold));
        assert_eq!(spreadsheet.rows[0].cells[0].font_size, Some(20));
    }

    #[test]
    fn parse_with_row_option() {
        let source_code = build_source_code("![[t=b]]foo,bar,baz");
        let spreadsheet = Spreadsheet::parse(&source_code).unwrap();

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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        };

        assert_eq!(spreadsheet.widest_row(), 3);
    }
}
