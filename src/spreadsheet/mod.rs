//! # Spreadsheet
//!
//!
use crate::ast::{Node, VariableValue, Variables};
use crate::{csv_reader, Result, Row, Runtime};
use serde::{Deserialize, Serialize};
use std::collections;

mod display;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Spreadsheet {
    pub rows: Vec<Row>,
}

impl Spreadsheet {
    /// Parse the spreadsheet section of a csv++ source file.
    pub(crate) fn parse(runtime: &Runtime) -> Result<Spreadsheet> {
        runtime.progress("Parsing spreadsheet");

        let mut csv_reader = csv_reader()
            .trim(csv::Trim::All)
            .from_reader(runtime.source_code.csv_section.as_bytes());

        let mut rows: Vec<Row> = vec![];
        for (row_index, result) in csv_reader.records().enumerate() {
            let row = Row::parse(result, row_index, runtime)?;
            rows.push(row);
        }

        Ok(Spreadsheet { rows })
    }

    /// Extract all of the variables that were defined by cells contained in this spreadsheet
    //
    // NOTE: we could also store these in a HashMap on the Spreadsheet as we build it rather than
    // parsing them out at runtime
    pub(crate) fn variables(&self) -> Variables {
        let mut vars = collections::HashMap::new();

        for row in &self.rows {
            // does the row itself have a var?
            if let Some(var_id) = &row.modifier.var {
                let reference = if let Some(scope) = row.modifier.expand {
                    // if there's also an expand it's relative to that
                    Node::Variable {
                        name: var_id.clone(),
                        value: VariableValue::RowRelative {
                            scope,
                            row: row.row,
                        },
                    }
                } else {
                    // otherwise it's just relative to the single row where it was defined
                    Node::Variable {
                        name: var_id.clone(),
                        value: VariableValue::Row(row.row),
                    }
                };

                vars.insert(var_id.to_owned(), Box::new(reference));
            };

            row.cells.iter().for_each(|c| {
                if let Some(var_id) = &c.modifier.var {
                    let reference = if let Some(scope) = row.modifier.expand {
                        Node::Variable {
                            name: var_id.clone(),
                            value: VariableValue::ColumnRelative {
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
    use crate::modifier::TextFormat;
    use crate::test_utils::*;
    use crate::*;
    use a1_notation::Address;

    fn build_runtime(input: &str) -> Runtime {
        TestFile::new("csv", input).into()
    }

    #[test]
    fn parse_simple() {
        let runtime = build_runtime("foo,bar,baz\n1,2,3\n");
        let spreadsheet = Spreadsheet::parse(&runtime).unwrap();

        // 2 rows
        assert_eq!(spreadsheet.rows.len(), 2);

        // each row has 3 cells
        assert_eq!(spreadsheet.rows[0].cells.len(), 3);
        assert_eq!(spreadsheet.rows[1].cells.len(), 3);

        // the cells have the correct positions
        assert_eq!(spreadsheet.rows[0].cells[0].position.to_string(), "A1");
        assert_eq!(spreadsheet.rows[0].cells[1].position.to_string(), "B1");
        assert_eq!(spreadsheet.rows[0].cells[2].position.to_string(), "C1");
        assert_eq!(spreadsheet.rows[1].cells[0].position.to_string(), "A2");
        assert_eq!(spreadsheet.rows[1].cells[1].position.to_string(), "B2");
        assert_eq!(spreadsheet.rows[1].cells[2].position.to_string(), "C2");

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
        let runtime = build_runtime("=1,=2 * 3,=foo\n");
        let spreadsheet = Spreadsheet::parse(&runtime).unwrap();

        assert!(spreadsheet.rows[0].cells[0].ast.is_some());
        assert!(spreadsheet.rows[0].cells[1].ast.is_some());
        assert!(spreadsheet.rows[0].cells[2].ast.is_some());
    }

    #[test]
    fn parse_trim_spaces() {
        let runtime = build_runtime("   foo   , bar\n");
        let spreadsheet = Spreadsheet::parse(&runtime).unwrap();

        assert_eq!(spreadsheet.rows[0].cells[0].value, "foo");
        assert_eq!(spreadsheet.rows[0].cells[1].value, "bar");
    }

    #[test]
    fn parse_with_modifiers() {
        let runtime = build_runtime("[[f=b / fs=20]]foo");
        let spreadsheet = Spreadsheet::parse(&runtime).unwrap();

        assert!(spreadsheet.rows[0].cells[0]
            .modifier
            .formats
            .contains(&TextFormat::Bold));
        assert_eq!(spreadsheet.rows[0].cells[0].modifier.font_size, Some(20))
    }

    #[test]
    fn parse_with_row_modifier() {
        let runtime = build_runtime("![[f=b]]foo,bar,baz");
        let spreadsheet = Spreadsheet::parse(&runtime).unwrap();

        assert!(spreadsheet.rows[0].cells[0]
            .modifier
            .formats
            .contains(&TextFormat::Bold));
        assert!(spreadsheet.rows[0].cells[1]
            .modifier
            .formats
            .contains(&TextFormat::Bold));
        assert!(spreadsheet.rows[0].cells[2]
            .modifier
            .formats
            .contains(&TextFormat::Bold));
    }

    #[test]
    fn variables_unscoped() {
        let spreadsheet = Spreadsheet {
            rows: vec![Row {
                row: 0.into(),
                modifier: RowModifier::default(),
                cells: vec![
                    Cell {
                        ast: None,
                        position: Address::new(0, 0),
                        modifier: Modifier {
                            var: Some("foo".to_string()),
                            ..Default::default()
                        },
                        value: "".to_string(),
                    },
                    Cell {
                        ast: None,
                        position: Address::new(1, 1),
                        modifier: Modifier {
                            var: Some("bar".to_string()),
                            ..Default::default()
                        },
                        value: "".to_string(),
                    },
                ],
            }],
        };

        let variables = spreadsheet.variables();
        assert_eq!(
            **variables.get("foo").unwrap(),
            Node::var("foo", VariableValue::Absolute(Address::new(0, 0)))
        );
        assert_eq!(
            **variables.get("bar").unwrap(),
            Node::var("bar", VariableValue::Absolute(Address::new(1, 1)))
        );
    }

    #[test]
    fn variables_with_scope() {
        let spreadsheet = Spreadsheet {
            rows: vec![
                Row {
                    row: 0.into(),
                    modifier: RowModifier {
                        expand: Some(Expand::new(0, Some(10))),
                        ..Default::default()
                    },
                    cells: vec![Cell {
                        ast: None,
                        position: (0, 0).into(),
                        modifier: Modifier {
                            var: Some("foo".to_string()),
                            ..Default::default()
                        },
                        value: "".to_string(),
                    }],
                },
                Row {
                    row: 1.into(),
                    modifier: RowModifier {
                        expand: Some(Expand::new(10, Some(100))),
                        ..Default::default()
                    },
                    cells: vec![Cell {
                        ast: None,
                        position: (1, 1).into(),
                        modifier: Modifier {
                            var: Some("bar".to_string()),
                            ..Default::default()
                        },
                        value: "".to_string(),
                    }],
                },
            ],
        };

        let variables = spreadsheet.variables();
        assert_eq!(
            **variables.get("foo").unwrap(),
            Node::var(
                "foo",
                VariableValue::ColumnRelative {
                    scope: Expand {
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
                VariableValue::ColumnRelative {
                    scope: Expand {
                        amount: Some(100),
                        start_row: 10.into()
                    },
                    column: 1.into(),
                }
            )
        );
    }

    #[test]
    fn widest_row() {
        let cell = Cell {
            ast: None,
            position: Address::new(0, 0),
            modifier: Modifier::default(),
            value: "foo".to_string(),
        };
        let spreadsheet = Spreadsheet {
            rows: vec![
                Row {
                    cells: vec![cell.clone()],
                    row: 0.into(),
                    modifier: RowModifier::default(),
                },
                Row {
                    cells: vec![cell.clone(), cell.clone()],
                    row: 1.into(),
                    modifier: RowModifier::default(),
                },
                Row {
                    cells: vec![cell.clone(), cell.clone(), cell.clone()],
                    row: 2.into(),
                    modifier: RowModifier::default(),
                },
            ],
        };

        assert_eq!(spreadsheet.widest_row(), 3);
    }
}
