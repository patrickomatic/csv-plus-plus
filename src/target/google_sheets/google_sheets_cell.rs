//! # `GoogleSheetsCell`
//!
//! A wrapper around `Cell` to make it more compatible with the Google Sheets API.  An important
//! design principal here is that we only send the changes that the user set - we should not be
//! sending a bunch of default values and prefer to return None instead.  In other words the API
//! payloads should reflect only the things the user specified on the cell.
//!
use crate::cell_options::{
    BorderSide, BorderStyle, DataValidation, HorizontalAlign, NumberFormat, TextFormat, TextWrap,
    VerticalAlign,
};
use crate::{Cell, Rgb};
use google_sheets4::api;

pub(super) struct GoogleSheetsCell<'a>(pub(super) &'a Cell);

macro_rules! build_boolean_condition {
    ($gs_name:literal, $conditional_value_attr:ident $(, $dv:ident)*) => {
        api::BooleanCondition {
            type_: Some($gs_name.to_string()),
            values: Some(vec![
                $(
                    api::ConditionValue {
                        $conditional_value_attr: Some($dv.to_string()),
                        ..Default::default()
                    },
                )*
            ]),
        }
    };
}

macro_rules! validate_date {
    ($gs_name:literal $(, $dv:ident)*) => {
        build_boolean_condition!($gs_name, relative_date $(,$dv)*)
    };
}

macro_rules! validate_str {
    ($gs_name:literal $(, $dv:ident)*) => {
        build_boolean_condition!($gs_name, user_entered_value $(,$dv)*)
    };
}

fn color_style(rgb: &Option<Rgb>) -> Option<api::ColorStyle> {
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

impl<'a> GoogleSheetsCell<'a> {
    pub(super) fn cell_format(&self) -> api::CellFormat {
        let borders = self.borders();
        let background_color_style = color_style(&self.0.color);
        let horizontal_alignment = self.horizontal_alignment();
        let number_format = self.number_format();
        let text_format = self.text_format();
        let vertical_alignment = self.vertical_alignment();
        let wrap_strategy = self.wrap_strategy();

        api::CellFormat {
            background_color_style,
            borders,
            horizontal_alignment: Some(horizontal_alignment),
            number_format: Some(number_format),
            text_format,
            vertical_alignment: Some(vertical_alignment),
            wrap_strategy: Some(wrap_strategy),
            ..Default::default()
        }
    }

    pub(super) fn data_validation_rule(&self) -> Option<api::DataValidationRule> {
        Some(api::DataValidationRule {
            condition: Some(match self.0.data_validation.as_ref()? {
                DataValidation::Custom(c) => validate_str!("CUSTOM", c),
                DataValidation::DateAfter(d) => validate_date!("DATE_AFTER", d),
                DataValidation::DateBefore(d) => validate_date!("DATE_BEFORE", d),
                DataValidation::DateBetween(da, db) => {
                    validate_date!("DATE_BETWEEN", da, db)
                }
                DataValidation::DateEqualTo(d) => validate_date!("DATE_EQUAL_TO", d),
                DataValidation::DateIsValid => validate_date!("DATE_IS_VALID_DATE"),
                DataValidation::DateNotBetween(da, db) => {
                    validate_date!("DATE_NOT_BETWEEN", da, db)
                }
                DataValidation::DateOnOrAfter(d) => validate_date!("DATE_ON_OR_AFTER", d),
                DataValidation::DateOnOrBefore(d) => {
                    validate_date!("DATE_ON_OR_BEFORE", d)
                }
                // TODO: these might need to be prefixed with `"="`
                DataValidation::NumberBetween(na, nb) => {
                    validate_str!("NUMBER_BETWEEN", na, nb)
                }
                DataValidation::NumberEqualTo(n) => validate_str!("NUMBER_EQUAL_TO", n),
                DataValidation::NumberGreaterThan(n) => {
                    validate_str!("NUMBER_GREATER_THAN", n)
                }
                DataValidation::NumberGreaterThanOrEqualTo(n) => {
                    validate_str!("NUMBER_GREATER_THAN_OR_EQUAL_TO", n)
                }
                DataValidation::NumberLessThan(n) => {
                    validate_str!("NUMBER_LESS_THAN", n)
                }
                DataValidation::NumberLessThanOrEqualTo(n) => {
                    validate_str!("NUMBER_LESS_THAN_OR_EQUAL_TO", n)
                }
                DataValidation::NumberNotBetween(na, nb) => {
                    validate_str!("NUMBER_NOT_BETWEEN", na, nb)
                }
                DataValidation::NumberNotEqualTo(n) => {
                    validate_str!("NUMBER_NOT_EQUAL_TO", n)
                }
                DataValidation::TextContains(t) => validate_str!("TEXT_CONTAINS", t),
                DataValidation::TextDoesNotContain(t) => {
                    validate_str!("TEXT_DOES_NOT_CONTAIN", t)
                }
                DataValidation::TextEqualTo(t) => validate_str!("TEXT_EQUAL_TO", t),
                DataValidation::TextIsValidEmail => validate_str!("TEXT_IS_VALID_EMAIL"),
                DataValidation::TextIsValidUrl => validate_str!("TEXT_IS_VALID_URL"),
                DataValidation::ValueInList(list) => api::BooleanCondition {
                    type_: Some("VALUE_IN_LIST".to_string()),
                    values: Some(
                        list.iter()
                            .map(|l| api::ConditionValue {
                                user_entered_value: Some(l.to_string()),
                                ..Default::default()
                            })
                            .collect(),
                    ),
                },
                DataValidation::ValueInRange(a1) => validate_str!("VALUE_IN_RANGE", a1),
            }),
            // TODO: show a helpful message?
            input_message: None,
            // TODO: I dunno?
            show_custom_ui: None,
            // TODO: build this into the syntax
            strict: None,
        })
    }

    fn border_side(&self, side: BorderSide) -> Option<api::Border> {
        if self.0.borders.contains(&side) {
            Some(self.border())
        } else {
            None
        }
    }

    /// <https://developers.google.com/apps-script/reference/spreadsheet/border-style>
    fn border_style(&self) -> String {
        match self.0.border_style {
            BorderStyle::Dashed => "DASHED",
            BorderStyle::Dotted => "DOTTED",
            BorderStyle::Double => "DOUBLE",
            BorderStyle::Solid => "SOLID",
            BorderStyle::SolidMedium => "SOLID_MEDIUM",
            BorderStyle::SolidThick => "SOLID_THICK",
        }
        .to_string()
    }

    fn borders(&self) -> Option<api::Borders> {
        if self.0.borders.is_empty() {
            return None;
        }

        Some(api::Borders {
            bottom: self.border_side(BorderSide::Bottom),
            left: self.border_side(BorderSide::Left),
            right: self.border_side(BorderSide::Right),
            top: self.border_side(BorderSide::Top),
        })
    }

    fn border(&self) -> api::Border {
        api::Border {
            color_style: color_style(&self.0.border_color),
            style: Some(self.border_style()),
            ..Default::default()
        }
    }

    fn format_as_option(&self, format: TextFormat) -> Option<bool> {
        if self.0.text_formats.contains(&format) {
            Some(true)
        } else {
            None
        }
    }

    fn horizontal_alignment(&self) -> String {
        match self.0.horizontal_align {
            HorizontalAlign::Left => "LEFT",
            HorizontalAlign::Center => "MIDDLE",
            HorizontalAlign::Right => "RIGHT",
        }
        .to_string()
    }

    /// <https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/cells#numberformat>
    /// <https://docs.rs/google-sheets4/latest/google_sheets4/api/struct.NumberFormat.html>
    fn number_format(&self) -> api::NumberFormat {
        let nf_type = match self.0.number_format {
            NumberFormat::Currency => "CURRENCY",
            NumberFormat::Date => "DATE",
            NumberFormat::DateTime => "DATE_TIME",
            NumberFormat::Number => "NUMBER",
            NumberFormat::Percent => "PERCENT",
            NumberFormat::Text => "TEXT",
            NumberFormat::Time => "TIME",
            NumberFormat::Scientific => "SCIENTIFIC",
        }
        .to_string();

        api::NumberFormat {
            type_: Some(nf_type),
            pattern: None,
        }
    }

    fn text_format(&self) -> Option<api::TextFormat> {
        let bold = self.format_as_option(TextFormat::Bold);
        let font_family = self.0.font_family.clone();
        let font_size = self.0.font_size.map(i32::from);
        let foreground_color_style = color_style(&self.0.font_color);
        let italic = self.format_as_option(TextFormat::Italic);
        let strikethrough = self.format_as_option(TextFormat::Strikethrough);
        let underline = self.format_as_option(TextFormat::Underline);

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

    fn vertical_alignment(&self) -> String {
        match self.0.vertical_align {
            VerticalAlign::Top => "TOP",
            VerticalAlign::Center => "MIDDLE",
            VerticalAlign::Bottom => "BOTTOM",
        }
        .to_string()
    }

    fn wrap_strategy(&self) -> String {
        match self.0.text_wrap {
            TextWrap::Wrap => "WRAP",
            TextWrap::Overflow => "OVERFLOW",
            TextWrap::Clip => "CLIP",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn cell_format() {
        let mut cell = Cell::new(build_field("", (0, 0)));
        cell.text_formats.insert(TextFormat::Bold);
        cell.vertical_align = VerticalAlign::Top;
        cell.horizontal_align = HorizontalAlign::Right;
        cell.number_format = NumberFormat::Date;
        cell.color = Some(Rgb::new(255, 0, 0));
        cell.borders.insert(BorderSide::All);

        let gs_cell = GoogleSheetsCell(&cell);
        let cell_format = gs_cell.cell_format();

        assert!(cell_format.borders.is_some());
        assert!(cell_format.background_color_style.is_some());
        assert!(cell_format.number_format.is_some());
        assert!(cell_format.text_format.is_some());
        assert!(cell_format.vertical_alignment.is_some());
        assert!(cell_format.horizontal_alignment.is_some());
    }

    #[test]
    fn data_validation_rule_none() {
        let cell = Cell::new(build_field("", (0, 0)));
        let gs_cell = GoogleSheetsCell(&cell);

        assert!(gs_cell.data_validation_rule().is_none());
    }

    #[test]
    fn data_validation_rule_some() {
        let mut cell = Cell::new(build_field("", (0, 0)));
        cell.data_validation = Some(DataValidation::DateIsValid);
        let gs_cell = GoogleSheetsCell(&cell);

        assert!(gs_cell.data_validation_rule().is_some());
    }
}
