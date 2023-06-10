// TODO
//
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::str::FromStr;

use crate::{Error, Rgb};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BorderSide {
    All,
    Top,
    Bottom,
    Left,
    Right,
}

impl FromStr for BorderSide {
    fn from_str(input: &str) -> Result<Self, Error> {
        match input {
            "a" | "all"       => Ok(Self::All),
            "t" | "top"       => Ok(Self::Top),
            "b" | "bottom"    => Ok(Self::Bottom),
            "l" | "left"      => Ok(Self::Left),
            "r" | "right"     => Ok(Self::Right),
            _ => Err(Error::InvalidModifier {
                message: "Invalid border= value",
                bad_input: input,
                possible_values: "all (a) | top (t) | bottom (b) | left (l) | right (r)",
            }),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BorderStyle {
    Dashed,
    Dotted,
    Double,
    Solid,
    SolidMedium,
    SolidThick,
}

impl FromStr for BorderStyle {
    fn from_str(input: &str) -> Result<Self, Error> {
        match input {
            "dash" | "dashed"        => Ok(Self::Dashed),
            "dot"  | "dotted"        => Ok(Self::Dotted),
            "dbl"  | "double"        => Ok(Self::Double),
            "1"    | "solid"         => Ok(Self::Solid),
            "2"    | "solid_medium"  => Ok(Self::SolidMedium),
            "3"    | "solid_thick"   => Ok(Self::SolidThick),
            _ => Err(Error::InvalidModifier {
                message: "Invalid borderstyle= value",
                bad_input: input,
                possible_values: "dashed (dash) | dotted (dot) | double (dbl) \
                                    | solid (1) | solid_medium (2) | solid_thick (3)",
            }),
        }
    }
}

/// The possible values for aligning a cell horizontally.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum HorizontalAlign {
    Center,
    Left,
    Right,
}

impl FromStr for HorizontalAlign {
    fn from_str(input: &str) -> Result<Self, Error> {
        match input {
            "c" | "center"  => Ok(Self::Center),
            "l" | "left"    => Ok(Self::Left),
            "r" | "right"   => Ok(Self::Right),
            _ => Err(Error::InvalidModifier { 
                message: "Invalid halign= value",
                bad_input: input, 
                possible_values: "center (c) | left (l) | right (r)",
            }),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

    fn from_str(input: &str) -> Result<Self, Error> {
        match input {
            "c"  | "currency"      => Ok(Self::Currency),
            "d"  | "date"          => Ok(Self::Date),
            "dt" | "datetime"      => Ok(Self::DateTime),
            "n"  | "number"        => Ok(Self::Number),
            "p"  | "percent"       => Ok(Self::Percent),
            "text"                 => Ok(Self::Text),  // TODO: think of a shortcut!!!
            "t"  | "time"          => Ok(Self::Time),
            "s"  | "scientific"    => Ok(Self::Scientific),
            _ => Err(Error::InvalidModifier { 
                message: "Invalid numberformat= value",
                bad_input: input,
                possible_values: "currency (c) | date (d) | datetime (dt) | number (n) | percent (p) \
                                    | text | time (t) | scientific (s)",
            }),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub enum TextFormat {
    Bold,
    Italic,
    Strikethrough,
    Underline,
}

impl FromStr for TextFormat {
    fn from_str(input: &str) -> Result<Self, Error> {
        match input {
            "b" | "bold"          => Ok(Self::Bold),
            "i" | "italic"        => Ok(Self::Italic),
            "s" | "strikethrough" => Ok(Self::Strikethrough),
            "u" | "underline"     => Ok(Self::Underline),
            _ => Err(Error::InvalidModifier { 
                message: "Invalid format= value",
                bad_input: input, 
                possible_values: "bold (b) | italic (i) | strikethrough (s) | underline (u)",
            }),
        }
    }
}

/// The possible values for aligning a cell vertically.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VerticalAlign {
    Bottom,
    Center,
    Top,
}

impl FromStr for VerticalAlign {
    fn from_str(input: &str) -> Result<Self, Error> {
        match input {
            "b" | "bottom"    => Ok(Self::Bottom),
            "c" | "center"    => Ok(Self::Center),
            "t" | "top"       => Ok(Self::Top),
            _ => Err(Error::InvalidModifier { 
                message: "Invalid valign= value",
                bad_input: input, 
                possible_values: "bottom (b) | center (c) | top (t)",
            }),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Expand {
    pub amount: Option<usize>,
}

/// # Modifier
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
