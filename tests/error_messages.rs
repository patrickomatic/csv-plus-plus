use csvpp::Template;
use std::path;
mod common;

#[test]
fn syntax_error_in_code_section() {
    let mut s = common::Setup::new(
        "csv",
        r#"
## Welcome to the all_features.csvpp test. this is a comment
##
fn foo_fn<a, b, c> a + b * c
---
foo,bar
"#,
    );
    s.runtime.source_code.filename =
        path::Path::new("integration_test_syntax_error_in_code_section.csvpp").to_path_buf();
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Syntax error in code section of integration_test_syntax_error_in_code_section.csvpp
On line 3:9, Expected `(` but saw `<`

 1: 
 2: ## Welcome to the all_features.csvpp test. this is a comment
 3: ##
 4: fn foo_fn<a, b, c> a + b * c
  : ---------^
 5: ---
 6: foo,bar

"
    );
}

#[test]
fn syntax_error_in_modifier_definition() {
    let mut s = common::Setup::new(
        "csv",
        r#"
---
foo,bar,[[format=bold ,foo
"#,
    );
    s.runtime.source_code.filename =
        path::Path::new("integration_test_syntax_error_in_modifier_definition.csvpp").to_path_buf();
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Invalid modifier definition in cell C1 (2, 0) of integration_test_syntax_error_in_modifier_definition.csvpp
On line 2:21, Error parsing input, expected ']]' but saw unrecognized token ``

 1: 
 2: ---
 3: foo,bar,[[format=bold ,foo
  : ---------------------^

"
    );
}

#[test]
fn bad_choice_in_modifier_with_possibilities() {
    let mut s = common::Setup::new(
        "csv",
        r#"
---
foo,bar,[[b=foo]],foo
"#,
    );
    s.runtime.source_code.filename =
        path::Path::new("integration_test_bad_choice_in_modifier_with_possibilities.csvpp")
            .to_path_buf();
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Invalid modifier definition in cell C1 (2, 0) of integration_test_bad_choice_in_modifier_with_possibilities.csvpp
On line 2:15, expected a valid value when parsing `border` modifier but saw `foo`
Possible values: all (a) | top (t) | bottom (b) | left (l) | right (r)

 1: 
 2: ---
 3: foo,bar,[[b=foo]],foo
  : ---------------^

"
    );
}
