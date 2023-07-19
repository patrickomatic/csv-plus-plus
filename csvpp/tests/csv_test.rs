//! CSV is a particularly nice for testing because we can exihibit language features without yucky
//! things like a binary format (excel) or an API (google sheets)
use csvpp::{CliArgs, Template, Runtime};
use rand::Rng;
use std::fs;
use std::path;

struct Setup {
    input_path: path::PathBuf,
    output_path: path::PathBuf,
}

impl Setup {
    fn new(input: &str) -> Setup {
        let mut rng = rand::thread_rng();

        let input_filename = format!("integration_test_input{}.csv", rng.gen::<u64>());
        let input_path = path::Path::new(&input_filename);
        fs::write(input_path, input).unwrap();

        let output_filename = format!("integration_test_output{}.csv", rng.gen::<u64>());
        let output_path = path::Path::new(&output_filename);

        Setup {
            input_path: input_path.to_path_buf(),
            output_path: output_path.to_path_buf(),
        }
    }

    fn read_output(&self) -> String {
        fs::read_to_string(&self.output_path).unwrap()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        fs::remove_file(&self.input_path).unwrap();
        fs::remove_file(&self.output_path).unwrap();
    }
}

#[test]
fn test_write_no_code_section() {
    let s = Setup::new(r#"
---
foo,bar,baz
"#);
    let runtime = Runtime::new(CliArgs {
        input_filename: s.input_path.clone(),
        output_filename: Some(s.output_path.clone()),
        ..Default::default()
    }).unwrap();

    let template = Template::compile(&runtime).unwrap();
    let target = runtime.target().unwrap();
    target.write(&template).unwrap();
    
    assert_eq!(s.read_output(), "foo,bar,baz\n");
}

#[test]
fn test_write_variable() {
    let s = Setup::new(r#"
foo := 1
---
foo,bar,baz,=foo
"#);
    let runtime = Runtime::new(CliArgs {
        input_filename: s.input_path.clone(),
        output_filename: Some(s.output_path.clone()),
        ..Default::default()
    }).unwrap();

    let template = Template::compile(&runtime).unwrap();
    let target = runtime.target().unwrap();
    target.write(&template).unwrap();
    
    assert_eq!(
        s.read_output(),
        r#"foo,bar,baz,1
"#);
}
