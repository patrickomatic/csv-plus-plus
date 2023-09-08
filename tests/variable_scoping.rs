use csvpp::Template;
mod common;

/// When a variable is defined in an expand, it should be relative to the 
#[test]
fn variable_in_expand() {
    let s = common::Setup::new("csv", r#"
bar := test + 1
---
Foo,Bar,Baz
![[e=3]][[var=test]],=test*5,=bar
"#);
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&template).unwrap();
    
    assert_eq!(s.read_output(), 
"Foo,Bar,Baz
,=(A2 * 5),=(A2 + 1)
,=(A3 * 5),=(A3 + 1)
,=(A4 * 5),=(A4 + 1)
");
}
