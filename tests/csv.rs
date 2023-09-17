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

#[test]
fn odd_row_widths() {
    let s = common::Setup::new("csv", r#"
var1 := 42

var2 := var1 + 5

fn funky_fun(a, b) a + b

---
[[var=a1]]A1,foo,bar
![[expand=10]],bar,=var2
foo
[[lock]]test,
![[lock]]test1,test2,test3,
"#);

    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&template).unwrap();
    
    assert_eq!(
        s.read_output(),
        "A1,foo,bar,
,bar,=(42 + 5),
,bar,=(42 + 5),
,bar,=(42 + 5),
,bar,=(42 + 5),
,bar,=(42 + 5),
,bar,=(42 + 5),
,bar,=(42 + 5),
,bar,=(42 + 5),
,bar,=(42 + 5),
,bar,=(42 + 5),
foo,,,
test,,,
test1,test2,test3,
");
}
