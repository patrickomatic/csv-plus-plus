mod common;

#[test]
fn all_operators() {
    let compiled = common::Setup::str_to_csv(
        "all_operators",
        r#"
# infix operators
plus := 1 + 2
minus := 1 - 2
multiply := 1 * 2
divide := 1 / 2
power := 1 ^ 2
concat := 1 & 2
equal := 1 = 2
less_than := 1 < 2
less_than_equal := 1 <= 2
greater_than := 1 > 2
greater_than_equal := 1 >= 2

# postfix
percentage := 1%

# prefix
positive := +1
negative := -1

---
=plus,
=minus,
=multiply,
=divide,
=power,
=concat,
=equal,
=less_than,
=less_than_equal,
=greater_than,
=greater_than_equal,
=percentage,
=positive,
=negative,
"#,
    );

    assert_eq!(
        compiled,
        "=(1 + 2),
=(1 - 2),
=(1 * 2),
=(1 / 2),
=(1 ^ 2),
=(1 & 2),
=(1 = 2),
=(1 < 2),
=(1 <= 2),
=(1 > 2),
=(1 >= 2),
=1%,
=+1,
=-1,
",
    );
}

#[test]
fn infix_operators_precedence() {
    let compiled = common::Setup::str_to_csv(
        "infix_operators_precedence",
        r#"
foo := 1 - 2 + 3 / 4 * 5 ^ 6 & 7 = 8 < 9 <= 10 > 11 >= 12
---
=foo"#,
    );

    assert_eq!(
        compiled,
        "=(((1 - (2 + (3 / (4 * (5 ^ 6))))) & 7) = (8 < (9 <= (10 > (11 >= 12)))))\n",
    );
}
