// use serde::{Serialize, Deserialize};
mod ast_lexer;
mod ast_parser;
mod code_section;
mod csv_section;
mod modifier;
pub mod template;
pub mod token_library;

use crate::{Modifier, Node, Position};

// TODO rename to SpreadsheetCell? Rust has too many Cell-like names... RefCell, OnceCell, etc
// #[derive(Clone, Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub struct Cell {
    ast: Option<Box<dyn Node>>,
    index: Position,
    modifier: Modifier,
    value: String,
}
