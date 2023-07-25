use csvpp::Template;
mod common;

#[test]
fn test_syntax_error() {
    let s = common::Setup::new(r#"
## Welcome to the all_features.csvpp test. this is a comment
##
fn foo_fn<a, b, c> a + b * c
---
foo,bar
"#);
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "3:10: Expected `(` but saw <");
}

