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
    let module = s.compiler.compile();

    assert_eq!(
        module.unwrap_err().to_string(),
        "Syntax error in code section of integration_test_syntax_error_in_code_section.csvpp
On line 4 Expected `(` for a function definition but saw `<`

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
fn syntax_error_in_option_definition() {
    let s = common::Setup::from_str(
        "syntax_error_in_option_definition",
        "csv",
        r#"
---
foo,bar,[[text=bold ,foo
"#,
    );
    let module = s.compiler.compile();

    assert_eq!(
        module.unwrap_err().to_string(),
        "Syntax error in cell C1 of integration_test_syntax_error_in_option_definition.csvpp
On line 3 Expected a OptionName but saw unrecognized token ``

 1: 
 2: ---
 3: foo,bar,[[text=bold ,foo
  : -------------------^

"
    );
}

#[test]
fn bad_choice_in_option_with_possibilities() {
    let s = common::Setup::from_str(
        "bad_choice_in_option_with_possibilities",
        "csv",
        r#"
---
foo,bar,[[b=foo]],foo
"#,
    );
    let module = s.compiler.compile();

    assert_eq!(
        module.unwrap_err().to_string(),
        "Syntax error in cell C1 of integration_test_bad_choice_in_option_with_possibilities.csvpp
On line 3 received invalid value when parsing `border` option but saw `foo`
Possible values: all (a) | top (t) | bottom (b) | left (l) | right (r)

 1: 
 2: ---
 3: foo,bar,[[b=foo]],foo
  : --------------^

"
    );
}

#[test]
fn syntax_error_in_csv_section() {
    let s = common::Setup::from_str(
        "syntax_error_in_csv_section",
        "csv",
        r#"
# it's a common problem that the `function_in_file1(1, 2)` call needs to be quoted 
# because it has a comma
---
function_in_file1(1 * 2)  ,   =function_in_file1(1, 2)    , should be 1 * 44
"#,
    );
    let module = s.compiler.compile();

    assert_eq!(module.unwrap_err().to_string(), "Syntax error in cell B1 of integration_test_syntax_error_in_csv_section.csvpp
On line 5 Expected an expression but saw EOF
If your formula has a comma in it, you might need to escape it with quotes (i.e. `foo,\"=my_function(1, 2)\",bar`)

 2: # it's a common problem that the `function_in_file1(1, 2)` call needs to be quoted 
 3: # because it has a comma
 4: ---
 5: function_in_file1(1 * 2)  ,   =function_in_file1(1, 2)    , should be 1 * 44
  : --------------------------------------------------^

");
}

#[test]
fn module_loader_module_does_not_exist() {
    let s = common::Setup::from_str(
        "module_loader_module_does_not_exist",
        "csv",
        r#"
use foobar

a := 1 * 2
---
"#,
    );
    let module = s.compiler.compile();

    assert_eq!(
        module.unwrap_err().to_string(),
        "Error loading module foobar
Error reading source foobar.csvpp
Error reading source code foobar.csvpp: No such file or directory (os error 2)

"
    );
}
