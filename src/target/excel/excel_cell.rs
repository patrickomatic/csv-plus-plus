//! # `ExcelCell`
//!
//! Converts between a Cell and an `umya_spreadsheet::Style` (which feature-wise actually happen
//! to map pretty nicely)
use crate::cell_options::{
    BorderSide, BorderStyle, HorizontalAlign, NumberFormat, TextFormat, TextWrap, VerticalAlign,
};
use crate::{Cell, Rgb};

pub(super) struct ExcelCell<'a>(pub(super) &'a Cell);

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

impl From<ExcelCell<'_>> for umya_spreadsheet::Style {
    fn from(value: ExcelCell<'_>) -> Self {
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
            NumberFormat::Scientific => "0.00E+00",
        });
        nf
    }
}

impl From<Rgb> for umya_spreadsheet::Color {
    fn from(value: Rgb) -> Self {
        let mut color = umya_spreadsheet::Color::default();
        color.set_argb(value.to_rgba());
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

impl From<TextWrap> for bool {
    fn from(value: TextWrap) -> Self {
        matches!(value, TextWrap::Wrap)
    }
}

impl<'a> ExcelCell<'a> {
    fn set_alignment(&self, s: &mut umya_spreadsheet::Style) {
        let mut alignment = umya_spreadsheet::Alignment::default();
        alignment.set_horizontal(self.0.horizontal_align.into());
        alignment.set_vertical(self.0.vertical_align.into());

        alignment.set_wrap_text(self.0.text_wrap.into());

        s.set_alignment(alignment);
    }

    fn set_background_color(&self, s: &mut umya_spreadsheet::Style) {
        if let Some(c) = self.0.color.clone() {
            s.set_background_color(c.to_rgba());
        }
    }

    fn set_borders(&self, s: &mut umya_spreadsheet::Style) {
        if self.0.borders.is_empty() {
            return;
        }

        let border_style: umya_spreadsheet::Border = self.0.border_style.into();

        let b = s.get_borders_mut();
        if self.0.side_has_border(BorderSide::Left) {
            b.set_left_border(border_style.clone());
        }

        if self.0.side_has_border(BorderSide::Right) {
            b.set_right_border(border_style.clone());
        }

        if self.0.side_has_border(BorderSide::Top) {
            b.set_top_border(border_style.clone());
        }

        if self.0.side_has_border(BorderSide::Bottom) {
            b.set_bottom_border(border_style);
        }
    }

    fn set_font(&self, s: &mut umya_spreadsheet::Style) {
        if self.0.font_size.is_none()
            && self.0.font_color.is_none()
            && self.0.font_family.is_none()
            && self.0.text_formats.is_empty()
        {
            return;
        }

        let mut font = umya_spreadsheet::Font::default();

        if let Some(fs) = self.0.font_size {
            font.set_size(f64::from(fs));
        }

        if let Some(ff) = self.0.font_family.clone() {
            font.set_name(ff);
        }

        if let Some(fc) = self.0.font_color.clone() {
            font.set_color(fc.into());
        }

        if self.0.text_formats.contains(&TextFormat::Bold) {
            font.set_bold(true);
        }

        if self.0.text_formats.contains(&TextFormat::Italic) {
            font.set_italic(true);
        }

        if self.0.text_formats.contains(&TextFormat::Strikethrough) {
            font.set_strikethrough(true);
        }

        if self.0.text_formats.contains(&TextFormat::Underline) {
            font.set_underline("single");
        }

        s.set_font(font);
    }

    fn set_number_format(&self, s: &mut umya_spreadsheet::Style) {
        if let Some(nf) = self.0.number_format {
            s.set_numbering_format(nf.into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

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
    fn into_excel_cell_style() {
        let mut cell = Cell::new(build_field("", (0, 0)));
        cell.font_size = Some(50);
        cell.border_style = BorderStyle::Dashed;
        cell.note = Some("a note".to_string());
        cell.borders.insert(BorderSide::Top);
        cell.text_formats.insert(TextFormat::Bold);

        let style: umya_spreadsheet::Style = ExcelCell(&cell).into();
        assert!((style.get_font().unwrap().get_size() - 50.0).abs() < f64::EPSILON);
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
        let rgb = Rgb { r: 255, g: 0, b: 0 };
        let color: umya_spreadsheet::Color = rgb.into();

        assert_eq!(color.get_argb(), "FF0000FF");
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
