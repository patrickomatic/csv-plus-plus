mod common;

#[test]
fn module_loading_and_variable_shadowing() {
    let s = common::Setup::from_fixture("module_loader/main", "csv");
    let module = s.compiler.compile().unwrap();
    let target = s.compiler.target().unwrap();

    target.write(&module).unwrap();

    assert_eq!(
        s.read_output(),
        "defined_in_file1,=22,should be 22
defined_in_file2,=defined_in_file2,should not resolve
function_in_file1(1 * 2),=(1 * 44),should be 1 * 44
another_defined_in_file1,=555,should be shadowed to be 555
"
    );
}

#[test]
fn module_loading_repeatedly_from_cache() {
    // TODO
}
