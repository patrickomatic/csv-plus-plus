mod common;

#[test]
fn infix_operators_precedence() {
    let compiled = common::Setup::str_to_csv(
        "infix_operators",
        r#"
foo := 1 - 2 + 3 / 4 * 5 ^ 6 & 7 = 8 < 9 <= 10 > 11 >= 12
---
=foo"#,
    );

    assert_eq!(
        compiled,
        "=((((((((1 - 2) + ((3 / 4) * (5 ^ 6))) & 7) = 8) < 9) <= 10) > 11) >= 12)\n"
    );
}
