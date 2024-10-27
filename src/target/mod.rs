use crate::{Cell, Module, Options, Result};
use std::cmp;

mod csv;
mod excel;
mod file_backer_upper;
mod google_sheets;
mod open_document;

pub(crate) use crate::target::csv::Csv;
pub(crate) use excel::Excel;
pub(crate) use google_sheets::GoogleSheets;
pub(crate) use open_document::OpenDocument;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ExistingCell<V: Clone> {
    Value(V),
    Empty,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ExistingValues<V: Clone> {
    cells: Vec<Vec<ExistingCell<V>>>,
}

#[allow(clippy::module_name_repetitions)]
pub trait CompilationTarget {
    /// Create a backup of the spreadsheet, at the given target (for most this is making a copy of
    /// a file, but different for Google Sheets
    ///
    /// # Errors
    ///
    /// * OS-level I/O errors
    fn write_backup(&self) -> Result<()>;

    /// Write the compiled `Module` to the target.
    ///
    /// # Errors
    ///
    /// * OS-level I/O errors
    fn write(&self, module: &Module) -> Result<()>;
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MergeResult<V: Clone> {
    Existing(V),
    New(Cell),
    Empty,
}

fn merge_rows<V: Clone>(
    existing_row: &[ExistingCell<V>],
    module_row: &[Cell],
    options: &Options,
) -> Vec<MergeResult<V>> {
    (0..cmp::max(existing_row.len(), module_row.len()))
        .map(|i| {
            merge_cell(
                existing_row.get(i).unwrap_or(&ExistingCell::Empty),
                module_row.get(i),
                options,
            )
        })
        .collect()
}

fn merge_cell<V: Clone>(
    existing: &ExistingCell<V>,
    new: Option<&Cell>,
    options: &Options,
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
            }
            ExistingCell::Empty => MergeResult::New(new_val.clone()),
        }
    } else {
        // no new value - return existing or empty
        match existing {
            ExistingCell::Value(v) => MergeResult::Existing(v.clone()),
            ExistingCell::Empty => MergeResult::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

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
        let new = vec![Cell::new(build_field("new value", (0, 0)))];
        let merged_row = merge_rows(existing.as_slice(), &new, &options);

        assert_eq!(3, merged_row.len());
    }

    #[test]
    fn merge_rows_overwrite_false() {
        let options = build_options(false);

        assert_eq!(
            MergeResult::Empty,
            merge_cell(&ExistingCell::<usize>::Empty, None, &options)
        );

        assert_eq!(
            MergeResult::Existing(1),
            merge_cell(&ExistingCell::Value(1), None, &options)
        );

        let cell = Cell::new(build_field("new value", (0, 0)));
        assert_eq!(
            MergeResult::Existing(1),
            merge_cell(&ExistingCell::Value(1), Some(&cell), &options)
        );
    }

    #[test]
    fn merge_cell_overwrite_true() {
        let options = build_options(true);

        assert_eq!(
            MergeResult::Empty,
            merge_cell(&ExistingCell::<usize>::Empty, None, &options)
        );

        assert_eq!(
            MergeResult::Existing(1),
            merge_cell(&ExistingCell::Value(1), None, &options)
        );

        let cell = Cell::new(build_field("new value", (0, 0)));
        assert_eq!(
            MergeResult::New(cell.clone()),
            merge_cell(&ExistingCell::Value(1), Some(&cell), &options)
        );
    }
}
