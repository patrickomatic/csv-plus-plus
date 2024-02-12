mod common;
use common::*;
use std::process::Command;

#[test]
fn module_loading_and_variable_shadowing() {
    assert_fixture_compiles_eq(
        "module_loader/main",
        "defined_in_file1,=22,should be 22
defined_in_file2,=defined_in_file2,should not resolve
function_in_file1(1 * 2),=(1 * 44),should be 1 * 44
another_defined_in_file1,=555,should be shadowed to be 555
",
    );
}

#[test]
fn module_loading_invalidate_cache() {
    // we keep around each result so it doesn't get cleaned up and file removed
    let _s1 = compile_str("module_loading_invalidate_cache", "foo := 1\n---\n=foo");
    dbg!("1");
    dbg!(Command::new("ls").args(["-l"]).output().unwrap());
    let _s2 = compile_str("module_loading_invalidate_cache", "foo := 2\n---\n=foo");
    dbg!("2");
    dbg!(Command::new("ls").args(["-l"]).output().unwrap());
    let s3 = compile_str("module_loading_invalidate_cache", "foo := 3\n---\n=foo");
    dbg!("3");
    dbg!(Command::new("ls").args(["-l"]).output().unwrap());

    assert_eq!(s3.unwrap().read_output(), "=3\n");
}
