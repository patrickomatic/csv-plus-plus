#![warn(clippy::pedantic)]
#![deny(warnings)]

mod config;
mod error;
mod field;
mod parser;
mod source_position;

pub use config::Config;
pub use error::Error;
pub use field::Field;
pub use parser::parse;
pub use source_position::SourcePosition;

pub type Record = Vec<Field>;
pub type Records = Vec<Record>;
pub type Offset = usize;
pub type Result<T> = std::result::Result<T, Error>;
