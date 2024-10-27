mod common;
use common::*;

#[test]
fn all_operators() {
    assert_str_compiles_eq(
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
    assert_str_compiles_eq(
        "infix_operators_precedence",
        r#"
foo := 1 - 2 + 3 / 4 * 5 ^ 6 & 7 = 8 < 9 <= 10 > 11 >= 12
---
=foo"#,
        "=(((1 - (2 + (3 / (4 * (5 ^ 6))))) & 7) = (8 < (9 <= (10 > (11 >= 12)))))\n",
    );
}

#[test]
fn escape_newlines() {
    assert_str_compiles_eq(
        "escape_newlines",
        r#"---
[[t=b/ \
    t=u/ \
    halign=left]]foo, bar, baz"#,
        "foo,bar,baz\n",
    );
}
