mod common;
use common::*;
use std::process::Command;
use std::{thread, time};

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
#[ignore]
fn module_loading_invalidate_cache() {
    let ten_millis = time::Duration::from_millis(10);

    // we keep around each result so it doesn't get cleaned up and file removed
    dbg!("Writing the first file");
    let _s1 = compile_str("module_loading_invalidate_cache", "foo := 1\n---\n=foo");
    thread::sleep(ten_millis);
    dbg!(Command::new("ls").args(["-l"]).output().unwrap());

    dbg!("Writing the second file");
    let _s2 = compile_str("module_loading_invalidate_cache", "foo := 2\n---\n=foo");
    thread::sleep(ten_millis);
    dbg!(Command::new("ls").args(["-l"]).output().unwrap());

    dbg!("Writing the third file");
    let s3 = compile_str("module_loading_invalidate_cache", "foo := 3\n---\n=foo");
    thread::sleep(ten_millis);
    dbg!(Command::new("ls").args(["-l"]).output().unwrap());

    assert_eq!(s3.unwrap().read_output(), "=3\n");
}
