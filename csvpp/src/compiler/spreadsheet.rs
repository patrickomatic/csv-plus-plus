// TODO:
// * use the row modifier as intended
// * globally allocate the default modifier
use std::collections::HashMap;
use std::fmt;
use csv;

use crate::{A1, Modifier, Node, Reference, Result, Runtime, Variables};
use super::ast_parser::AstParser;
use super::modifier_parser::ModifierParser;

// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub struct SpreadsheetCell {
    pub ast: Option<Box<dyn Node>>,
    pub index: A1,
    pub modifier: Modifier,
    pub value: String,
}

impl SpreadsheetCell {
    pub fn parse(
        input: &str,
        index: A1,
        runtime: &Runtime,
    ) -> Result<SpreadsheetCell> {
        let parsed_modifiers = ModifierParser::parse(index, input, runtime.default_modifier.clone())?;

        // XXX use the row_modifier
        // default_modifier = row_modifier;

        Ok(SpreadsheetCell {
            ast: Self::parse_ast(&parsed_modifiers.value, runtime)?,
            index: parsed_modifiers.index,
            modifier: parsed_modifiers.modifier,
            value: parsed_modifiers.value,
        })
    }

    fn parse_ast(input: &str, runtime: &Runtime) -> Result<Option<Box<dyn Node>>> {
        if let Some(without_equals) = input.strip_prefix('=') {
            Ok(Some(AstParser::parse(without_equals, false, &runtime.token_library)?))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Default)]
pub struct Spreadsheet {
    pub cells: Vec<Vec<SpreadsheetCell>>,
}

impl Spreadsheet {
    /// Parse the spreadsheet section of a csv++ source file.
    pub fn parse(runtime: &Runtime) -> Result<Spreadsheet> {
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(runtime.source_code.csv_section.as_bytes());
        let mut cell_index = 0;
        let mut cells: Vec<Vec<SpreadsheetCell>> = vec![];

        for (row_index, result) in csv_reader.records().enumerate() {
            let mut row: Vec<SpreadsheetCell> = vec![];

            for unparsed_value in &result.unwrap_or(csv::StringRecord::new()) {
                let a1 = A1::builder().xy(cell_index, row_index).build()?;
                row.push(SpreadsheetCell::parse(unparsed_value, a1, runtime)?);

                cell_index += 1;
            }

            cells.push(row);
        }

        Ok(Spreadsheet { cells })
    }

    /// Extract all of the variables that were defined by cells contained in this spreadsheet
    // 
    // NOTE: we could also store these in a HashMap on the Spreadsheet as we build it
    pub fn variables(&self) -> Variables {
        let mut vars = HashMap::new();
        self.cells.iter().flatten().for_each(|c| {
            if let Some(var_id) = &c.modifier.var {
                let reference: Box<dyn Node> = Box::new(Reference(c.index.to_string()));
                vars.insert(var_id.to_owned(), reference);
            }
        });

        vars
    }
}

impl fmt::Display for Spreadsheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: something better
        write!(f, "{}", self.cells.len())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{CliArgs, Runtime, SourceCode};
    use crate::modifier::TextFormat;

    fn build_runtime(csv_section: &str) -> Runtime {
        let cli_args = CliArgs {
            input_filename: PathBuf::from("foo.csvpp"),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };

        let mut runtime = Runtime::new(cli_args).unwrap();
        runtime.source_code = SourceCode {
            filename: PathBuf::from("foo.csvpp"),
            lines: 3,
            length_of_code_section: 0,
            length_of_csv_section: 0,
            code_section: None,
            csv_section: csv_section.to_string(),
        };

        runtime
    }

    #[test]
    fn parse_simple() {
        let runtime = build_runtime("foo,bar,baz\n1,2,3\n");
        let spreadsheet = Spreadsheet::parse(&runtime).unwrap();

        // 2 rows
        assert_eq!(spreadsheet.cells.len(), 2);

        // each row has 3 cells
        assert_eq!(spreadsheet.cells[0].len(), 3);
        assert_eq!(spreadsheet.cells[1].len(), 3);

        // each row has a parsed value
        assert_eq!(spreadsheet.cells[0][0].value, "foo");
        assert_eq!(spreadsheet.cells[0][1].value, "bar");
        assert_eq!(spreadsheet.cells[0][2].value, "baz");
        assert_eq!(spreadsheet.cells[1][0].value, "1");
        assert_eq!(spreadsheet.cells[1][1].value, "2");
        assert_eq!(spreadsheet.cells[1][2].value, "3");
        
        // none have ASTs (didn't start with `=`)
        assert!(spreadsheet.cells[0][0].ast.is_none());
        assert!(spreadsheet.cells[0][1].ast.is_none());
        assert!(spreadsheet.cells[0][2].ast.is_none());
        assert!(spreadsheet.cells[1][0].ast.is_none());
        assert!(spreadsheet.cells[1][1].ast.is_none());
        assert!(spreadsheet.cells[1][2].ast.is_none());
    }

    #[test]
    fn parse_with_asts() {
        let runtime = build_runtime("=1,=2 * 3,=foo\n");
        let spreadsheet = Spreadsheet::parse(&runtime).unwrap();

        assert!(spreadsheet.cells[0][0].ast.is_some());
        assert!(spreadsheet.cells[0][1].ast.is_some());
        assert!(spreadsheet.cells[0][2].ast.is_some());
    }

    #[test]
    fn parse_with_modifiers() {
        let runtime = build_runtime("[[f=b / fs=20]]foo");
        let spreadsheet = Spreadsheet::parse(&runtime).unwrap();

        assert!(spreadsheet.cells[0][0].modifier.formats.contains(&TextFormat::Bold));
        assert_eq!(spreadsheet.cells[0][0].modifier.font_size, Some(20))
    }

    #[test]
    fn parse_with_row_modifier() {
        // TODO
    }
}
