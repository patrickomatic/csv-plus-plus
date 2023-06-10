// TODO:
// * use the row modifier as intended
// * globally allocate the default modifier
use csv;

use crate::compiler::code;
use crate::{Cell, Error, Position, Runtime, Spreadsheet};

fn parse_cell<'a>(
    input: String,
    index: Position,
    runtime: &Runtime,
) -> Result<Cell, Error> {
    let parsed_modifiers = super::modifier::parse(index, input, runtime.default_modifier.clone())?;

    // XXX use the row_modifier
    // default_modifier = row_modifier;

    Ok(Cell {
        // XXX use &str instead
        // XXX there may or may not be an AST
        ast: Some(code::AstParser::parse(&parsed_modifiers.value, &runtime.token_library)?), 
        index: parsed_modifiers.index,
        modifier: parsed_modifiers.modifier,
        value: parsed_modifiers.value,
    })
}

pub fn parse<'a>(runtime: &'a Runtime) -> Result<Spreadsheet, Error> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(runtime.options.input.csv_section.as_bytes());

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

