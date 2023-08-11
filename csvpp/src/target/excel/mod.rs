//! # Excel
//!
//! Functions for writing compiled templates to Excel
//!
mod excel_modifier;
mod compilation_target;

use std::ffi;
use std::path;
use crate::{Error, Result, Runtime, SpreadsheetCell, Template};
use crate::ast::Node;
use super::{ merge_cell, ExistingCell, MergeResult };

type ExcelValue = umya_spreadsheet::Cell;

#[derive(Debug)]
pub struct Excel<'a> {
    path: path::PathBuf,
    runtime: &'a Runtime,
}

impl<'a> Excel<'a> {
    pub fn new(runtime: &'a Runtime, path: path::PathBuf) -> Self {
        Self { path, runtime }
    }

    pub fn supports_extension(os_str: &ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("xlsx")
            || os_str.eq_ignore_ascii_case("xlsm")
            || os_str.eq_ignore_ascii_case("xltx")
            || os_str.eq_ignore_ascii_case("xltm")
    }

    /// Since the excel library allows us to modify the speadsheet in place, the strategy here is
    /// to be a light-touch as possible and just loop through our values and set them (or not
    /// depending on the merge strategy).
    fn build_worksheet(
        &self,
        template: &Template,
        worksheet: &mut umya_spreadsheet::Worksheet,
    ) -> Result<()> {
        let s = template.spreadsheet.borrow();
        for row in &s.cells {
            for cell in row {
                let merged_cell = merge_cell(
                    &self.get_existing_cell(&cell.position, worksheet),
                    Some(cell),
                    &self.runtime.options,
                );

                match merged_cell {
                    // if the value already exists we don't need to do anything because we're just
                    // writing changes with this strategy
                    MergeResult::Existing(_) | MergeResult::Empty => (),

                    // build a new value
                    MergeResult::New(cell) => {
                        let e = worksheet.get_cell_mut(cell.position.to_string());

                        self.set_value(e, &cell);

                        if let Some(style) = self.build_style(&cell) {
                            e.set_style(style);
                        }
                    },
                }
            }
        }

        Ok(())
    }

    fn set_value(
        &self,
        existing_cell: &mut umya_spreadsheet::Cell,
        cell: &SpreadsheetCell,
    ) {
        if let Some(ast) = &cell.ast {
            match *ast.clone() {
                Node::Boolean(b) => 
                    existing_cell.set_value_bool(b),
                Node::Text(t) =>
                    existing_cell.set_value_string(t),
                Node::Float(f) =>
                    existing_cell.set_value_number(f),
                Node::Integer(i) =>
                    existing_cell.set_value_number(i as f64),
                _ => 
                    existing_cell.set_formula(ast.to_string()),
            };
        } else if !cell.value.is_empty() {
            existing_cell.set_value_string(cell.value.clone());
        }
    }

    fn build_style(&self, cell: &SpreadsheetCell) -> Option<umya_spreadsheet::Style> {
        let modifier = cell.modifier.clone();
        if modifier.is_empty() {
            return None
        }

        Some(excel_modifier::ExcelModifier(modifier).into())
    }

    fn get_existing_cell(
        &self,
        position: &a1_notation::A1,
        worksheet: &umya_spreadsheet::Worksheet,
    ) -> ExistingCell<ExcelValue> {
        let cell_value = worksheet.get_cell(position.to_string());
        if let Some(cell) = cell_value {
            ExistingCell::Value(cell.clone())
        } else {
            ExistingCell::Empty
        }
    }

    fn open_spreadsheet(&self) -> Result<umya_spreadsheet::Spreadsheet> {
        if self.path.exists() {
            umya_spreadsheet::reader::xlsx::read(self.path.as_path()).map_err(|e| {
                Error::TargetWriteError {
                    message: format!("Unable to open target file: {}", e),
                    output: self.runtime.output.clone(),
                }
            })
        } else {
            Ok(umya_spreadsheet::new_file())
        }
    }

    fn create_worksheet(&self, spreadsheet: &mut umya_spreadsheet::Spreadsheet) -> Result<()> {
        let sheet_name = self.runtime.options.sheet_name.clone();

        let existing = spreadsheet.get_sheet_by_name(&sheet_name);
        if existing.is_err() {
            spreadsheet.new_sheet(&sheet_name).map_err(|e| {
                Error::TargetWriteError {
                    message: format!("Unable to create new worksheet {} in target file: {}", sheet_name, e),
                    output: self.runtime.output.clone(),
                }
            })?;
        }

        Ok(())
    }

    fn get_worksheet_mut(&'a self, spreadsheet: &'a mut umya_spreadsheet::Spreadsheet) -> Result<&'a mut umya_spreadsheet::Worksheet> {
        let sheet_name = &self.runtime.options.sheet_name;
        spreadsheet.get_sheet_by_name_mut(sheet_name).map_err(|e| {
            Error::TargetWriteError {
                message: format!("Unable to open worksheet {} in target file: {}", sheet_name, e),
                output: self.runtime.output.clone(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn open_worksheet_does_exist() {
        let runtime = build_runtime();
        let setup = TestFile::new("xlsx", "");
        let spreadsheet = Excel::new(&runtime, setup.output).open_worksheet().unwrap();

        // assert!(spreadsheet.is_ok());
    }

    #[test]
    fn open_worksheet_does_not_exist() {
        let runtime = build_runtime();
        let filename = path::PathBuf::from("foobar.xlsx");
        let spreadsheet = Excel::new(&runtime, filename).open_worksheet().unwrap();

        // assert!(spreadsheet.is_ok());
    }
    */

    #[test]
    fn supports_extension_true() {
        assert!(Excel::supports_extension(ffi::OsStr::new("xlsx")));
        assert!(Excel::supports_extension(ffi::OsStr::new("XLSX")));
        assert!(Excel::supports_extension(ffi::OsStr::new("xlsm")));
        assert!(Excel::supports_extension(ffi::OsStr::new("xltm")));
        assert!(Excel::supports_extension(ffi::OsStr::new("xltx")));
    }

    #[test]
    fn supports_extension_false() {
        assert!(!Excel::supports_extension(ffi::OsStr::new("foo")));
        assert!(!Excel::supports_extension(ffi::OsStr::new("csv")));
    }
}
