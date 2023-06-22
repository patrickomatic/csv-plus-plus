// TODO:
// * use the row modifier as intended
// * globally allocate the default modifier
use csv;

use crate::{Cell, Node, Position, Result, Runtime, Spreadsheet};
use super::ast_parser::AstParser;
use super::modifier;

fn parse_cell_ast(input: &str, runtime: &Runtime) -> Result<Option<Box<dyn Node>>> {
    if input.starts_with("=") {
        // TODO maybe a more robust skipping-the-first-char logic
        Ok(Some(AstParser::parse(&input[1..], false, &runtime.token_library)?))
    } else {
        Ok(None)
    }
}

fn parse_cell(
    input: String,
    index: Position,
    runtime: &Runtime,
) -> Result<Cell> {
    let parsed_modifiers = modifier::parse(index, input, runtime.default_modifier.clone())?;

    // XXX use the row_modifier
    // default_modifier = row_modifier;

    Ok(Cell {
        ast: parse_cell_ast(&parsed_modifiers.value, runtime)?,
        index: parsed_modifiers.index,
        modifier: parsed_modifiers.modifier,
        value: parsed_modifiers.value,
    })
}

pub fn parse(runtime: &Runtime) -> Result<Spreadsheet> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(runtime.source_code.csv_section.as_bytes());

    let mut cell_index = 0;
    let mut row_index = 0;

    let mut parsed_csv: Spreadsheet = vec![];

    for result in csv_reader.records() {
        let csv_row = result.unwrap_or(csv::StringRecord::new());

        let mut row: Vec<Cell> = vec![];

        for unparsed_value in &csv_row {
            let index = Position(cell_index, row_index);
            row.push(parse_cell(unparsed_value.to_string(), index, runtime)?);

            cell_index += 1;
        }

        row_index += 1;
        parsed_csv.push(row);
    }

    Ok(parsed_csv)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{CliArgs, Runtime, SourceCode};
    use crate::modifier::TextFormat;

    fn build_runtime(csv_section: &str) -> Runtime {
        let mut cli_args = CliArgs::default();
        cli_args.input_filename = PathBuf::from("foo.csvpp");
        cli_args.google_sheet_id = Some("abc123".to_string());

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
        let spreadsheet = parse(&runtime).unwrap();

        // 2 rows
        assert_eq!(spreadsheet.len(), 2);

        // each row has 3 cells
        assert_eq!(spreadsheet[0].len(), 3);
        assert_eq!(spreadsheet[1].len(), 3);

        // each row has a parsed value
        assert_eq!(spreadsheet[0][0].value, "foo");
        assert_eq!(spreadsheet[0][1].value, "bar");
        assert_eq!(spreadsheet[0][2].value, "baz");
        assert_eq!(spreadsheet[1][0].value, "1");
        assert_eq!(spreadsheet[1][1].value, "2");
        assert_eq!(spreadsheet[1][2].value, "3");
        
        // none have ASTs (didn't start with `=`)
        assert!(spreadsheet[0][0].ast.is_none());
        assert!(spreadsheet[0][1].ast.is_none());
        assert!(spreadsheet[0][2].ast.is_none());
        assert!(spreadsheet[1][0].ast.is_none());
        assert!(spreadsheet[1][1].ast.is_none());
        assert!(spreadsheet[1][2].ast.is_none());
    }

    #[test]
    fn parse_with_asts() {
        let runtime = build_runtime("=1,=2 * 3,=foo\n");
        let spreadsheet = parse(&runtime).unwrap();

        assert!(spreadsheet[0][0].ast.is_some());
        assert!(spreadsheet[0][1].ast.is_some());
        assert!(spreadsheet[0][2].ast.is_some());
    }

    #[test]
    fn parse_with_modifiers() {
        let runtime = build_runtime("[[f=b / fs=20]]foo");
        let spreadsheet = parse(&runtime).unwrap();

        assert!(spreadsheet[0][0].modifier.formats.contains(&TextFormat::Bold));
        assert_eq!(spreadsheet[0][0].modifier.font_size, Some(20))
    }

    #[test]
    fn parse_with_row_modifier() {
        // TODO
    }
}
