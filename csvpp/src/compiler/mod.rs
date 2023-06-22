// use serde::{Serialize, Deserialize};
mod ast_lexer;
pub mod ast_parser;
mod code_section_parser;
mod csv_section;
mod modifier;
pub mod template;
pub mod token_library;

use crate::{Modifier, Node, Position};

// TODO rename to SpreadsheetCell? Rust has too many Cell-like names... RefCell, OnceCell, etc
// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub struct Cell {
    pub ast: Option<Box<dyn Node>>,
    pub index: Position,
    pub modifier: Modifier,
    pub value: String,
}
