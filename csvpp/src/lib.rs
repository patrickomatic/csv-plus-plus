//! # csv++
//!
//! At the most basic, this is a tool that can take CSV file and output it to Excel, Google Sheets
//! or OpenDocument.  However csv++ provides a superset of CSV which allows you to develop
//! spreadsheets like you would code.
//!
//! You can specify formatting in the CSV:
//!
//! ```csvpp
//! ![[format=bold/fontsize=20]]Header1     ,Header2    ,Header3
//!                             foo         ,bar        ,baz
//! ```
//!
//! or you can use short-hand notation:
//!
//! ```csvpp
//! ![[f=b/fs=20]]Header1     ,Header2    ,Header3
//!               foo         ,bar        ,baz
//! ```
//!
//! You can also define a code section at the top with functions and variables:
//!
//! ```csvpp
//! # define a variable that we can use in the code section and cells
//! foo := 42
//! fn bar(a) a + 3
//! ---
//! =foo   ,=foo + 2   ,=bar(foo)
//! ```
//!
mod ast;
mod compiler;
mod cli_args;
mod error;
mod modifier;
mod options;
mod output;
mod rgb;
mod runtime;
mod source_code;
mod target;

pub use cli_args::CliArgs;
pub use compiler::spreadsheet::{Spreadsheet, SpreadsheetCell};
pub use compiler::template::Template;
pub use error::{Error, InnerError, Result, InnerResult};
pub use modifier::Modifier;
pub use options::Options;
pub use output::Output;
pub use rgb::Rgb;
pub use runtime::Runtime;
pub use source_code::SourceCode;
pub use target::CompilationTarget;

