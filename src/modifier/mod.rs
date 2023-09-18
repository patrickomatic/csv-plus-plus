//! # Modifier
//!
//! All of the things that can be done to a cell.  For the most part this comprises visual things
//! like fonts, formatting, alignment, etc but there are a couple tricky things here like `var`
//! which allows the user to bind variables to cells.
//!
mod border_side;
mod border_style;
mod horizontal_align;
mod number_format;
mod text_format;
mod vertical_align;

use crate::{Expand, Rgb};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
pub use border_side::BorderSide;
pub use border_style::BorderStyle;
pub use horizontal_align::HorizontalAlign;
pub use number_format::NumberFormat;
pub use text_format::TextFormat;
pub use vertical_align::VerticalAlign;

/// All of the traits that can be set on a cell modifier
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Modifier {
    pub border_color: Option<Rgb>,
    pub border_style: Option<BorderStyle>,
    pub borders: HashSet<BorderSide>,
    pub color: Option<Rgb>,
    pub font_color: Option<Rgb>,
    pub font_family: Option<String>,
    pub font_size: Option<u8>,
    pub formats: HashSet<TextFormat>,
    pub horizontal_align: Option<HorizontalAlign>,
    pub lock: bool,
    pub note: Option<String>,
    pub number_format: Option<NumberFormat>,
    pub var: Option<String>,
    pub vertical_align: Option<VerticalAlign>,
}

/// Verrrry similar to `Modifier`, except `RowModifier` can also have an `expand`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RowModifier {
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
    pub lock: bool,
    pub note: Option<String>,
    pub number_format: Option<NumberFormat>,
    pub var: Option<String>,
    pub vertical_align: Option<VerticalAlign>,
}

#[allow(clippy::from_over_into)]
impl Into<Modifier> for RowModifier {
    fn into(self) -> Modifier {
        Modifier {
            border_color: self.border_color,
            border_style: self.border_style,
            borders: self.borders.clone(),
            color: self.color,
            font_color: self.font_color,
            font_family: self.font_family,
            font_size: self.font_size,
            formats: self.formats.clone(),
            horizontal_align: self.horizontal_align,
            lock: self.lock,
            note: self.note,
            number_format: self.number_format,
            var: self.var,
            vertical_align: self.vertical_align,
        }
    }
}

impl Modifier {
    /// With the exception of `row_level` - has anything been set on this Modifier?
    pub fn is_empty(&self) -> bool {
        // I wish this wasn't so error prone
        self.border_color.is_none()
            && self.border_style.is_none() 
            && self.borders.is_empty()
            && self.color.is_none() 
            && self.font_color.is_none() 
            && self.font_family.is_none() 
            && self.font_size.is_none()
            && self.formats.is_empty()
            && self.horizontal_align.is_none()
            && !self.lock
            && self.note.is_none()
            && self.number_format.is_none()
            && self.var.is_none()
            && self.vertical_align.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_true() {
        assert!(Modifier::default().is_empty());
    }

    #[test]
    fn is_empty_false() {
        let modifier = Modifier {
            note: Some("this is a note".to_string()),
            font_size: Some(12),
            vertical_align: Some(VerticalAlign::Top),

            ..Default::default()
        };

        assert!(!modifier.is_empty());
    }
}
