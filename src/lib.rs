mod ast;
mod border_side;
mod border_style;
mod cell;
mod cli_args;
mod code_section;
mod compiler;
mod data_validation;
mod date_time;
mod error;
mod fill;
mod horizontal_align;
mod module;
mod module_path;
mod number_format;
mod options;
mod output;
mod parser;
mod rgb;
mod row;
mod source_code;
mod spreadsheet;
mod target;
mod text_format;
mod vertical_align;

pub use border_side::BorderSide;
pub use border_style::BorderStyle;
pub use cell::Cell;
pub use cli_args::CliArgs;
pub use code_section::CodeSection;
pub use compiler::Compiler;
pub(crate) use data_validation::DataValidation;
pub use date_time::DateTime;
pub use error::{Error, ParseError, Result};
pub use fill::Fill;
pub use horizontal_align::HorizontalAlign;
pub use module::Module;
pub use module_path::ModulePath;
pub use number_format::NumberFormat;
pub use options::Options;
pub use output::Output;
pub use rgb::Rgb;
pub use row::Row;
pub(crate) use source_code::ArcSourceCode;
pub use source_code::{CharOffset, LineNumber, SourceCode};
pub use spreadsheet::Spreadsheet;
pub use target::CompilationTarget;
pub use text_format::TextFormat;
pub use vertical_align::VerticalAlign;

// test_utils should only be included in tests, never referenced by release code (or built into the
// release)
#[cfg(test)]
mod test_utils;

pub(crate) fn csv_reader() -> csv::ReaderBuilder {
    let mut csv_reader = csv::ReaderBuilder::new();
    csv_reader.flexible(true).has_headers(false);
    csv_reader
}
