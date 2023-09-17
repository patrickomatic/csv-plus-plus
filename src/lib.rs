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
mod expand;
mod modifier;
mod options;
mod output;
mod rgb;
mod runtime;
mod source_code;
mod target;
mod template;

pub use cli_args::CliArgs;
pub use compiler::spreadsheet::Spreadsheet;
pub use compiler::spreadsheet_cell::SpreadsheetCell;
pub use template::Template;
pub use error::{Error, InnerError, Result, InnerResult};
pub use expand::Expand;
pub use modifier::Modifier;
pub use options::Options;
pub use output::Output;
pub use rgb::Rgb;
pub use runtime::Runtime;
pub use source_code::SourceCode;
pub use target::CompilationTarget;

/// Some shared test utility functions.  
#[cfg(test)]
pub(crate) mod test_utils {
    use rand::Rng;
    use std::fs;
    use std::path;
    use crate::{CliArgs, Runtime, SourceCode};

    /// Only to be used as a helper in tests, this provides two important properties:
    ///
    /// * A random filename -  we need this because tests run in parallel and will step on each
    /// other's runs otherwise.
    ///
    /// * A `Drop` trait impl - this makes sure the files get cleaned up after the tests run.  The
    /// same functionality as an "after each" step in other test frameworks.
    ///
    #[derive(Clone, Debug)]
    pub(crate) struct TestFile {
        pub(crate) input_file: path::PathBuf,
        pub(crate) output_file: path::PathBuf,
    }

    /// We frequently need to be able to produce a Runtime given a source file.
    impl From<TestFile> for Runtime {
        fn from(test_file: TestFile) -> Self {
            Self::new(CliArgs {
                input_filename: test_file.input_file.clone(),
                output_filename: Some(test_file.output_file.clone()),
                ..Default::default()
            }).unwrap()
        }
    }

    impl From<TestFile> for SourceCode {
        fn from(test_file: TestFile) -> Self {
            Self::new(
                &test_file.read_input(),
                test_file.input_file.clone()).unwrap()
        }
    }

    impl TestFile {
        pub(crate) fn new(output_extension: &str, input: &str) -> Self {
            let mut rng = rand::thread_rng();

            let input_filename = format!("unit_test_input{}.csvpp", rng.gen::<u64>());
            let source_path = path::Path::new(&input_filename);
            fs::write(source_path, input).unwrap();

            let output_filename = format!("unit_test_output{}.{output_extension}", rng.gen::<u64>());
            let output_path = path::Path::new(&output_filename);

            Self {
                input_file: source_path.to_path_buf(),
                output_file: output_path.to_path_buf(),
            }
        }

        /*
        pub(crate) fn read_output(&self) -> String {
            fs::read_to_string(&self.output_file).unwrap()
        }
        */

        fn read_input(&self) -> String {
            fs::read_to_string(&self.input_file).unwrap()
        }
    }

    // we purposefully don't care about the Result here since we're just doing our best effort to clean
    // up.  if the first remove_file fails there's no reason for it to block the second one
    #[allow(unused_must_use)]
    impl Drop for TestFile {
        fn drop(&mut self) {
            fs::remove_file(&self.input_file);
            fs::remove_file(&self.output_file);
        }
    }
}
