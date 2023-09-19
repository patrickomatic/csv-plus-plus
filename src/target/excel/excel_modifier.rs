//! # ExcelModifier
//!
//! Converts between a Modifier and an umya_spreadsheet::Style (which feature-wise actually happen
//! to map pretty nicely)
use crate::modifier::{
    BorderSide, BorderStyle, HorizontalAlign, NumberFormat, TextFormat, VerticalAlign,
};
use crate::{Modifier, Rgb};

// A Newtype around `Modifier` which allows us to have umya/excel-specific functionality
pub(super) struct ExcelModifier(pub Modifier);

impl From<BorderStyle> for umya_spreadsheet::Border {
    fn from(value: BorderStyle) -> Self {
        let mut b = umya_spreadsheet::Border::default();
        b.set_border_style(match value {
            BorderStyle::Dashed => umya_spreadsheet::Border::BORDER_DASHED,
            BorderStyle::Dotted => umya_spreadsheet::Border::BORDER_DOTTED,
            BorderStyle::Double => umya_spreadsheet::Border::BORDER_DOUBLE,
            BorderStyle::Solid => umya_spreadsheet::Border::BORDER_THIN,
            BorderStyle::SolidMedium => umya_spreadsheet::Border::BORDER_MEDIUM,
            BorderStyle::SolidThick => umya_spreadsheet::Border::BORDER_THICK,
        });
        b
    }
}

impl From<ExcelModifier> for umya_spreadsheet::Style {
    fn from(value: ExcelModifier) -> Self {
        let mut style = umya_spreadsheet::Style::default();

        value.set_alignment(&mut style);
        value.set_background_color(&mut style);
        value.set_borders(&mut style);
        value.set_font(&mut style);
        value.set_number_format(&mut style);

        style
    }
}

impl From<HorizontalAlign> for umya_spreadsheet::HorizontalAlignmentValues {
    fn from(value: HorizontalAlign) -> Self {
        match value {
            HorizontalAlign::Center => umya_spreadsheet::HorizontalAlignmentValues::Center,
            HorizontalAlign::Left => umya_spreadsheet::HorizontalAlignmentValues::Left,
            HorizontalAlign::Right => umya_spreadsheet::HorizontalAlignmentValues::Right,
        }
    }
}

impl From<NumberFormat> for umya_spreadsheet::NumberingFormat {
    fn from(value: NumberFormat) -> Self {
        let mut nf = umya_spreadsheet::NumberingFormat::default();
        nf.set_format_code(match value {
            NumberFormat::Currency => umya_spreadsheet::NumberingFormat::FORMAT_CURRENCY_USD,
            NumberFormat::Date => umya_spreadsheet::NumberingFormat::FORMAT_DATE_YYYYMMDD,
            NumberFormat::DateTime => umya_spreadsheet::NumberingFormat::FORMAT_DATE_DATETIME,
            NumberFormat::Number => umya_spreadsheet::NumberingFormat::FORMAT_NUMBER,
            NumberFormat::Percent => umya_spreadsheet::NumberingFormat::FORMAT_PERCENTAGE,
            NumberFormat::Text => umya_spreadsheet::NumberingFormat::FORMAT_TEXT,
            NumberFormat::Time => umya_spreadsheet::NumberingFormat::FORMAT_DATE_TIME1,
            NumberFormat::Scientific =>
            // TODO: I dunno if excel has a "scientific" formatting?
            {
                umya_spreadsheet::NumberingFormat::FORMAT_NUMBER
            }
        });
        nf
    }
}

impl From<Rgb> for umya_spreadsheet::Color {
    fn from(value: Rgb) -> Self {
        let mut color = umya_spreadsheet::Color::default();
        color.set_argb(value.to_string());
        color
    }
}

impl From<VerticalAlign> for umya_spreadsheet::VerticalAlignmentValues {
    fn from(value: VerticalAlign) -> Self {
        match value {
            VerticalAlign::Center => umya_spreadsheet::VerticalAlignmentValues::Center,
            VerticalAlign::Top => umya_spreadsheet::VerticalAlignmentValues::Top,
            VerticalAlign::Bottom => umya_spreadsheet::VerticalAlignmentValues::Bottom,
        }
    }
}

impl ExcelModifier {
    fn set_alignment(&self, s: &mut umya_spreadsheet::Style) {
        if self.0.horizontal_align.is_none() && self.0.vertical_align.is_none() {
            return;
        }

        let mut alignment = umya_spreadsheet::Alignment::default();
        if let Some(h) = self.0.horizontal_align.clone() {
            alignment.set_horizontal(h.into());
        }

        if let Some(v) = self.0.vertical_align.clone() {
            alignment.set_vertical(v.into());
        }

        s.set_alignment(alignment);
    }

    fn set_background_color(&self, s: &mut umya_spreadsheet::Style) {
        if let Some(c) = self.0.color.clone() {
            s.set_background_color(c.to_string());
        }
    }

    fn set_borders(&self, s: &mut umya_spreadsheet::Style) {
        if self.0.borders.is_empty() {
            return;
        }

        let border: umya_spreadsheet::Border =
            self.0.clone().border_style.unwrap_or_default().into();

        let b = s.get_borders_mut();
        if self.0.borders.contains(&BorderSide::All) || self.0.borders.contains(&BorderSide::Left) {
            b.set_left_border(border.clone());
        }

        if self.0.borders.contains(&BorderSide::All) || self.0.borders.contains(&BorderSide::Right)
        {
            b.set_right_border(border.clone());
        }

        if self.0.borders.contains(&BorderSide::All) || self.0.borders.contains(&BorderSide::Top) {
            b.set_top_border(border.clone());
        }

        if self.0.borders.contains(&BorderSide::All) || self.0.borders.contains(&BorderSide::Bottom)
        {
            b.set_bottom_border(border);
        }
    }

    fn set_font(&self, s: &mut umya_spreadsheet::Style) {
        if self.0.font_size.is_none()
            && self.0.font_color.is_none()
            && self.0.font_family.is_none()
            && self.0.formats.is_empty()
        {
            return;
        }

        let mut font = umya_spreadsheet::Font::default();

        if let Some(fs) = self.0.font_size {
            font.set_size(fs as f64);
        }

        if let Some(ff) = self.0.font_family.clone() {
            font.set_name(ff);
        }

        if let Some(fc) = self.0.font_color.clone() {
            font.set_color(fc.into());
        }

        if self.0.formats.contains(&TextFormat::Bold) {
            font.set_bold(true);
        }

        if self.0.formats.contains(&TextFormat::Italic) {
            font.set_italic(true);
        }

        if self.0.formats.contains(&TextFormat::Strikethrough) {
            font.set_strikethrough(true);
        }

        if self.0.formats.contains(&TextFormat::Underline) {
            font.set_underline("single");
        }

        s.set_font(font);
    }

    fn set_number_format(&self, s: &mut umya_spreadsheet::Style) {
        if let Some(nf) = self.0.number_format.clone() {
            s.set_numbering_format(nf.into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn into_border_borderstyle() {
        let dashed: umya_spreadsheet::Border = BorderStyle::Dashed.into();
        assert_eq!(dashed.get_border_style(), "dashed");

        let dotted: umya_spreadsheet::Border = BorderStyle::Dotted.into();
        assert_eq!(dotted.get_border_style(), "dotted");

        let double: umya_spreadsheet::Border = BorderStyle::Double.into();
        assert_eq!(double.get_border_style(), "double");

        let solid: umya_spreadsheet::Border = BorderStyle::Solid.into();
        assert_eq!(solid.get_border_style(), "thin");

        let solid_medium: umya_spreadsheet::Border = BorderStyle::SolidMedium.into();
        assert_eq!(solid_medium.get_border_style(), "medium");

        let solid_thick: umya_spreadsheet::Border = BorderStyle::SolidThick.into();
        assert_eq!(solid_thick.get_border_style(), "thick");
    }

    #[test]
    fn into_excel_modifier_style() {
        let mut modifier = Modifier {
            font_size: Some(50),
            border_style: Some(BorderStyle::Dashed),
            note: Some("a note".to_string()),

            ..Default::default()
        };
        modifier.borders.insert(BorderSide::Top);
        modifier.formats.insert(TextFormat::Bold);

        let style: umya_spreadsheet::Style = ExcelModifier(modifier).into();
        assert_eq!(style.get_font().clone().unwrap().get_size(), &50.0);
    }

    #[test]
    fn into_horizontal_align_horizontal_alignment_values() {
        let left: umya_spreadsheet::HorizontalAlignmentValues = HorizontalAlign::Left.into();
        assert_eq!(left, umya_spreadsheet::HorizontalAlignmentValues::Left);

        let center: umya_spreadsheet::HorizontalAlignmentValues = HorizontalAlign::Center.into();
        assert_eq!(center, umya_spreadsheet::HorizontalAlignmentValues::Center);

        let right: umya_spreadsheet::HorizontalAlignmentValues = HorizontalAlign::Right.into();
        assert_eq!(right, umya_spreadsheet::HorizontalAlignmentValues::Right);
    }

    #[test]
    fn into_number_format_numbering_format() {
        // TODO
    }

    #[test]
    fn into_rgb_color() {
        let rgb = Rgb::from_str("FFAA00").unwrap();
        let color: umya_spreadsheet::Color = rgb.into();

        assert_eq!(color.get_argb(), "#FFAA00");
    }

    #[test]
    fn into_vertical_align_vertical_alignment_values() {
        let top: umya_spreadsheet::VerticalAlignmentValues = VerticalAlign::Top.into();
        assert_eq!(top, umya_spreadsheet::VerticalAlignmentValues::Top);

        let center: umya_spreadsheet::VerticalAlignmentValues = VerticalAlign::Center.into();
        assert_eq!(center, umya_spreadsheet::VerticalAlignmentValues::Center);

        let bottom: umya_spreadsheet::VerticalAlignmentValues = VerticalAlign::Bottom.into();
        assert_eq!(bottom, umya_spreadsheet::VerticalAlignmentValues::Bottom);
    }
}
