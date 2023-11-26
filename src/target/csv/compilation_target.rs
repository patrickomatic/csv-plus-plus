use crate::target::{file_backer_upper, merge_rows, CompilationTarget, Csv, MergeResult};
use crate::{Module, Result};

impl CompilationTarget for Csv<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(self.runtime, &self.path)?;
        Ok(())
    }

    fn write(&self, module: &Module) -> Result<()> {
        let existing_values = Self::read(&self.path, &self.runtime.output)?;

        let new_values = module.spreadsheet.borrow();
        let widest_row = new_values.widest_row();

        let mut writer = csv::WriterBuilder::new()
            .flexible(true)
            .from_path(&self.path)
            .map_err(|e| {
                self.runtime
                    .output_error(format!("Unable to open output file for writing: {e:?}"))
            })?;

        for (index, row) in new_values.rows.iter().enumerate() {
            let mut output_row: Vec<String> = merge_rows(
                existing_values
                    .cells
                    .get(index)
                    .unwrap_or(&vec![].to_owned()),
                &row.cells,
                &self.runtime.options,
            )
            .iter()
            .map(|cell| match cell {
                MergeResult::New(v) => v.to_string(),
                MergeResult::Existing(v) => v.to_string(),
                MergeResult::Empty => "".to_owned(),
            })
            .collect();

            // all rows have to be as wide as the widest row
            output_row.resize(widest_row, "".to_string());

            writer.write_record(output_row).map_err(|e| {
                self.runtime
                    .output_error(format!("Unable to write row {index}: {e}"))
            })?;
        }

        writer.flush().map_err(|e| {
            self.runtime
                .output_error(format!("Unable to finish writing to output: {e}"))
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn write() {
        let test_file = &TestSourceCode::new(
            "csv",
            "foo,bar,baz
one,,two,,three
",
        );
        let output_file = test_file.output_file.clone();
        let runtime: Runtime = test_file.into();
        let module = runtime.compile().unwrap();
        let csv = Csv::new(&runtime, output_file);

        assert!(csv.write(&module).is_ok());
    }
}
