use super::{Modifier, RowModifier};

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

#[allow(clippy::from_over_into)]
impl RowModifier {
    pub(crate) fn into_without_var(self) -> Modifier {
        let mut modifier: Modifier = self.into();
        modifier.var = None;
        modifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn into() {
        let row_modifier = RowModifier {
            border_color: Some(Rgb::new(0, 0, 255)),
            note: Some("Test note".to_string()),
            font_size: Some(20),
            var: Some("foo".to_string()),
            ..Default::default()
        };
        let modifier: Modifier = row_modifier.into();

        assert_eq!(modifier.note, Some("Test note".to_string()));
        assert_eq!(modifier.font_size, Some(20));
        assert_eq!(modifier.border_color, Some(Rgb::new(0, 0, 255)));
        assert_eq!(modifier.var, Some("foo".to_string()));
    }

    #[test]
    fn into_without_var() {
        let row_modifier = RowModifier {
            border_color: Some(Rgb::new(0, 0, 255)),
            note: Some("Test note".to_string()),
            font_size: Some(20),
            var: Some("foo".to_string()),
            ..Default::default()
        };
        let modifier: Modifier = row_modifier.into_without_var();

        assert_eq!(modifier.note, Some("Test note".to_string()));
        assert_eq!(modifier.font_size, Some(20));
        assert_eq!(modifier.border_color, Some(Rgb::new(0, 0, 255)));
        assert_eq!(modifier.var, None);
    }
}
