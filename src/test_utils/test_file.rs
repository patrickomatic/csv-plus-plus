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
pub(crate) struct TestFile(pub(crate) path::PathBuf);

impl TestFile {
    pub(crate) fn new(ext: &str, input: &str) -> Self {
        let mut rng = rand::thread_rng();
        let filename = &format!("unit_test_file_{}.{ext}", rng.gen::<u64>());
        let path = path::Path::new(&filename);
        fs::write(path, input).unwrap();

        Self(path.to_path_buf())
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        fs::remove_file(&self.0).unwrap();
    }
}
