//! CSV is nice for testing certain language features because we can compare the output easily and
//! don't have to deal with things like APIs or binary formats.
//!
mod common;
use common::*;

#[test]
fn write_no_code_section() {
    assert_eq!(
        Setup::from_str(
            "write_no_code_section",
            "csv",
            r#"
---
foo,bar,baz
"#,
        )
        .compile()
        .unwrap()
        .read_output(),
        "foo,bar,baz\n"
    );
}

#[test]
fn write_string() {
    assert_eq!(
        Setup::from_str(
            "write_string",
            "csv",
            r#"
foo := "1"
bar := "quoted ""string"""
---
foo,bar,baz,=foo,=bar
"#,
        )
        .compile()
        .unwrap()
        .read_output(),
        r#"foo,bar,baz,"=""1""","=""quoted ""string"""""
"#
    );
}

#[test]
fn write_variable() {
    assert_eq!(
        Setup::from_str(
            "write_variable",
            "csv",
            r#"
foo := 1
---
foo,bar,baz,=foo
"#,
        )
        .compile()
        .unwrap()
        .read_output(),
        r"foo,bar,baz,=1
"
    );
}

#[test]
fn write_expand() {
    assert_eq!(
        Setup::from_str(
            "write_expand",
            "csv",
            r#"
commission_charge := 0.65 # the broker charges $0.65 a contract/share

fees := commission_charge * D
profit := (B * C) - fees

---
![[text=bold halign=center]]Date   ,[[t=b]] Purchase ,Price  ,Quantity ,Profit     ,Fees
![[fill=2]]                        ,[[text=bold]]    ,       ,         ,"=profit"  ,"=fees"
"#,
        )
        .compile()
        .unwrap()
        .read_output(),
        "Date,Purchase,Price,Quantity,Profit,Fees
,,,,=((B * C) - (0.65 * D)),=(0.65 * D)
,,,,=((B * C) - (0.65 * D)),=(0.65 * D)
"
    );
}

#[test]
fn odd_row_widths() {
    assert_eq!(
        Setup::from_str(
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
        )
        .compile()
        .unwrap()
        .read_output(),
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
