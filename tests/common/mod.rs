#![allow(dead_code)]

use csvpp::*;
use rand::Rng;
use std::fs;
use std::path;

#[derive(Debug)]
pub(crate) struct Setup {
    pub(crate) input_path: path::PathBuf,
    pub(crate) output_path: path::PathBuf,
    cleanup_input: bool,
    pub(crate) compiler: Compiler,
}

pub(crate) fn compile_str(test_name: &str, input: &str) -> Result<Setup> {
    Setup::from_str(test_name, "csv", input).compile()
}

pub(crate) fn assert_fixture_compiles_ok(filename: &str, extension: &str) -> Setup {
    let setup = Setup::from_fixture(filename, extension).compile();
    assert!(setup.is_ok());
    setup.unwrap()
}

pub(crate) fn assert_fixture_compiles_eq(filename: &str, expected: &str) {
    let setup = Setup::from_fixture(filename, "csv").compile().unwrap();
    assert_eq!(setup.read_output(), expected);
}

pub(crate) fn assert_str_compiles_eq(test_name: &str, input: &str, expected: &str) {
    let setup = compile_str(test_name, input).unwrap();
    assert_eq!(setup.read_output(), expected);
}

impl Setup {
    pub(crate) fn compile(self) -> Result<Self> {
        {
            let target = self.compiler.target()?;
            let module = self.compiler.compile().map_err(|e| dbg!(e))?;
            target.write(&module).unwrap();
        }
        Ok(self)
    }

    pub(crate) fn from_str(test_name: &str, extension: &str, input: &str) -> Self {
        let input_filename = format!("integration_test_{test_name}.csvpp");
        let input_path = path::Path::new(&input_filename);
        fs::write(input_path, input).unwrap();

        Self::from_file(input_path.to_path_buf(), extension, true)
    }

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
