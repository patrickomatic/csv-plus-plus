use crate::{CliArgs, Runtime, SourceCode};
use rand::Rng;
use std::fs;
use std::path;

/// Only to be used as a helper in tests, this makes it easy to create a text file that will get
/// cleaned up by the borrow checker.  It provides two important properties:
///
/// * A random filename -  we need this because tests run in parallel and will step on each
/// other's runs otherwise.
///
/// * A `Drop` trait impl - this makes sure the files get cleaned up after the tests run.  The
/// same functionality as an "after each" step in other test frameworks.
///
#[derive(Clone, Debug)]
pub(crate) struct TestSourceCode {
    pub(crate) input_file: path::PathBuf,
    pub(crate) output_file: path::PathBuf,
}

/// We frequently need to be able to produce a `Runtime` given a source file.
/// NOTE: It's important that this takes a reference to a `TestSourceCode` rather than an owned
/// struct, because if it took the latter it would immediately trigger the `Drop` impl which would
/// remove the files.  So we need to keep a reference open to it rather than owning it
impl From<&TestSourceCode> for Runtime {
    fn from(test_file: &TestSourceCode) -> Self {
        Self::try_from(&CliArgs {
            input_filename: test_file.input_file.clone(),
            output_filename: Some(test_file.output_file.clone()),
            ..Default::default()
        })
        .unwrap()
    }
}

/// NOTE: It's important that this takes a reference to a `TestSourceCode` rather than an owned
/// struct, because if it took the latter it would immediately trigger the `Drop` impl which would
/// remove the files.  So we need to keep a reference open to it rather than owning it
impl From<&TestSourceCode> for SourceCode {
    fn from(test_file: &TestSourceCode) -> Self {
        Self::new(test_file.read_input(), test_file.input_file.clone()).unwrap()
    }
}

impl TestSourceCode {
    pub(crate) fn new(output_extension: &str, input: &str) -> Self {
        let mut rng = rand::thread_rng();

        let input_filename = format!("unit_test_input_{}.csvpp", rng.gen::<u64>());
        let source_path = path::Path::new(&input_filename);
        fs::write(source_path, input).unwrap();

        let output_filename = format!("unit_test_output_{}.{output_extension}", rng.gen::<u64>());
        let output_path = path::Path::new(&output_filename);

        Self {
            input_file: source_path.to_path_buf(),
            output_file: output_path.to_path_buf(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn read_output(&self) -> String {
        fs::read_to_string(&self.output_file).unwrap()
    }

    fn object_code_filename(&self) -> path::PathBuf {
        let mut f = self.input_file.clone();
        f.set_extension("csvpo");
        f
    }

    fn read_input(&self) -> String {
        fs::read_to_string(&self.input_file).unwrap()
    }
}

// we purposefully don't care about the Result here since we're just doing our best effort to clean
// up.  if the first remove_file fails there's no reason for it to block the second one
#[allow(unused_must_use)]
impl Drop for TestSourceCode {
    fn drop(&mut self) {
        fs::remove_file(self.object_code_filename());
        fs::remove_file(&self.input_file);
        fs::remove_file(&self.output_file);
    }
}
