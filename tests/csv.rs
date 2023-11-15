//! CSV is a particularly nice for testing because we can exihibit language features without yucky
//! things like a binary format (excel) or an API (google sheets)
use csvpp::Module;
mod common;

#[test]
fn write_no_code_section() {
    let s = common::Setup::from_str(
        "write_no_code_section",
        "csv",
        r#"
---
foo,bar,baz
"#,
    );
    let module = Module::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&module).unwrap();

    assert_eq!(s.read_output(), "foo,bar,baz\n");
}

#[test]
fn write_variable() {
    let s = common::Setup::from_str(
        "write_variable",
        "csv",
        r#"
foo := 1
---
foo,bar,baz,=foo
"#,
    );
    let module = Module::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&module).unwrap();

    assert_eq!(
        s.read_output(),
        r"foo,bar,baz,=1
"
    );
}

#[test]
fn write_expand() {
    let s = common::Setup::from_str(
        "write_expand",
        "csv",
        r#"
commission_charge := 0.65 # the broker charges $0.65 a contract/share

fees := commission_charge * D
profit := (B * C) - fees

---
![[text=bold/halign=center]]Date   ,[[t=b]] Purchase ,Price  ,Quantity ,Profit     ,Fees
![[fill=2]]                        ,[[text=bold]]    ,       ,         ,"=profit"  ,"=fees"
"#,
    );

    let module = Module::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&module).unwrap();

    assert_eq!(
        s.read_output(),
        "Date,Purchase,Price,Quantity,Profit,Fees
,,,,=((B * C) - (0.65 * D)),=(0.65 * D)
,,,,=((B * C) - (0.65 * D)),=(0.65 * D)
"
    );
}

#[test]
fn odd_row_widths() {
    let s = common::Setup::from_str(
        "odd_row_widths",
        "csv",
        r#"
var1 := 42

var2 := var1 + 5

fn funky_fun(a, b) a + b

---
[[var=a1]]A1,foo,bar
![[fill=10]],bar,=var2
foo
[[lock]]test,
![[lock]]test1,test2,test3,
"#,
    );

    let module = Module::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&module).unwrap();

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
"
    );
}
