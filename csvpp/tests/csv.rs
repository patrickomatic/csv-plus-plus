//! CSV is a particularly nice for testing because we can exihibit language features without yucky
//! things like a binary format (excel) or an API (google sheets)
use csvpp::Template;
mod common;

#[test]
fn write_no_code_section() {
    let s = common::Setup::new("csv", r#"
---
foo,bar,baz
"#);
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&template).unwrap();
    
    assert_eq!(s.read_output(), "foo,bar,baz\n");
}

#[test]
fn write_variable() {
    let s = common::Setup::new("csv", r#"
foo := 1
---
foo,bar,baz,=foo
"#);
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&template).unwrap();
    
    assert_eq!(
        s.read_output(),
        r"foo,bar,baz,1
");
}

#[test]
fn write_expand() {
    let s = common::Setup::new("csv", r#"
commission_charge := 0.65 # the broker charges $0.65 a contract/share

fees := commission_charge * celladjacent(D)
profit := (celladjacent(B) * celladjacent(C)) - fees

---
![[format=bold/halign=center]]Date ,Purchase         ,Price  ,Quantity ,Profit     ,Fees
![[expand=2]]                      ,[[format=bold]]  ,       ,         ,"=profit"  ,"=fees"
"#);

    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&template).unwrap();
    
    assert_eq!(
        s.read_output(),
        "Date,Purchase         ,Price  ,Quantity ,Profit     ,Fees
,,       ,         ,=((B2 * C2) - (0.65 * D2)),=(0.65 * D2)
,,       ,         ,=((B3 * C3) - (0.65 * D3)),=(0.65 * D3)
");
}
