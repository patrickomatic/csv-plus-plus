use rand::Rng;
use std::fs;
use std::path;
use csvpp::{CliArgs, Runtime};

pub struct Setup {
    pub input_path: path::PathBuf,
    pub output_path: path::PathBuf,
    pub runtime: Runtime,
}

impl Setup {
    pub fn new(extension: &str, input: &str) -> Self {
        let mut rng = rand::thread_rng();

        let input_filename = format!("integration_test_input{}.{}", rng.gen::<u64>(), extension);
        let input_path = path::Path::new(&input_filename);
        fs::write(input_path, input).unwrap();

        let output_filename = format!("integration_test_output{}.{}", rng.gen::<u64>(), extension);
        let output_path = path::Path::new(&output_filename);

        let runtime = Runtime::try_from(&CliArgs {
            input_filename: input_path.to_path_buf(),
            output_filename: Some(output_path.to_path_buf()),
            ..Default::default()
        }).unwrap();

        Setup {
            input_path: input_path.to_path_buf(),
            output_path: output_path.to_path_buf(),
            runtime,
        }
    }

    // this is used by tests but the linter doesn't seem to know that
    #[allow(dead_code)]
    pub fn read_output(&self) -> String {
        fs::read_to_string(&self.output_path).unwrap()
    }
}

// we purposefully don't care about the Result here since we're just doing our best effort to clean
// up.  if the first remove_file fails there's no reason for it to block the second one
#[allow(unused_must_use)]
impl Drop for Setup {
    fn drop(&mut self) {
        fs::remove_file(&self.input_path);
        fs::remove_file(&self.output_path);
    }
}

