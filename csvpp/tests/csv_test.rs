//! CSV is a particularly nice for testing because we can exihibit language features without yucky
//! things like a binary format (excel) or an API (google sheets)
use csvpp::Template;
mod common;

#[test]
fn test_write_no_code_section() {
    let s = common::Setup::new(r#"
---
foo,bar,baz
"#);
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&template).unwrap();
    
    assert_eq!(s.read_output(), "foo,bar,baz\n");
}

#[test]
fn test_write_variable() {
    let s = common::Setup::new(r#"
foo := 1
---
foo,bar,baz,=foo
"#);
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&template).unwrap();
    
    assert_eq!(
        s.read_output(),
        r#"foo,bar,baz,1
"#);
}
