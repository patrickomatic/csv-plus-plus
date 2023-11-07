use csvpp::Template;
mod common;

#[test]
fn syntax_error_in_code_section() {
    let s = common::Setup::from_str(
        "syntax_error_in_code_section",
        "csv",
        r#"
## Welcome to the all_features.csvpp test. this is a comment
##
fn foo_fn<a, b, c> a + b * c
---
foo,bar
"#,
    );
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Syntax error in code section of integration_test_syntax_error_in_code_section.csvpp
On line 4 Expected `(` but saw `<`

 1: \u{1b}[2m\u{1b}[0m
 2: \u{1b}[2m## Welcome to the all_features.csvpp test. this is a comment\u{1b}[0m
 3: \u{1b}[2m##\u{1b}[0m
 4: \u{1b}[91mfn foo_fn<a, b, c> a + b * c\u{1b}[0m
  : \u{1b}[33m---------^\u{1b}[0m
 5: \u{1b}[2m---\u{1b}[0m
 6: \u{1b}[2mfoo,bar\u{1b}[0m

"
    );
}

#[test]
fn syntax_error_in_modifier_definition() {
    let s = common::Setup::from_str(
        "syntax_error_in_modifier_definition",
        "csv",
        r#"
---
foo,bar,[[format=bold ,foo
"#,
    );
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Invalid modifier definition in cell C1 (2, 0) of integration_test_syntax_error_in_modifier_definition.csvpp
On line 3 Error parsing input, expected ']]' but saw unrecognized token ``

 1: \u{1b}[2m\u{1b}[0m
 2: \u{1b}[2m---\u{1b}[0m
 3: \u{1b}[91mfoo,bar,[[format=bold ,foo\u{1b}[0m
  : \u{1b}[33m---------------------^\u{1b}[0m

"
    );
}

#[test]
fn bad_choice_in_modifier_with_possibilities() {
    let s = common::Setup::from_str(
        "bad_choice_in_modifier_with_possibilities",
        "csv",
        r#"
---
foo,bar,[[b=foo]],foo
"#,
    );
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Invalid modifier definition in cell C1 (2, 0) of integration_test_bad_choice_in_modifier_with_possibilities.csvpp
On line 3 received invalid value when parsing `border` modifier but saw `foo`
Possible values: all (a) | top (t) | bottom (b) | left (l) | right (r)

 1: \u{1b}[2m\u{1b}[0m
 2: \u{1b}[2m---\u{1b}[0m
 3: \u{1b}[91mfoo,bar,[[b=foo]],foo\u{1b}[0m
  : \u{1b}[33m---------------^\u{1b}[0m

"
    );
}
