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

fn get_cell<'a>(
    spreadsheet: &'a umya_spreadsheet::Spreadsheet,
    cell: &str,
) -> &'a umya_spreadsheet::Cell {
    spreadsheet
        .get_sheet_by_name("all_features")
        .unwrap()
        .get_cell(cell)
        .unwrap()
}

#[test]
fn all_features_excel() {
    let setup = assert_fixture_compiles_ok("all_features", "xlsx");

    let s = umya_spreadsheet::reader::xlsx::read(&setup.output_path).unwrap();
    // borders
    dbg!(get_cell(&s, "B2").get_style());
    assert_eq!(
        get_cell(&s, "B2")
            .get_style()
            .get_borders()
            .unwrap()
            .get_top()
            .get_border_style(),
        umya_spreadsheet::structs::Border::BORDER_THIN
    );

    // color
    assert_eq!(
        get_cell(&s, "B13")
            .get_style()
            .get_fill()
            .unwrap()
            .get_pattern_fill()
            .unwrap()
            .get_foreground_color()
            .unwrap()
            .get_argb(),
        "FF0000FF"
    );
    assert_eq!(
        get_cell(&s, "B22")
            .get_style()
            .get_font()
            .unwrap()
            .get_color()
            .get_argb(),
        "FF0000FF"
    );

    // text formats
    assert!(get_cell(&s, "B52")
        .get_style()
        .get_font()
        .unwrap()
        .get_bold());
    assert!(get_cell(&s, "B53")
        .get_style()
        .get_font()
        .unwrap()
        .get_italic());
    assert_eq!(
        get_cell(&s, "B54")
            .get_style()
            .get_font()
            .unwrap()
            .get_underline(),
        "single"
    );
    assert!(get_cell(&s, "B55")
        .get_style()
        .get_font()
        .unwrap()
        .get_strikethrough());

    // fonts
    assert_eq!(
        get_cell(&s, "B30")
            .get_style()
            .get_font()
            .unwrap()
            .get_size(),
        &20.0
    );

    // alignments
    assert_eq!(
        get_cell(&s, "B34")
            .get_style()
            .get_alignment()
            .unwrap()
            .get_horizontal(),
        &umya_spreadsheet::HorizontalAlignmentValues::Left
    );
    assert_eq!(
        get_cell(&s, "B84")
            .get_style()
            .get_alignment()
            .unwrap()
            .get_vertical(),
        &umya_spreadsheet::VerticalAlignmentValues::Top
    );
}

// TODO:
// #[test]
// fn all_features_google_sheets() {
// }
