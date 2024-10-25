//! # Excel
//!
//! Functions for writing compiled modules to Excel
//!
use super::{merge_cell, ExistingCell, MergeResult};
use crate::ast::Node;
use crate::{Cell, Compiler, Module, Result};
use std::ffi;
use std::path;
use umya_spreadsheet as u;

mod cell_validation;
mod compilation_target;
mod excel_cell;

use cell_validation::CellValidation;

type ExcelValue = u::Cell;

#[derive(Debug)]
pub(crate) struct Excel<'a> {
    path: path::PathBuf,
    compiler: &'a Compiler,
}

impl<'a> Excel<'a> {
    pub(crate) fn new<P: Into<path::PathBuf>>(compiler: &'a Compiler, path: P) -> Self {
        Self {
            path: path.into(),
            compiler,
        }
    }

    pub(crate) fn supports_extension(os_str: &ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("xlsx")
            || os_str.eq_ignore_ascii_case("xlsm")
            || os_str.eq_ignore_ascii_case("xltx")
            || os_str.eq_ignore_ascii_case("xltm")
    }

    /// Since the excel library allows us to modify the speadsheet in place, the strategy here is
    /// to be as light-touch as possible and just loop through our values and set them (or not
    /// depending on the merge strategy).
    fn build_worksheet(&self, module: &Module, worksheet: &mut u::Worksheet) {
        let mut cell_validations = Vec::with_capacity(module.spreadsheet.rows.len());

        for (row_index, row) in module.spreadsheet.rows.iter().enumerate() {
            for (cell_index, cell) in row.cells.iter().enumerate() {
                let position = a1::Address::new(cell_index, row_index);

                let merged_cell = merge_cell(
                    &Self::get_existing_cell(position, worksheet),
                    Some(cell),
                    &self.compiler.options,
                );

                match merged_cell {
                    // if the value already exists we don't need to do anything because we're just
                    // writing changes with this strategy
                    MergeResult::Existing(_) | MergeResult::Empty => (),

                    // build a new value
                    MergeResult::New(cell) => {
                        let e = worksheet.get_cell_mut(position.to_string());

                        Self::set_value(e, &cell);

                        if let Some(style) = Self::build_style(&cell) {
                            e.set_style(style);
                        }

                        if let Some(n) = &cell.note {
                            Self::set_comment(worksheet, position, n);
                        }

                        if let Some(data_validation) = cell.data_validation {
                            cell_validations.push(CellValidation(position, data_validation));
                        }
                    }
                }
            }
        }

        Self::set_data_validations(worksheet, cell_validations);
    }

    fn set_data_validations(worksheet: &mut u::Worksheet, cell_validations: Vec<CellValidation>) {
        let mut validations = u::DataValidations::default();
        if cell_validations.is_empty() {
            return;
        }

        validations.set_data_validation_list(
            cell_validations
                .into_iter()
                .map(std::convert::Into::into)
                .collect(),
        );
        worksheet.set_data_validations(validations);
    }

    fn set_comment(worksheet: &mut u::Worksheet, position: a1::Address, note: &str) {
        let mut comment = u::Comment::default();
        comment.set_author("csvpp");

        let rt = comment.get_text_mut();
        rt.set_text(note);

        let coord = comment.get_coordinate_mut();
        coord.set_col_num(u32::try_from(position.column.x).expect("32 bit coordinate for comment"));
        coord.set_row_num(u32::try_from(position.row.y).expect("32 bit coordinate for comment"));

        worksheet.add_comments(comment);
    }

    // TODO: turn into an impl (from/into)? the problem is we're mutating existing_cell...
    fn set_value(existing_cell: &mut u::Cell, cell: &Cell) {
        if let Some(ast) = &cell.ast {
            match ast.clone().into_inner() {
                Node::Boolean(b) => existing_cell.set_value_bool(b),
                Node::Text(t) => existing_cell.set_value_string(t),
                Node::Float {
                    value,
                    percentage,
                    sign: None,
                } if !percentage => existing_cell.set_value_number(value),
                #[allow(clippy::cast_precision_loss)]
                Node::Integer {
                    value,
                    percentage,
                    sign: None,
                } if !percentage => existing_cell.set_value_number(value as f64),
                _ => existing_cell.set_formula(ast.to_string()),
            };
        } else if !cell.value.is_empty() {
            existing_cell.set_value_string(cell.value.clone());
        }
    }

    fn build_style(cell: &Cell) -> Option<u::Style> {
        let excel_cell = excel_cell::ExcelCell(cell);
        if !excel_cell.has_style() {
            return None;
        }

        Some(excel_cell.into())
    }

    fn get_existing_cell(
        position: a1::Address,
        worksheet: &u::Worksheet,
    ) -> ExistingCell<ExcelValue> {
        let cell_value = worksheet.get_cell(position.to_string());
        if let Some(cell) = cell_value {
            ExistingCell::Value(cell.clone())
        } else {
            ExistingCell::Empty
        }
    }

    fn open_spreadsheet(&self) -> Result<u::Spreadsheet> {
        if self.path.exists() {
            u::reader::xlsx::read(self.path.as_path()).map_err(|e| {
                self.compiler
                    .output_error(format!("Unable to open target file: {e}"))
            })
        } else {
            Ok(u::new_file_empty_worksheet())
        }
    }

    fn create_worksheet(&self, spreadsheet: &mut u::Spreadsheet) -> Result<()> {
        let sheet_name = self.compiler.options.sheet_name.clone();
        let existing = spreadsheet.get_sheet_by_name(&sheet_name);
        if existing.is_none() {
            spreadsheet.new_sheet(&sheet_name).map_err(|e| {
                self.compiler.output_error(format!(
                    "Unable to create new worksheet {sheet_name} in target file: {e}"
                ))
            })?;
        }

        Ok(())
    }

    fn get_worksheet_mut(
        &'a self,
        spreadsheet: &'a mut u::Spreadsheet,
    ) -> Result<&'a mut u::Worksheet> {
        let sheet_name = &self.compiler.options.sheet_name;
        spreadsheet
            .get_sheet_by_name_mut(sheet_name)
            .ok_or_else(|| {
                self.compiler.output_error(format!(
                    "Unable to open worksheet {sheet_name} in target file"
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn open_worksheet_does_exist() {
        let compiler = build_compiler();
        let setup = TestFile::new("xlsx", "");
        let spreadsheet = Excel::new(&compiler, setup.output).open_worksheet().unwrap();

        // assert!(spreadsheet.is_ok());
    }

    #[test]
    fn open_worksheet_does_not_exist() {
        let compiler = build_compiler();
        let filename = path::PathBuf::from("foobar.xlsx");
        let spreadsheet = Excel::new(&compiler, filename).open_worksheet().unwrap();

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
