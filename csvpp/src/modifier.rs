// TODO
//
// * open up the ModifierRightSide parser fn so that it actually can parse non-letter characters
//
use crate::rgb::Rgb;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BorderSide {
    All,
    Top,
    Bottom,
    Left,
    Right,
}

impl FromStr for BorderSide {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "a" | "all"       => Ok(Self::All),
            "t" | "top"       => Ok(Self::Top),
            "b" | "bottom"    => Ok(Self::Bottom),
            "l" | "left"      => Ok(Self::Left),
            "r" | "right"     => Ok(Self::Right),
            _ => 
                Err(format!("Invalid border= value: {}", input)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BorderStyle {
    Dashed,
    Dotted,
    Double,
    Solid,
    SolidMedium,
    SolidThick,
}

impl FromStr for BorderStyle {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dash" | "dashed"        => Ok(Self::Dashed),
            "dot"  | "dotted"        => Ok(Self::Dotted),
            "dbl"  | "double"        => Ok(Self::Double),
            "1"    | "solid"         => Ok(Self::Solid),
            "2"    | "solid_medium"  => Ok(Self::SolidMedium),
            "3"    | "solid_thick"   => Ok(Self::SolidThick),
            _ => 
                Err(format!("Invalid borderstyle= value: {}", input)),
        }
    }
}

/// The possible values for aligning a cell horizontally.
#[derive(Clone, Debug, PartialEq)]
pub enum HorizontalAlign {
    Center,
    Left,
    Right,
}

impl FromStr for HorizontalAlign {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "c" | "center"  => Ok(Self::Center),
            "l" | "left"    => Ok(Self::Left),
            "r" | "right"   => Ok(Self::Right),
            _ => 
                Err(format!("Invalid halign= value: {}", input)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberFormat {
    Currency,
    Date,
    DateTime,
    Number,
    Percent,
    Text,
    Time,
    Scientific,
}

impl FromStr for NumberFormat {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "c"  | "currency"      => Ok(Self::Currency),
            "d"  | "date"          => Ok(Self::Date),
            "dt" | "date_time"     => Ok(Self::DateTime),
            "n"  | "number"        => Ok(Self::Number),
            "p"  | "percent"       => Ok(Self::Percent),
            "o"  | "text"          => Ok(Self::Text), // TODO change "text" to "none"... or
                                                      // something.  fix this
            "t"  | "time"          => Ok(Self::Time),
            "s"  | "scientific"    => Ok(Self::Scientific),
            _ =>
                Err(format!("Invalid numberformat= value: {}", input)),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum TextFormat {
    Bold,
    Italic,
    Strikethrough,
    Underline,
}

impl FromStr for TextFormat {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "b" | "bold"          => Ok(Self::Bold),
            "i" | "italic"        => Ok(Self::Italic),
            "s" | "strikethrough" => Ok(Self::Strikethrough),
            "u" | "underline"     => Ok(Self::Underline),
            _ =>
                Err(format!("Invalid format= value: {}", input)),
        }
    }
}

/// The possible values for aligning a cell vertically.
#[derive(Clone, Debug, PartialEq)]
pub enum VerticalAlign {
    Bottom,
    Center,
    Top,
}

impl FromStr for VerticalAlign {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "b" | "bottom"    => Ok(Self::Bottom),
            "c" | "center"    => Ok(Self::Center),
            "t" | "top"       => Ok(Self::Top),
            _ 
                => Err(format!("Invalid valign= value: {}", input)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expand {
    pub amount: Option<usize>,
}

/// # Modifier
#[derive(Clone, Debug, PartialEq)]
pub struct Modifier {
    pub border_color: Option<Rgb>,
    pub border_style: Option<BorderStyle>,
    pub borders: HashSet<BorderSide>,
    pub color: Option<Rgb>,
    pub expand: Option<Expand>,
    pub font_color: Option<Rgb>,
    pub font_family: Option<String>,
    pub font_size: Option<u8>,
    pub formats: HashSet<TextFormat>,
    pub horizontal_align: Option<HorizontalAlign>,
    pub note: Option<String>,
    pub number_format: Option<NumberFormat>,
    pub row_level: bool,
    pub var: Option<String>,
    pub vertical_align: Option<VerticalAlign>,
}

impl Default for Modifier {
    fn default() -> Self {
        // TODO maybe make this lazy static?
        Self {
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

impl Modifier {
    pub fn new(row_level: bool) -> Self {
        let default = Self::default();
        Self {
            row_level,
            ..default
        }
    }

    pub fn from(modifier: &Modifier) -> Self {
        Self {
            row_level: false,
            ..modifier.clone()
        }
    }
}

