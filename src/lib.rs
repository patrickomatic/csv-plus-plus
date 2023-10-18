mod ast;
mod cell;
mod cli_args;
mod date_time;
mod error;
mod expand;
mod modifier;
mod options;
mod output;
mod parser;
mod rgb;
mod row;
mod runtime;
mod source_code;
mod spreadsheet;
mod target;
mod template;

pub use cell::Cell;
pub use cli_args::CliArgs;
pub use date_time::DateTime;
pub use error::{Error, ParseError, Result};
pub use expand::Expand;
pub use modifier::{Modifier, RowModifier};
pub use options::Options;
pub use output::Output;
pub use rgb::Rgb;
pub use row::Row;
pub use runtime::Runtime;
pub use source_code::{CharOffset, LineNumber, SourceCode};
pub use spreadsheet::Spreadsheet;
pub use target::CompilationTarget;
pub use template::Template;

pub(crate) use error::ParseResult;

// test_utils should only be included in tests, never referenced by release code (or built into the
// release)
#[cfg(test)]
mod test_utils;

pub(crate) fn csv_reader() -> csv::ReaderBuilder {
    let mut csv_reader = csv::ReaderBuilder::new();
    csv_reader.flexible(true).has_headers(false);
    csv_reader
}
