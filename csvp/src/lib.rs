#![warn(clippy::pedantic)]
#![deny(warnings)]

mod cell;
mod error;
mod parser;
mod source_position;

pub use cell::Cell;
pub use error::Error;
pub use parser::{parse, Config};
pub use source_position::SourcePosition;

pub type ParsedCells = Vec<Vec<Cell>>;
pub type Offset = usize;
pub type Result<T> = std::result::Result<T, Error>;
