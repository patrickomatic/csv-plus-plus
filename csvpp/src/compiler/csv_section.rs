use std::error::Error;
use csv;

use crate::modifier::Modifier;
use crate::compiler::{Cell, Spreadsheet};
use crate::options::Options;
use crate::Position;
use crate::error::CsvppError;

fn parse_cell<'a>(input: String, index: Position) -> Result<Cell, CsvppError<'a>> {
    // XXX make a more global/static version of this so we're not allocating on every cell parse
    let default_modifier = Modifier::new(true);

    let parsed_modifiers = super::modifier::parse(index, input, default_modifier)?;

    // XXX use the row_modifier
    // default_modifier = row_modifier;

    Ok(Cell {
        ast: None, // XXX parse the AST
        index: parsed_modifiers.index,
        modifier: parsed_modifiers.modifier,
        value: parsed_modifiers.value,
    })
}

pub fn parse<'a>(options: &'a Options) -> Result<Spreadsheet, Box<dyn Error>> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(options.input.csv_section.as_bytes());

    let mut cell_index = 0;
    let mut row_index = 0;

    let mut parsed_csv: Spreadsheet = vec![];

    for result in csv_reader.records() {
        let csv_row = result.unwrap_or(csv::StringRecord::new());

        let mut row: Vec<Cell> = vec![];

        for unparsed_value in &csv_row {
            let index = Position(cell_index, row_index);
            row.push(parse_cell(unparsed_value.to_string(), index)?);

            cell_index += 1;
        }

        row_index += 1;
        parsed_csv.push(row);
    }

    Ok(parsed_csv)
}

