//! # GoogleSheetsModifier
//!
//! A wrapper around `Modifier` to make it more compatible with the Google Sheets API.  An
//! important design principal here is that we only send the changes that the user set - we should
//! not be sending a bunch of default values and prefer to return None instead.  In other words the
//! API payloads should reflect only the things the user specified in the modifier.
//!
use crate::modifier;
use crate::{Modifier, Rgb};
use google_sheets4::api;

pub struct GoogleSheetsModifier<'a>(pub &'a Modifier);

impl<'a> GoogleSheetsModifier<'a> {
    pub fn cell_format(&self) -> Option<api::CellFormat> {
        let borders = self.borders();
        let background_color_style = self.color_style(&self.0.color);
        let horizontal_alignment = self.horizontal_alignment();
        let number_format = self.number_format();
        let text_format = self.text_format();
        let vertical_alignment = self.vertical_alignment();

        if borders.is_none()
            && background_color_style.is_none()
            && horizontal_alignment.is_none()
            && number_format.is_none()
            && text_format.is_none()
            && vertical_alignment.is_none()
        {
            return None;
        }

        Some(api::CellFormat {
            background_color_style,
            borders,
            horizontal_alignment,
            number_format,
            text_format,
            vertical_alignment,
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
            }
            .to_string()
        })
    }

    fn borders(&self) -> Option<api::Borders> {
        if self.0.borders.is_empty() {
            return None;
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

    fn format_as_option(&self, format: &modifier::TextFormat) -> Option<bool> {
        if self.0.formats.contains(format) {
            Some(true)
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
            }
            .to_string()
        })
    }

    /// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/cells#numberformat
    /// https://docs.rs/google-sheets4/latest/google_sheets4/api/struct.NumberFormat.html
    fn number_format(&self) -> Option<api::NumberFormat> {
        self.0.number_format.clone().map(|nf| {
            let nf_type = match nf {
                modifier::NumberFormat::Currency => "CURRENCY",
                modifier::NumberFormat::Date => "DATE",
                modifier::NumberFormat::DateTime => "DATE_TIME",
                modifier::NumberFormat::Number => "NUMBER",
                modifier::NumberFormat::Percent => "PERCENT",
                modifier::NumberFormat::Text => "TEXT",
                modifier::NumberFormat::Time => "TIME",
                modifier::NumberFormat::Scientific => "SCIENTIFIC",
            }
            .to_string();

            api::NumberFormat {
                type_: Some(nf_type),
                pattern: None,
            }
        })
    }

    fn text_format(&self) -> Option<api::TextFormat> {
        let bold = self.format_as_option(&modifier::TextFormat::Bold);
        let font_family = self.0.font_family.clone();
        let font_size = self.0.font_size.map(|fs| fs as i32);
        let foreground_color_style = self.color_style(&self.0.font_color);
        let italic = self.format_as_option(&modifier::TextFormat::Italic);
        let strikethrough = self.format_as_option(&modifier::TextFormat::Strikethrough);
        let underline = self.format_as_option(&modifier::TextFormat::Underline);

        if font_family.is_none()
            && font_size.is_none()
            && foreground_color_style.is_none()
            && bold.is_none()
            && italic.is_none()
            && strikethrough.is_none()
            && underline.is_none()
        {
            return None;
        }

        Some(api::TextFormat {
            bold,
            font_family,
            font_size,
            foreground_color: None,
            foreground_color_style,
            italic,
            link: None,
            strikethrough,
            underline,
        })
    }

    fn vertical_alignment(&self) -> Option<String> {
        self.0.vertical_align.clone().map(|va| {
            match va {
                modifier::VerticalAlign::Top => "TOP",
                modifier::VerticalAlign::Center => "MIDDLE",
                modifier::VerticalAlign::Bottom => "BOTTOM",
            }
            .to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modifier;

    #[test]
    fn cell_format_none() {
        let modifier = Modifier::default();
        let gs_modifier = GoogleSheetsModifier(&modifier);
        let cell_format = gs_modifier.cell_format();

        assert!(cell_format.is_none());
    }

    #[test]
    fn cell_format_some() {
        let mut modifier = Modifier::default();
        modifier.formats.insert(modifier::TextFormat::Bold);
        modifier.vertical_align = Some(modifier::VerticalAlign::Top);
        let gs_modifier = GoogleSheetsModifier(&modifier);
        let cell_format = gs_modifier.cell_format().unwrap();

        assert!(cell_format.text_format.is_some());
        assert!(cell_format.vertical_alignment.is_some());
    }
}
