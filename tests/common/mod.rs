use csvpp::*;
use rand::Rng;
use std::fs;
use std::path;

pub(crate) struct Setup {
    pub(crate) input_path: path::PathBuf,
    pub(crate) output_path: path::PathBuf,
    cleanup_input: bool,
    #[allow(dead_code)]
    pub(crate) compiler: Compiler,
}

impl Setup {
    #[allow(dead_code)]
    pub(crate) fn str_to_csv(test_name: &str, input: &str) -> String {
        let s = Self::from_str(test_name, "csv", input);
        let target = s.compiler.target().unwrap();
        let module = s
            .compiler
            .compile()
            .map_err(|e| dbg!(e.to_string()))
            .unwrap();
        target.write(&module).unwrap();

        s.read_output()
    }

    #[allow(dead_code)]
    pub(crate) fn from_str(test_name: &str, extension: &str, input: &str) -> Self {
        let input_filename = format!("integration_test_{test_name}.csvpp");
        let input_path = path::Path::new(&input_filename);
        fs::write(input_path, input).unwrap();

        Self::from_file(input_path.to_path_buf(), extension, true)
    }

    #[allow(dead_code)]
    pub(crate) fn from_fixture(fixture_name: &str, extension: &str) -> Self {
        let input_path = path::Path::new("playground")
            .join(format!("{fixture_name}.csvpp"))
            .to_path_buf();
        Self::from_file(input_path, extension, false)
    }

    pub(crate) fn from_file(
        input_path: path::PathBuf,
        extension: &str,
        cleanup_input: bool,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let output_filename = format!("integration_test_output_{}.{extension}", rng.gen::<u64>());
        let output_path = path::Path::new(&output_filename);

        let compiler = Compiler::try_from(&CliArgs {
            input_filename: input_path.clone(),
            output_filename: Some(output_path.to_path_buf()),
            ..Default::default()
        })
        .unwrap();

        // we want to match on the output, so suppress colors (which introduce weird control chars)
        colored::control::set_override(false);

        Setup {
            input_path,
            output_path: output_path.to_path_buf(),
            cleanup_input,
            compiler,
        }
    }

    fn object_code_filename(&self) -> path::PathBuf {
        let mut f = self.input_path.clone();
        f.set_extension("csvpo");
        f
    }

    // this is used by tests but the linter doesn't seem to know that
    #[allow(dead_code)]
    pub(crate) fn read_output(&self) -> String {
        fs::read_to_string(&self.output_path).unwrap()
    }
}

// we purposefully don't care about the Result here since we're just doing our best effort to clean
// up.  if the first remove_file fails there's no reason for it to block the second one
#[allow(unused_must_use)]
impl Drop for Setup {
    fn drop(&mut self) {
        fs::remove_file(self.object_code_filename());
        if self.cleanup_input {
            fs::remove_file(&self.input_path);
        }
        fs::remove_file(&self.output_path);
    }
}
