//! # csv++
//!
mod ast;
mod cell;
mod cli_args;
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
pub use error::{Error, InnerError, InnerResult, Result};
pub use expand::Expand;
pub use modifier::{Modifier, RowModifier};
pub use options::Options;
pub use output::Output;
pub use rgb::Rgb;
pub use row::Row;
pub use runtime::Runtime;
pub use source_code::SourceCode;
pub use spreadsheet::Spreadsheet;
pub use target::CompilationTarget;
pub use template::Template;

/// Some shared test utility functions.  
#[cfg(test)]
pub(crate) mod test_utils {
    use crate::{CliArgs, Runtime, SourceCode};
    use rand::Rng;
    use std::fs;
    use std::path;

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
            Self::try_from(&CliArgs {
                input_filename: test_file.input_file.clone(),
                output_filename: Some(test_file.output_file.clone()),
                ..Default::default()
            })
            .unwrap()
        }
    }

    impl From<TestFile> for SourceCode {
        fn from(test_file: TestFile) -> Self {
            Self::new(&test_file.read_input(), test_file.input_file.clone()).unwrap()
        }
    }

    impl TestFile {
        pub(crate) fn new(output_extension: &str, input: &str) -> Self {
            let mut rng = rand::thread_rng();

            let input_filename = format!("unit_test_input{}.csvpp", rng.gen::<u64>());
            let source_path = path::Path::new(&input_filename);
            fs::write(source_path, input).unwrap();

            let output_filename =
                format!("unit_test_output{}.{output_extension}", rng.gen::<u64>());
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
