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

macro_rules! validate_str {
    ($gs_name:literal $(, $dv:ident)*) => {
        api::BooleanCondition {
            type_: Some($gs_name.to_string()),
            values: Some(vec![
                $(
                    api::ConditionValue {
                        user_entered_value: Some($dv.to_string()),
                        ..Default::default()
                    },
                )*
            ]),
        }
    };
}

// TODO: make the underlying macro work with the above?
macro_rules! validate_date {
    ($gs_name:literal $(, $dv:ident)*) => {
        api::BooleanCondition {
            type_: Some($gs_name.to_string()),
            values: Some(vec![
                $(
                    api::ConditionValue {
                        relative_date: Some($dv.to_string()),
                        ..Default::default()
                    },
                )*
            ]),
        }
    };
}

impl<'a> GoogleSheetsModifier<'a> {
    pub(super) fn cell_format(&self) -> Option<api::CellFormat> {
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

    pub(super) fn data_validation_rule(&self) -> Option<api::DataValidationRule> {
        Some(api::DataValidationRule {
            condition: Some(match self.0.data_validation.as_ref()? {
                modifier::DataValidation::Custom(c) => validate_str!("CUSTOM", c),
                modifier::DataValidation::DateAfter(d) => validate_date!("DATE_AFTER", d),
                modifier::DataValidation::DateBefore(d) => validate_date!("DATE_BEFORE", d),
                modifier::DataValidation::DateBetween(da, db) => {
                    validate_date!("DATE_BETWEEN", da, db)
                }
                modifier::DataValidation::DateEqualTo(d) => validate_date!("DATE_EQUAL_TO", d),
                modifier::DataValidation::DateIsValid => validate_date!("DATE_IS_VALID_DATE"),
                modifier::DataValidation::DateNotBetween(da, db) => {
                    validate_date!("DATE_NOT_BETWEEN", da, db)
                }
                modifier::DataValidation::DateOnOrAfter(d) => validate_date!("DATE_ON_OR_AFTER", d),
                modifier::DataValidation::DateOnOrBefore(d) => {
                    validate_date!("DATE_ON_OR_BEFORE", d)
                }
                // TODO: these might need to be prefixed with `"="`
                modifier::DataValidation::NumberBetween(na, nb) => {
                    validate_str!("NUMBER_BETWEEN", na, nb)
                }
                modifier::DataValidation::NumberEqualTo(n) => validate_str!("NUMBER_EQUAL_TO", n),
                modifier::DataValidation::NumberGreaterThan(n) => {
                    validate_str!("NUMBER_GREATER_THAN", n)
                }
                modifier::DataValidation::NumberGreaterThanOrEqualTo(n) => {
                    validate_str!("NUMBER_GREATER_THAN_OR_EQUAL_TO", n)
                }
                modifier::DataValidation::NumberLessThan(n) => {
                    validate_str!("NUMBER_LESS_THAN", n)
                }
                modifier::DataValidation::NumberLessThanOrEqualTo(n) => {
                    validate_str!("NUMBER_LESS_THAN_OR_EQUAL_TO", n)
                }
                modifier::DataValidation::NumberNotBetween(na, nb) => {
                    validate_str!("NUMBER_NOT_BETWEEN", na, nb)
                }
                modifier::DataValidation::NumberNotEqualTo(n) => {
                    validate_str!("NUMBER_NOT_EQUAL_TO", n)
                }
                modifier::DataValidation::TextContains(t) => validate_str!("TEXT_CONTAINS", t),
                modifier::DataValidation::TextDoesNotContain(t) => {
                    validate_str!("TEXT_DOES_NOT_CONTAIN", t)
                }
                modifier::DataValidation::TextEqualTo(t) => validate_str!("TEXT_EQUAL_TO", t),
                modifier::DataValidation::TextIsValidEmail => validate_str!("TEXT_IS_VALID_EMAIL"),
                modifier::DataValidation::TextIsValidUrl => validate_str!("TEXT_IS_VALID_URL"),
                modifier::DataValidation::ValueInList(_) => todo!(),
                modifier::DataValidation::ValueInRange => todo!(),
            }),
            // TODO: show a helpful message?
            input_message: None,
            // TODO: I dunno?
            show_custom_ui: None,
            // TODO: maybe make a CLI flag? if true, the spreadsheet will reject the data
            strict: None,
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
