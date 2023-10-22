//! # Excel
//!
//! Functions for writing compiled templates to Excel
//!
use super::{merge_cell, ExistingCell, MergeResult};
use crate::ast::Node;
use crate::modifier::DataValidation;
use crate::{Cell, Error, Result, Runtime, Template};
use a1_notation::Address;
use std::ffi;
use std::path;
use umya_spreadsheet as u;

mod compilation_target;
mod excel_modifier;

type ExcelValue = u::Cell;

#[derive(Debug)]
pub(crate) struct Excel<'a> {
    path: path::PathBuf,
    runtime: &'a Runtime,
}

#[derive(Debug)]
pub(crate) struct CellValidation(a1_notation::Address, DataValidation);

// TODO:
// * optimize this so there isn't a separate data validation for each cell - if a data validation
//     is placed on a fill, we can specify the range covered by that fill instead of each cell
// * finish the unimplemented ones
// * .set_allow_blank()? does GS allow that?
// * .set_prompt_title() too?
// * does it matter that I use Decimal rather than Whole?
impl From<CellValidation> for u::DataValidation {
    fn from(CellValidation(position, dv): CellValidation) -> Self {
        let mut sqref = u::SequenceOfReferences::default();
        sqref.set_sqref(position.to_string());

        let mut validation = u::DataValidation::default();
        validation.set_sequence_of_references(sqref);

        match dv {
            DataValidation::Custom(c) => {
                validation
                    .set_formula1(c.clone())
                    .set_type(u::DataValidationValues::Custom)
                    .set_prompt(format!("Custom formula: {c}"));
            }
            DataValidation::DateAfter(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::GreaterThan)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date after {d}"));
            }
            DataValidation::DateBefore(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::LessThan)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date before {d}"));
            }
            DataValidation::DateBetween(d1, d2) => {
                validation
                    .set_formula1(d1.to_string())
                    .set_formula2(d2.to_string())
                    .set_operator(u::DataValidationOperatorValues::Between)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date between {d1} and {d2}"));
            }
            DataValidation::DateEqualTo(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::Equal)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date equal to {d}"));
            }
            // TODO: use a custom formula?
            DataValidation::DateIsValid => todo!(),
            DataValidation::DateNotBetween(d1, d2) => {
                validation
                    .set_formula1(d1.to_string())
                    .set_formula2(d2.to_string())
                    .set_operator(u::DataValidationOperatorValues::NotBetween)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date not between {d1} and {d2}"));
            }
            DataValidation::DateOnOrAfter(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::GreaterThanOrEqual)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date on or after {d}"));
            }
            DataValidation::DateOnOrBefore(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::LessThanOrEqual)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date on or before {d}"));
            }
            DataValidation::NumberBetween(n1, n2) => {
                validation
                    .set_formula1(n1.to_string())
                    .set_formula2(n2.to_string())
                    .set_operator(u::DataValidationOperatorValues::Between)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number between {n1} and {n2}"));
            }
            DataValidation::NumberEqualTo(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::Equal)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number equal to {n}"));
            }
            DataValidation::NumberGreaterThan(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::GreaterThan)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number greater than {n}"));
            }
            DataValidation::NumberGreaterThanOrEqualTo(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::GreaterThanOrEqual)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number greater than or equal to {n}"));
            }
            DataValidation::NumberLessThan(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::LessThan)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number less than {n}"));
            }
            DataValidation::NumberLessThanOrEqualTo(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::LessThanOrEqual)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number less than or equal to {n}"));
            }
            DataValidation::NumberNotBetween(n1, n2) => {
                validation
                    .set_formula1(n1.to_string())
                    .set_formula2(n2.to_string())
                    .set_operator(u::DataValidationOperatorValues::NotBetween)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number not between {n1} and {n2}"));
            }
            DataValidation::NumberNotEqualTo(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::NotEqual)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number equal to {n}"));
            }
            // TODO
            DataValidation::TextContains(_) => todo!(),
            // TODO
            DataValidation::TextDoesNotContain(_) => todo!(),
            // TODO
            DataValidation::TextEqualTo(_) => todo!(),
            // TODO
            DataValidation::TextIsValidEmail => todo!(),
            // TODO
            DataValidation::TextIsValidUrl => todo!(),
            DataValidation::ValueInList(values) => {
                let list_as_string = values
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                validation
                    .set_formula1(&list_as_string)
                    .set_operator(u::DataValidationOperatorValues::Equal)
                    .set_type(u::DataValidationValues::List)
                    .set_prompt(format!("Number equal to {list_as_string}"));
            }
            DataValidation::ValueInRange(a1) => {
                validation
                    .set_formula1(a1.to_string())
                    .set_operator(u::DataValidationOperatorValues::Equal)
                    .set_type(u::DataValidationValues::List)
                    .set_prompt(format!("Number equal to {a1}"));
            }
        }
        validation
    }
}

impl<'a> Excel<'a> {
    pub(crate) fn new(runtime: &'a Runtime, path: path::PathBuf) -> Self {
        Self { path, runtime }
    }

    pub(crate) fn supports_extension(os_str: &ffi::OsStr) -> bool {
        os_str.eq_ignore_ascii_case("xlsx")
            || os_str.eq_ignore_ascii_case("xlsm")
            || os_str.eq_ignore_ascii_case("xltx")
            || os_str.eq_ignore_ascii_case("xltm")
    }

    /// Since the excel library allows us to modify the speadsheet in place, the strategy here is
    /// to be a light-touch as possible and just loop through our values and set them (or not
    /// depending on the merge strategy).
    fn build_worksheet(&self, template: &Template, worksheet: &mut u::Worksheet) -> Result<()> {
        let s = template.spreadsheet.borrow();
        let mut cell_validations = vec![];

        for row in &s.rows {
            for cell in &row.cells {
                let merged_cell = merge_cell(
                    &self.get_existing_cell(cell.position, worksheet),
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

                        if let Some(n) = &cell.modifier.note {
                            self.set_comment(worksheet, &cell, n);
                        }

                        if let Some(data_validation) = cell.modifier.data_validation {
                            cell_validations.push(CellValidation(cell.position, data_validation));
                        }
                    }
                }
            }
        }

        self.set_data_validations(worksheet, cell_validations);

        Ok(())
    }

    fn set_data_validations(
        &self,
        worksheet: &mut u::Worksheet,
        cell_validations: Vec<CellValidation>,
    ) {
        let mut validations = u::DataValidations::default();
        validations
            .set_data_validation_list(cell_validations.into_iter().map(|dv| dv.into()).collect());
        worksheet.set_data_validations(validations);
    }

    fn set_comment(&self, worksheet: &mut u::Worksheet, cell: &Cell, note: &str) {
        let mut comment = u::Comment::default();
        let rt = comment.get_text_mut();
        rt.set_text(note);

        let coord = comment.get_coordinate_mut();
        coord.set_col_num(cell.position.column.x as u32);
        coord.set_row_num(cell.position.row.y as u32);

        worksheet.add_comments(comment);
    }

    // TODO: turn into an impl (from/into)? the problem is we're mutating existing_cell...
    fn set_value(&self, existing_cell: &mut u::Cell, cell: &Cell) {
        if let Some(ast) = &cell.ast {
            match *ast.clone() {
                Node::Boolean(b) => existing_cell.set_value_bool(b),
                Node::Text(t) => existing_cell.set_value_string(t),
                Node::Float(f) => existing_cell.set_value_number(f),
                Node::Integer(i) => existing_cell.set_value_number(i as f64),
                _ => existing_cell.set_formula(ast.to_string()),
            };
        } else if !cell.value.is_empty() {
            existing_cell.set_value_string(cell.value.clone());
        }
    }

    fn build_style(&self, cell: &Cell) -> Option<u::Style> {
        let modifier = cell.modifier.clone();
        if modifier.is_empty() {
            return None;
        }

        Some(excel_modifier::ExcelModifier(modifier).into())
    }

    fn get_existing_cell(
        &self,
        position: Address,
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
            u::reader::xlsx::read(self.path.as_path()).map_err(|e| Error::TargetWriteError {
                message: format!("Unable to open target file: {e}"),
                output: self.runtime.output.clone(),
            })
        } else {
            Ok(u::new_file_empty_worksheet())
        }
    }

    fn create_worksheet(&self, spreadsheet: &mut u::Spreadsheet) -> Result<()> {
        let sheet_name = self.runtime.options.sheet_name.clone();

        let existing = spreadsheet.get_sheet_by_name(&sheet_name);
        if existing.is_err() {
            spreadsheet
                .new_sheet(&sheet_name)
                .map_err(|e| Error::TargetWriteError {
                    message: format!(
                        "Unable to create new worksheet {sheet_name} in target file: {e}"
                    ),
                    output: self.runtime.output.clone(),
                })?;
        }

        Ok(())
    }

    fn get_worksheet_mut(
        &'a self,
        spreadsheet: &'a mut u::Spreadsheet,
    ) -> Result<&'a mut u::Worksheet> {
        let sheet_name = &self.runtime.options.sheet_name;
        spreadsheet
            .get_sheet_by_name_mut(sheet_name)
            .map_err(|e| Error::TargetWriteError {
                message: format!("Unable to open worksheet {sheet_name} in target file: {e}"),
                output: self.runtime.output.clone(),
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
