use std::fmt;
use std::result::Result;
use std::error::Error;

mod code_section;
mod csv_section;
mod modifier;

use crate::Position;
use crate::ast;
use crate::error::CsvppError;
use crate::options::Options;
 
// TODO: include Options? what is really shared? does it even matter?
pub trait Parser {
    fn parse(&self, input: String) -> Result<Box<Self>, CsvppError>;
}

#[derive(Clone, Debug)]
pub struct Cell {
    ast: Option<ast::Node>,
    index: Position,
    modifier: super::modifier::Modifier,
    value: String,
}

type Spreadsheet = Vec<Vec<Cell>>; 

pub struct Template {
    spreadsheet: Spreadsheet,
}

impl<'a> fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"
    rows: {}
            "#,
            self.spreadsheet.len()
        )
    }
}

fn resolve_cell_variables<'a>(spreadsheet: Spreadsheet) -> Spreadsheet {
    // TODO
    spreadsheet
}

pub fn compile_template<'a>(options: &'a Options) -> Result<Template, Box<dyn Error>> {
    // TODO do these in parallel
    let csv_section = csv_section::parse(options)?;
    let _code_section = code_section::parse(options)?;

    let spreadsheet = resolve_cell_variables(csv_section);

    // resolve_cell_variables(csv_section, code_section)
    Ok(Template { spreadsheet })
}
