use csv;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufReader;
use std::fmt;
use std::fs::File;
use std::result::Result;

use crate::ast;
use crate::error;
use crate::modifier;
use crate::options;

pub struct Template {
}

pub fn compile_template(options: &options::Options) -> Result<Template, Box<dyn Error>> {
    println!("compilign");
    // TODO do these in parallel
    let csv_section = parse_csv_section(options)?;
    dbg!(csv_section);
    let code_section = parse_code_section(options)?;

    // resolve_cell_variables(csv_section, code_section)
    Ok(Template {

    })
}

struct CodeSection {
    variables: HashMap<String, ast::Node>,
    // TODO can I enforce that this is only Node::Functions?
    functions: HashMap<String, ast::Node>,
}

fn parse_code_section(ontions: &options::Options) -> Result<CodeSection, Box<dyn Error>> {
    // XXX do the parsing
    Ok(CodeSection {
        variables: HashMap::new(),
        functions: HashMap::new(),
    })
}

#[derive(Debug)]
pub struct Cell {
    ast: Option<ast::Node>,
    index: u16,
    modifier: modifier::Modifier,
    row_index: u16,
    value: String,
}

type CSVSection = Vec<Vec<Cell>>; 

fn parse_csv_section(options: &options::Options) -> Result<CSVSection, Box<dyn Error>> {
    let file = File::open(&options.input.filename)?;
    let reader = BufReader::new(file);

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(options.input.csv_section.as_bytes());

    let mut index = 0;
    let mut row_index = 0;

    let mut parsed_csv: Vec<Vec<Cell>> = vec![];

    for result in csv_reader.records() {
        let csv_row = result.unwrap_or(csv::StringRecord::new());

        let mut row: Vec<Cell> = vec![];
        for value in &csv_row {
            row.push(Cell {
                // XXX parse the AST
                ast: None,
                index,
                modifier: modifier::Modifier::new(),
                row_index,
                value: value.to_string(),
            });
            index += 1;
        }

        row_index += 1;
        parsed_csv.push(row);
    }

    Ok(parsed_csv)
}
