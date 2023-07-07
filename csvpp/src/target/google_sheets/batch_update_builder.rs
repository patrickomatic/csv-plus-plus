//! # BatchUpdateBuilder
//!
// TODO:
// 
// * fix unwraps
use google_sheets4::api;
use std::str::FromStr;
use crate::{Runtime, SpreadsheetCell, Template};
use super::google_sheets_modifier;

pub struct BatchUpdateBuilder<'a> {
    runtime: &'a Runtime,
    template: &'a Template,
}

impl<'a> BatchUpdateBuilder<'a> {
    pub fn new(runtime: &'a Runtime, template: &'a Template) -> Self {
        Self { runtime, template }
    }

    /// Loops over each row of the spreadsheet, building up `UpdateCellsRequest`s.  
    /// 
    // TODO:
    // 
    // * it needs to be limited to chunks of 1000 
    pub fn build(&self) -> api::BatchUpdateSpreadsheetRequest {
        let requests = self.batch_update_cells_requests();
        // TODO: I might not need to do this...
        // requests.push(update_borders_request(template));

        api::BatchUpdateSpreadsheetRequest {
            requests: Some(requests), 
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

    fn cell_data(&self, row: &[SpreadsheetCell]) -> Vec<api::CellData> {
        row.iter()
            .map(|cell| {
                let modifier = google_sheets_modifier::GoogleSheetsModifier(&cell.modifier);
                api::CellData {
                    user_entered_format: modifier.cell_format(),
                    note: cell.modifier.note.clone(),

                    ..Default::default()
                }
            })
            .collect()
    }

    fn row_data(&self) -> Vec<api::RowData> {
        let spreadsheet = self.template.spreadsheet.borrow();

        spreadsheet
            .cells
            .iter()
            .map(|row| api::RowData { values: Some(self.cell_data(row)) })
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
}
