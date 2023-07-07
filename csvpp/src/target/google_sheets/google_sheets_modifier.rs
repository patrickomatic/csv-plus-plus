//! # GoogleSheetsModifier
//!
//! A wrapper around `Modifier` to make it more compatible with the Google Sheets API.  An
//! important design principal here is that we only send the changes that the user set - we should
//! not be sending a bunch of default values and prefer to return None instead.  In other words the
//! API payloads should reflect only the things the user specified in the modifier.
//!
use google_sheets4::api;
use crate::{Modifier, Rgb};
use crate::modifier;

pub struct GoogleSheetsModifier<'a>(pub &'a Modifier);

impl<'a> GoogleSheetsModifier<'a> {
    pub fn cell_format(&self) -> Option<api::CellFormat> {
        // XXX return None if nothing is set
        Some(api::CellFormat {
            background_color_style: self.color_style(&self.0.color),
            borders: self.borders(),
            horizontal_alignment: self.horizontal_alignment(),
            number_format: self.number_format(),
            text_format: self.text_format(),
            vertical_alignment: self.vertical_alignment(),
            ..Default::default()
        })
    }

    fn border_side(&self, side: &modifier::BorderSide) -> Option<api::Border> {
        if self.0.borders.contains(side) {
            Some(self.border())
        } else {
            None
        }
    }

    /// https://developers.google.com/apps-script/reference/spreadsheet/border-style
    fn border_style(&self) -> Option<String> {
        self.0.border_style.clone().map(|bs| {
            match bs {
                modifier::BorderStyle::Dashed => "DASHED",
                modifier::BorderStyle::Dotted => "DOTTED",
                modifier::BorderStyle::Double => "DOUBLE",
                modifier::BorderStyle::Solid => "SOLID",
                modifier::BorderStyle::SolidMedium => "SOLID_MEDIUM",
                modifier::BorderStyle::SolidThick => "SOLID_THICK",
            }.to_string()
        })
    }

    fn borders(&self) -> Option<api::Borders> {
        if self.0.borders.is_empty() {
            return None
        }

        Some(api::Borders {
            bottom: self.border_side(&modifier::BorderSide::Bottom),
            left: self.border_side(&modifier::BorderSide::Left),
            right: self.border_side(&modifier::BorderSide::Right),
            top: self.border_side(&modifier::BorderSide::Top),
        })
    }

    fn border(&self) -> api::Border {
        api::Border {
            color_style: self.color_style(&self.0.border_color),
            // TODO: I might need to do a mapping to the google style formats here
            style: self.border_style(),
            ..Default::default()
        }
    }

    fn color_style(&self, rgb: &Option<Rgb>) -> Option<api::ColorStyle> {
        if let Some(rgb) = rgb {
            let (r, g, b): (f32, f32, f32) = rgb.into();

            Some(api::ColorStyle {
                rgb_color: Some(api::Color {
                    alpha: None,
                    red: Some(r),
                    green: Some(g),
                    blue: Some(b),
                }),
                theme_color: None,
            })
        } else {
            None
        }
    }

    fn horizontal_alignment(&self) -> Option<String> {
        self.0.horizontal_align.clone().map(|ha| {
            match ha {
                modifier::HorizontalAlign::Left => "LEFT",
                modifier::HorizontalAlign::Center => "MIDDLE",
                modifier::HorizontalAlign::Right => "RIGHT",
            }.to_string()
        })
    }

    /// https://docs.rs/google-sheets4/latest/google_sheets4/api/struct.NumberFormat.html
    fn number_format(&self) -> Option<api::NumberFormat> {
        todo!();
        Some(api::NumberFormat {
            ..Default::default()
        })
    }

    pub fn text_format(&self) -> Option<api::TextFormat> {
        // XXX return None if everything is empty
        //
        Some(api::TextFormat {
            bold: None.or(Some(self.0.formats.contains(&modifier::TextFormat::Bold))),
            font_family: None.or(self.0.font_family.clone()),
            font_size: None.or(self.0.font_size.map(|fs| fs as i32)),
            foreground_color: None,
            foreground_color_style: self.color_style(&self.0.font_color),
            italic: None.or(Some(self.0.formats.contains(&modifier::TextFormat::Italic))),
            link: None,
            strikethrough: None.or(Some(self.0.formats.contains(&modifier::TextFormat::Strikethrough))),
            underline: None.or(Some(self.0.formats.contains(&modifier::TextFormat::Underline))),
        })
    }

    fn vertical_alignment(&self) -> Option<String> {
        self.0.vertical_align.clone().map(|va| {
            match va {
                modifier::VerticalAlign::Top => "TOP",
                modifier::VerticalAlign::Center => "MIDDLE",
                modifier::VerticalAlign::Bottom => "BOTTOM",
            }.to_string()
        })
    }
}
