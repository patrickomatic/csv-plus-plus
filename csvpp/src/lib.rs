// TODO:
// * use clippy
// * move some of this into lib.rs rather than main.rs
// * use (read from) the object file if it exists
mod ast;
mod compiler;
mod error;
mod modifier;
mod options;
mod position;
mod rgb;
mod runtime;
mod source_code;

pub use ast::Node;
pub use compiler::{Cell, compile_template};
pub use compiler::template::{Spreadsheet, Template};
pub use compiler::token_library::TokenLibrary;
pub use error::*;
pub use modifier::Modifier;
pub use options::{parse_cli_args, Options};
pub use position::Position;
pub use rgb::Rgb;
pub use runtime::Runtime;
pub use source_code::SourceCode;
