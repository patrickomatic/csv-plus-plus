use csvpp::Template;
mod common;

#[test]
fn test_syntax_error_in_code_section() {
    let s = common::Setup::new("csv", r#"
## Welcome to the all_features.csvpp test. this is a comment
##
fn foo_fn<a, b, c> a + b * c
---
foo,bar
"#);
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Syntax error on line 4: Expected `(` but saw <
 1: 
 2: ## Welcome to the all_features.csvpp test. this is a comment
 3: ##
 4: fn foo_fn<a, b, c> a + b * c
  : ---------^
 5: ---
 6: foo,bar
");
}

#[test]
fn test_syntax_error_in_modifier_definition() {
    let s = common::Setup::new("csv", r#"
---
foo,bar,[[format=bold ,foo
"#);
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Invalid modifier definition in cell C1 on line 5
Error parsing input, expected ']]'
bad input: 
");
}

#[test]
fn test_bad_choice_in_modifier_with_possibilities() {
    let s = common::Setup::new("csv", r#"
---
foo,bar,[[b=foo]],foo
"#);
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Invalid modifier definition in cell C1 on line 5
Invalid border= value
bad input: foo
possible values: all (a) | top (t) | bottom (b) | left (l) | right (r)
");
}
