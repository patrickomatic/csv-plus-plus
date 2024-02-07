mod common;
use common::*;

#[test]
fn cell_variable_in_fill() {
    assert_str_compiles_eq(
        "cell_variable_in_expand",
        r#"
bar := test + 1
---
Foo,Bar,Baz,=SUM(test)
![[f=3]][[var=test]],=test*5,=bar,
"#,
        "Foo,Bar,Baz,=SUM(A2:A4)
,=(A2 * 5),=(A2 + 1),
,=(A3 * 5),=(A3 + 1),
,=(A4 * 5),=(A4 + 1),
",
    );
}

#[test]
fn row_variable_in_fill() {
    assert_str_compiles_eq(
        "row_variable_in_expand",
        r#"
---
,=SUM(cell),=SUM(row)
![[f=3 / var=row]][[var=cell]],=cell,=row,
"#,
        ",=SUM(A2:A4),=SUM(2:4),
,=A2,=2:2,
,=A3,=3:3,
,=A4,=4:4,
",
    );
}
