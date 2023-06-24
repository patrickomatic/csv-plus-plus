//!
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

mod border_side;
mod border_style;
mod expand;
mod horizontal_align;
mod number_format;
mod text_format;
mod vertical_align;

pub use border_side::BorderSide;
pub use border_style::BorderStyle;
pub use expand::Expand;
pub use horizontal_align::HorizontalAlign;
pub use number_format::NumberFormat;
pub use text_format::TextFormat;
pub use vertical_align::VerticalAlign;

use crate::Rgb;

/// # Modifier
///
/// All of the things that can be done to a cell.  For the most part this comprises visual things
/// like fonts, formatting, alignment, etc but there are a couple tricky things here like `var`
/// which allows the user to bind variables to cells.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

impl Modifier {
    pub fn new(row_level: bool) -> Self {
        Self {
            row_level,
            ..Self::default()
        }
    }

    /// Copy the values from `modifier` and allocate a new one.  This is a common procedure
    /// because as we're parsing, we take the row modifier and use it to initialize each of the
    /// cell modifiers.  This is also why the `row_level` always gets set to `false` - this should
    /// always be `false` by default.  It's the only field we don't wanna carry over.
    pub fn from(modifier: &Modifier) -> Self {
        Self {
            row_level: false,
            ..modifier.clone()
        }
    }
}
