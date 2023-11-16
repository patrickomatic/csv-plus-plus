//! # BatchUpdateBuilder
//!
use super::{google_sheets_cell, SheetsValue};
use crate::ast::Node;
use crate::target::{merge_rows, ExistingValues, MergeResult};
use crate::{Cell, Module, Runtime};
use google_sheets4::api;
use std::str::FromStr;

pub(super) struct BatchUpdateBuilder<'a> {
    existing_values: &'a ExistingValues<SheetsValue>,
    runtime: &'a Runtime,
    module: &'a Module,
}

impl<'a> BatchUpdateBuilder<'a> {
    pub(super) fn new(
        runtime: &'a Runtime,
        module: &'a Module,
        existing_values: &'a ExistingValues<SheetsValue>,
    ) -> Self {
        Self {
            existing_values,
            runtime,
            module,
        }
    }

    /// Loops over each row of the spreadsheet, building up `UpdateCellsRequest`s.  
    pub(super) fn build(&self) -> api::BatchUpdateSpreadsheetRequest {
        api::BatchUpdateSpreadsheetRequest {
            requests: Some(self.batch_update_cells_requests()),
            ..Default::default()
        }
    }

    fn batch_update_cells_requests(&self) -> Vec<api::Request> {
        let all_rows = self.row_data();

        // split the rows into requests with 1000 per
        all_rows
            .chunks(1000)
            .map(|rows| api::Request {
                update_cells: Some(self.update_cells_request(rows)),
                ..Default::default()
            })
            .collect()
    }

    fn cell_data(&self, row: &[MergeResult<SheetsValue>]) -> Vec<api::CellData> {
        row.iter()
            .map(|cell| {
                match cell {
                    // just give back the data as we got it
                    MergeResult::Existing(cell_data) => cell_data.clone(),

                    // TODO: can I just return None or something?
                    MergeResult::Empty => api::CellData::default(),

                    // build a new value
                    MergeResult::New(cell) => {
                        let gs_cell = google_sheets_cell::GoogleSheetsCell(cell);
                        api::CellData {
                            data_validation: gs_cell.data_validation_rule(),
                            user_entered_format: gs_cell.cell_format(),
                            user_entered_value: self.user_entered_value(cell),
                            note: cell.note.clone(),
                            ..Default::default()
                        }
                    }
                }
            })
            .collect()
    }

    fn row_data(&self) -> Vec<api::RowData> {
        let spreadsheet = self.module.spreadsheet.borrow();

        spreadsheet
            .rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let empty_row = vec![];
                let existing_row = self.existing_values.cells.get(i).unwrap_or(&empty_row);
                let merged_row = merge_rows(existing_row, &row.cells, &self.runtime.options);

                api::RowData {
                    values: Some(self.cell_data(&merged_row)),
                }
            })
            .collect()
    }

    fn update_cells_request(&self, rows: &[api::RowData]) -> api::UpdateCellsRequest {
        api::UpdateCellsRequest {
            fields: Some(google_sheets4::FieldMask::from_str("*").unwrap()),
            start: Some(api::GridCoordinate {
                column_index: Some(self.runtime.options.offset.1 as i32),
                row_index: Some(self.runtime.options.offset.0 as i32),
                sheet_id: None,
            }),
            rows: Some(rows.to_vec()),
            range: None,
        }
    }

    fn user_entered_value(&self, cell: &Cell) -> Option<api::ExtendedValue> {
        if let Some(ast) = &cell.ast {
            Some(match *ast.clone() {
                Node::Boolean(b) => api::ExtendedValue {
                    bool_value: Some(b),
                    ..Default::default()
                },
                Node::Text(t) => api::ExtendedValue {
                    string_value: Some(t),
                    ..Default::default()
                },
                Node::Float(f) => api::ExtendedValue {
                    number_value: Some(f),
                    ..Default::default()
                },
                Node::Integer(i) => api::ExtendedValue {
                    number_value: Some(i as f64),
                    ..Default::default()
                },
                _ => api::ExtendedValue {
                    formula_value: Some(format!("={ast}")),
                    ..Default::default()
                },
            })
        } else if cell.value.is_empty() {
            None
        } else {
            Some(api::ExtendedValue {
                string_value: Some(cell.value.clone()),
                ..Default::default()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn build() {
        let runtime = build_runtime();

        let mut spreadsheet = Spreadsheet::default();
        spreadsheet.rows.push(Row {
            cells: vec![Cell {
                value: "Test".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        });

        let module = Module::new(spreadsheet, None, &runtime).unwrap();
        let existing_values = ExistingValues { cells: vec![] };
        let builder = BatchUpdateBuilder::new(&runtime, &module, &existing_values).build();

        assert_eq!(1, builder.requests.unwrap().len());
    }
}
