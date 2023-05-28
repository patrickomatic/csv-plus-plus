use rgb::RGB16;
use std::collections::HashSet;

#[derive(Debug)]
enum BorderSide {
    All,
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug)]
enum BorderStyle {
    Dashed,
    Dotted,
    Double,
    Solid,
    SolidMedium,
    SolidThick,
}

/// The possible values for aligning a cell horizontally.
#[derive(Debug)]
enum HorizontalAlign {
    Center,
    Left,
    Right,
}

#[derive(Debug)]
enum NumberFormat {
    Currency,
    Date,
    DateTime,
    Number,
    Percent,
    Text,
    Time,
    Scientific,
}

#[derive(Debug)]
enum TextFormat {
    Bold,
    Italic,
    Strikethrough,
    Underline,
}

/// The possible values for aligning a cell vertically.
#[derive(Debug)]
enum VerticalAlign {
    Bottom,
    Center,
    Top,
}

#[derive(Debug)]
struct Expand {
    amount: Option<u16>,
}

/// # Modifier
#[derive(Debug)]
pub struct Modifier {
    border_color: Option<RGB16>,
    border_style: Option<BorderStyle>,
    borders: HashSet<BorderSide>,
    color: Option<RGB16>,
    expand: Option<Expand>,
    font_color: Option<RGB16>,
    font_family: Option<String>,
    font_size: Option<u8>,
    formats: HashSet<TextFormat>,
    horizontal_align: Option<HorizontalAlign>,
    note: Option<String>,
    number_format: Option<NumberFormat>,
    row_level: bool,
    var: Option<String>,
    vertical_align: Option<VerticalAlign>,
}

impl Modifier {
    pub fn new() -> Modifier {
        Modifier {
            border_color: None,
            border_style: None,
            borders: HashSet::new(),
            color: None,
            expand: None,
            font_color: None,
            font_family: None,
            font_size: None,
            formats: HashSet::new(),
            horizontal_align: None,
            note: None,
            number_format: None,
            row_level: false,
            var: None,
            vertical_align: None,
        }
    }
}
