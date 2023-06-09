use serde::{Serialize, Deserialize};

mod code;
mod code_section;
mod csv_section;
mod modifier;
pub mod template;
pub mod token_library;

use crate::{Error, Modifier, Node, Position, Options, Runtime};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cell {
    ast: Option<Node>,
    index: Position,
    modifier: Modifier,
    value: String,
}

fn resolve_cell_variables<'a>(runtime: Runtime) -> Result<Runtime, Error> {
    // TODO
    Ok(runtime)
}

pub fn compile_template(options: Options) -> Result<Runtime, Error> {
    let runtime = Runtime::new(options)?;

    // TODO do these in parallel
    let _csv_section = csv_section::parse(&runtime)?;
    let _code_section = code_section::parse(&runtime)?;

    resolve_cell_variables(runtime)
}
