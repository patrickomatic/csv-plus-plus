mod common;
use common::assert_fixture_compiles;

#[test]
fn all_features_csv() {
    assert_fixture_compiles!("all_features", "csv");
}

#[test]
fn all_features_shorthand_csv_no_code_section() {
    assert_fixture_compiles!("all_features_shorthand", "csv");
}

#[test]
fn all_features_excel() {
    assert_fixture_compiles!("all_features", "xlsx");
}

// TODO:
// #[test]
// fn all_features_google_sheets() {
// }
