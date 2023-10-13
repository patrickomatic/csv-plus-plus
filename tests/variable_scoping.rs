use csvpp::Template;
mod common;

#[test]
fn cell_variable_in_expand() {
    let s = common::Setup::new(
        "csv",
        r#"
bar := test + 1
---
Foo,Bar,Baz,=SUM(test)
![[e=3]][[var=test]],=test*5,=bar,
"#,
    );
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&template).unwrap();

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
fn row_variable_in_expand() {
    let s = common::Setup::new(
        "csv",
        r#"
---
,=SUM(cell),=SUM(row)
![[e=3 / var=row]][[var=cell]],=cell,=row,
"#,
    );
    let template = Template::compile(&s.runtime).unwrap();
    let target = s.runtime.target().unwrap();
    target.write(&template).unwrap();

    assert_eq!(
        s.read_output(),
        ",=SUM(A2:A4),=SUM(2:4),
,=A2,=2:2,
,=A3,=3:3,
,=A4,=4:4,
"
    );
}
