mod common;

#[test]
fn cell_variable_in_fill() {
    let s = common::Setup::from_str(
        "cell_variable_in_expand",
        "csv",
        r#"
bar := test + 1
---
Foo,Bar,Baz,=SUM(test)
![[f=3]][[var=test]],=test*5,=bar,
"#,
    );
    let module = s.compiler.compile().unwrap();
    let target = s.compiler.target().unwrap();
    target.write(&module).unwrap();

    assert_eq!(
        s.read_output(),
        "Foo,Bar,Baz,=SUM(A2:A4)
,=(A2 * 5),=(A2 + 1),
,=(A3 * 5),=(A3 + 1),
,=(A4 * 5),=(A4 + 1),
"
    );
}

#[test]
fn row_variable_in_fill() {
    let s = common::Setup::from_str(
        "row_variable_in_expand",
        "csv",
        r#"
---
,=SUM(cell),=SUM(row)
![[f=3 / var=row]][[var=cell]],=cell,=row,
"#,
    );
    let module = s.compiler.compile().unwrap();
    let target = s.compiler.target().unwrap();
    target.write(&module).unwrap();

    assert_eq!(
        s.read_output(),
        ",=SUM(A2:A4),=SUM(2:4),
,=A2,=2:2,
,=A3,=3:3,
,=A4,=4:4,
"
    );
}
