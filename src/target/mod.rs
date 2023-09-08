use std::cmp;
use crate::{Options, Result, SpreadsheetCell, Template};

mod csv;
mod excel;
mod file_backer_upper;
mod google_sheets;
mod open_document;

pub use crate::target::csv::Csv;
pub use excel::Excel;
pub use google_sheets::GoogleSheets;
pub use open_document::OpenDocument;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ExistingCell<V: Clone> {
    Value(V),
    Empty,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ExistingValues<V: Clone> {
    cells: Vec<Vec<ExistingCell<V>>>,
}

pub trait CompilationTarget {
    /// Create a backup of the spreadsheet, at the given target (for most this is making a copy of
    /// a file, but different for Google Sheets
    fn write_backup(&self) -> Result<()>;

    /// Write the compiled `Template` to the target.
    fn write(&self, template: &Template) -> Result<()>;
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MergeResult<V: Clone> {
    Existing(V),
    New(SpreadsheetCell),
    Empty,
}

fn merge_rows<V: Clone>(
    existing_row: &[ExistingCell<V>],
    template_row: &[SpreadsheetCell],
    options: &Options,
) -> Vec<MergeResult<V>> {
    (0..cmp::max(existing_row.len(), template_row.len()))
        .map(|i| {
            merge_cell(
                existing_row.get(i).unwrap_or(&ExistingCell::Empty),
                template_row.get(i),
                options)
        })
        .collect()
}

fn merge_cell<V: Clone>(
    existing: &ExistingCell<V>,
    new: Option<&SpreadsheetCell>,
    options: &Options
) -> MergeResult<V> {
    if let Some(new_val) = new {
        match existing {
            ExistingCell::Value(v) => {
                // both new and existing values
                if options.overwrite_values {
                    MergeResult::New(new_val.clone())
                } else {
                    MergeResult::Existing(v.clone())
                }
            },
            ExistingCell::Empty => 
                MergeResult::New(new_val.clone()),

        }
    } else {
        // no new value - return existing or empty
        match existing {
            ExistingCell::Value(v) => 
                MergeResult::Existing(v.clone()),
            ExistingCell::Empty => MergeResult::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use a1_notation::Address;
    use crate::Modifier;
    use super::*;

    fn build_options(overwrite_values: bool) -> Options {
        Options {
            overwrite_values,
            ..Default::default()
        }
    }

    #[test]
    fn merge_rows_different_lengths() {
        let options = build_options(false);
        let existing = vec![
            ExistingCell::Value(1),
            ExistingCell::Value(2),
            ExistingCell::Value(3),
        ];
        let new = vec![
            SpreadsheetCell {
                ast: None,
                position: Address::new(0, 0),
                modifier: Modifier::default(),
                row_modifier: None,
                value: "new value".to_string(),
            }
        ];
        let merged_row = merge_rows(existing.as_slice(), &new, &options);

        assert_eq!(3, merged_row.len());
    }

    #[test]
    fn merge_rows_overwrite_false() {
        let options = build_options(false);

        assert_eq!(
            MergeResult::Empty,
            merge_cell(&ExistingCell::<usize>::Empty, None, &options));

        assert_eq!(
            MergeResult::Existing(1),
            merge_cell(&ExistingCell::Value(1), None, &options));

        let cell = SpreadsheetCell {
            ast: None,
            position: Address::new(0, 0),
            modifier: Modifier::default(),
            row_modifier: None,
            value: "new value".to_string(),
        };
        assert_eq!(
            MergeResult::Existing(1),
            merge_cell(&ExistingCell::Value(1), Some(&cell), &options));
    }

    #[test]
    fn merge_cell_overwrite_true() {
        let options = build_options(true);

        assert_eq!(
            MergeResult::Empty,
            merge_cell(&ExistingCell::<usize>::Empty, None, &options));

        assert_eq!(
            MergeResult::Existing(1),
            merge_cell(&ExistingCell::Value(1), None, &options));

        let cell = SpreadsheetCell {
            ast: None,
            position: Address::new(0, 0),
            modifier: Modifier::default(),
            row_modifier: None,
            value: "new value".to_string(),
        };
        assert_eq!(
            MergeResult::New(cell.clone()),
            merge_cell(&ExistingCell::Value(1), Some(&cell), &options));
    }
}
