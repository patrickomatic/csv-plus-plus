use csvpp::Template;
mod common;

#[test]
fn test_syntax_error() {
    let s = common::Setup::new("csv", r#"
## Welcome to the all_features.csvpp test. this is a comment
##
fn foo_fn<a, b, c> a + b * c
---
foo,bar
"#);
    let template = Template::compile(&s.runtime);

    // TODO: I think this is failing because the line number is wrong from the code section parser,
    // not in the highlighting logic
    assert_eq!(
        template.unwrap_err().to_string(),
        "Syntax error on line 3: Expected `(` but saw <
 1: 
 2: ## Welcome to the all_features.csvpp test. this is a comment
 3: ##
 4: fn foo_fn<a, b, c> a + b * c
  : ---------^
 5: ---
 6: foo,bar");
}
