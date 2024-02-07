mod common;
use common::assert_fixture_compiles_ok;

#[test]
fn all_features_csv() {
    assert_fixture_compiles_ok("all_features", "csv");
}

#[test]
fn all_features_shorthand_csv_no_code_section() {
    assert_fixture_compiles_ok("all_features_shorthand", "csv");
}

#[test]
fn all_features_excel() {
    assert_fixture_compiles_ok("all_features", "xlsx");
}

// TODO:
// #[test]
// fn all_features_google_sheets() {
// }
