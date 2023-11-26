mod common;

#[test]
fn all_features_csv() {
    let s = common::Setup::from_fixture("all_features", "csv");
    let module = s.compiler.compile().unwrap();
    let target = s.compiler.target().unwrap();

    assert!(target.write(&module).is_ok());
}

#[test]
fn all_features_shorthand_csv_no_code_section() {
    let s = common::Setup::from_fixture("all_features_shorthand", "csv");
    let module = s.compiler.compile().unwrap();
    let target = s.compiler.target().unwrap();

    assert!(target.write(&module).is_ok());
}

#[test]
fn all_features_excel() {
    let s = common::Setup::from_fixture("all_features", "xlsx");
    let module = s.compiler.compile().unwrap();
    let target = s.compiler.target().unwrap();

    assert!(target.write(&module).is_ok());
}

// TODO:
// #[test]
// fn all_features_google_sheets() {
// }
