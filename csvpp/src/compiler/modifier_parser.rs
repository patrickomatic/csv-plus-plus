//! Module for lexing and parsing the modifiers on a cell.
//!
//! TODO:
//! * fix the row parsing logic (it doesn't apply them)
//! * need to lowercase the input but we can't do it on the entire value because we don't want to
//!     lowercase the stuff outside the modifier definition
//! * get quoted strings working
use std::str::FromStr;
use crate::{Error, InnerError, InnerResult, Result, Rgb};
use crate::modifier::*;
use super::modifier_lexer::{ModifierLexer, Token};

pub struct ModifierParser<'a> {
    lexer: &'a mut ModifierLexer,
    modifier: &'a mut Modifier,
}

#[derive(Clone)]
pub struct ParsedModifiers {
    pub modifier: Modifier,
    pub row_modifier: Modifier,
    pub value: String,
    pub position: a1_notation::A1,
}

impl<'a> ModifierParser<'a> {
    pub fn parse(
        input: &str, 
        position: a1_notation::A1, 
        row_modifier: Modifier,
    ) -> Result<ParsedModifiers> {
        let lexer = &mut ModifierLexer::new(input);
        let (modifier, row_modifier) = Self::parse_all_modifiers(lexer, &position, row_modifier).map_err(|e| {
            Error::ModifierSyntaxError { position: position.clone(), message: e.to_string() }
        })?;

        Ok(ParsedModifiers {
            modifier,
            row_modifier,
            value: lexer.rest(),
            position,
        })
    }

    /// returns (modifier, row_modifier)
    fn parse_all_modifiers(
        lexer: &mut ModifierLexer,
        position: &a1_notation::A1,
        row_modifier: Modifier,
    ) -> InnerResult<(Modifier, Modifier)> {
        let mut new_modifier: Option<Modifier> = None;
        let mut new_row_modifier: Option<Modifier> = None;

        while let Some(start_token) = lexer.maybe_take_start_modifier() {
            let is_row_modifier = start_token == Token::StartRowModifier;
            if is_row_modifier && position.x() != Some(0) {
                return Err(InnerError::bad_input("![[", "You can only define a row modifier in the first cell"));
            }

            let mut modifier = Modifier::from(&new_row_modifier.clone().unwrap_or_else(|| row_modifier.clone()));

            // we'll instantiate a new parser for each modifier, but share the lexer so we're using
            // the same stream of tokens
            let mut modifier_parser = ModifierParser { 
                lexer,
                modifier: &mut modifier,
            };
            modifier_parser.modifiers()?;

            if is_row_modifier {
                new_row_modifier = Some(modifier)
            } else {
                new_modifier = Some(modifier)
            } 
        }

        Ok((
            new_modifier.unwrap_or_else(|| Modifier::from(&row_modifier)),
            new_row_modifier.unwrap_or(row_modifier),
        ))
    }

    fn border_modifier(&mut self) -> InnerResult<()> {
        self.modifier.borders.insert(BorderSide::from_str(&self.lexer.take_modifier_right_side()?)?);
        Ok(())
    }

    fn border_color_modifier(&mut self) -> InnerResult<()> {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.border_color = Some(Rgb::from_str(&color)?);
        Ok(())
    }

    fn border_style_modifier(&mut self) -> InnerResult<()> {
        self.modifier.border_style = Some(
            BorderStyle::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn color_modifier(&mut self) -> InnerResult<()> {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.color = Some(Rgb::from_str(&color)?);

        Ok(())
    }

    fn expand_modifier(&mut self) -> InnerResult<()> {
        let amount = if self.lexer.maybe_take_token(Token::Equals).is_some() {
            let amount_string = self.lexer.take_token(Token::PositiveNumber)?;
            
            match amount_string.parse::<usize>() {
                Ok(n) => 
                    Some(n),
                Err(e) => 
                    return Err(InnerError::bad_input(
                            &amount_string, 
                            &format!("Error parsing expand= repetitions: {}", e))),
            }
        } else {
            None
        };

        self.modifier.expand = Some(Expand { amount });

        Ok(())
    }

    fn font_color_modifier(&mut self) -> InnerResult<()> {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.font_color = Some(Rgb::from_str(&color)?);
        Ok(())
    }

    fn font_family_modifier(&mut self) -> InnerResult<()> {
        self.lexer.take_token(Token::Equals)?;

        let font_family = self.lexer.take_token(Token::String)?;
        self.modifier.font_family = Some(font_family);
        Ok(())
    }

    fn font_size_modifier(&mut self) -> InnerResult<()> {
        self.lexer.take_token(Token::Equals)?;

        let font_size_string = self.lexer.take_token(Token::PositiveNumber)?;
        match font_size_string.parse::<u8>() {
            Ok(n) => self.modifier.font_size = Some(n),
            Err(e) => return Err(InnerError::bad_input(
                &font_size_string,
                &format!("Error parsing fontsize: {}", e))),
        }

        Ok(())
    }

    fn format_modifier(&mut self) -> InnerResult<()> {
        self.modifier.formats.insert(TextFormat::from_str(&self.lexer.take_modifier_right_side()?)?);
        Ok(())
    }

    fn halign_modifier(&mut self) -> InnerResult<()> {
        self.modifier.horizontal_align = Some(
            HorizontalAlign::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn note(&mut self) -> InnerResult<()> {
        self.lexer.take_token(Token::Equals)?;
        self.modifier.note = Some(self.lexer.take_token(Token::String)?);
        Ok(())
    }

    fn number_format(&mut self) -> InnerResult<()> {
        self.modifier.number_format = Some(
            NumberFormat::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn valign_modifier(&mut self) -> InnerResult<()> {
        self.modifier.vertical_align = Some(
            VerticalAlign::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn var_modifier(&mut self) -> InnerResult<()> {
        todo!();
    }

    fn modifier(&mut self) -> InnerResult<()> {
        let modifier_name = self.lexer.take_token(Token::ModifierName)?;

        match modifier_name.as_str() {
            "b"  | "border"         => self.border_modifier(),
            "bc" | "bordercolor"    => self.border_color_modifier(),
            "bs" | "borderstyle"    => self.border_style_modifier(),
            "c"  | "color"          => self.color_modifier(),
            "e"  | "expand"         => self.expand_modifier(),
            "f"  | "format"         => self.format_modifier(),
            "fc" | "fontcolor"      => self.font_color_modifier(),
            "ff" | "fontfamily"     => self.font_family_modifier(),
            "fs" | "fontsize"       => self.font_size_modifier(),
            "ha" | "halign"         => self.halign_modifier(),
            "n"  | "note"           => self.note(),
            "nf" | "numberformat"   => self.number_format(),
            "v"  | "var"            => self.var_modifier(),
            "va" | "valign"         => self.valign_modifier(),
            _ => 
                Err(InnerError::bad_input(
                        &modifier_name, 
                        &format!("Unrecognized modifier: {}", &modifier_name))),
        }
    }

    fn modifiers(&mut self) -> InnerResult<()> {
        loop {
            self.modifier()?;

            if self.lexer.maybe_take_token(Token::Slash).is_none() {
                break
            }
        }

        self.lexer.take_token(Token::EndModifier)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(input: &str) -> ParsedModifiers {
        ModifierParser::parse(
            input,
            a1_notation::A1::builder().xy(0, 0).build().unwrap(),
            Modifier::new(true),
        ).unwrap()
    }

    #[test]
    fn parse_no_modifier() {
        let parsed_modifiers = test_parse("abc123");
        assert_eq!(parsed_modifiers.value, "abc123");
        assert!(!parsed_modifiers.modifier.row_level);
        assert!(parsed_modifiers.row_modifier.row_level);
    }

    #[test]
    fn parse_modifier() {
        let ParsedModifiers { value, modifier, .. } = test_parse("[[format=bold]]abc123");
        assert_eq!(value, "abc123");
        assert!(modifier.formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_multiple_modifiers() {
        let ParsedModifiers { value, modifier, .. } = test_parse("[[format=italic/valign=top/expand]]abc123");
        assert_eq!(value, "abc123");
        assert!(modifier.formats.contains(&TextFormat::Italic));
        assert_eq!(modifier.vertical_align, Some(VerticalAlign::Top));
        assert_eq!(modifier.expand, Some(Expand { amount: None }));
    }

    #[test]
    fn parse_multiple_modifiers_shorthand() {
        let ParsedModifiers { value, modifier, .. } = test_parse("[[ha=l/va=c/f=u/fs=12]]abc123");
        assert_eq!(value, "abc123");
        assert_eq!(modifier.font_size, Some(12));
        assert_eq!(modifier.horizontal_align, Some(HorizontalAlign::Left));
        assert_eq!(modifier.vertical_align, Some(VerticalAlign::Center));
        assert!(modifier.formats.contains(&TextFormat::Underline));
    }

    #[test]
    fn parse_row_modifier() {
        let ParsedModifiers { row_modifier, .. } = test_parse("![[format=bold]]abc123");
        assert!(row_modifier.formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_border() {
        let ParsedModifiers { modifier, .. } = test_parse("[[border=top]]abc123");
        assert!(modifier.borders.contains(&BorderSide::Top));
    }

    #[test]
    fn parse_borderstyle() {
        let ParsedModifiers { modifier, .. } = test_parse("[[b=t/bs=dotted]]abc123");
        assert_eq!(modifier.border_style, Some(BorderStyle::Dotted));
    }

    #[test]
    fn parse_color() {
        let ParsedModifiers { modifier, .. } = test_parse("[[color=#ABC]]abc123");
        assert!(modifier.color.is_some());
    }

    #[test]
    fn parse_expand() {
        let ParsedModifiers { modifier, .. } = test_parse("[[expand=20]]abc123");
        assert!(modifier.expand.is_some());
    }

    #[test]
    fn parse_fontcolor() {
        let ParsedModifiers { modifier, .. } = test_parse("[[fontcolor=#ABC]]abc123");
        assert!(modifier.font_color.is_some());
    }

    #[test]
    fn parse_fontfamily() {
        let ParsedModifiers { modifier, .. } = test_parse("[[fontfamily=Helvetica]]abc123");
        assert_eq!(modifier.font_family, Some("Helvetica".to_string()));
    }

    #[test]
    fn parse_fontsize() {
        let ParsedModifiers { modifier, .. } = test_parse("[[fontsize=20]]abc123");
        assert_eq!(modifier.font_size, Some(20));
    }

    #[test]
    fn parse_format() {
        let ParsedModifiers { modifier, .. } = test_parse("[[format=bold]]abc123");
        assert!(modifier.formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_halign() {
        let ParsedModifiers { modifier, .. } = test_parse("[[halign=left]]abc123");
        assert_eq!(modifier.horizontal_align, Some(HorizontalAlign::Left));
    }

    #[test]
    fn parse_note() {
        let ParsedModifiers { modifier, .. } = test_parse("[[note='foo']]abc123");
        assert_eq!(modifier.note, Some("foo".to_string()));
    }

    #[test]
    fn parse_numberformat() {
        let ParsedModifiers { modifier, .. } = test_parse("[[numberformat=datetime]]abc123");
        assert_eq!(modifier.number_format, Some(NumberFormat::DateTime));
    }

    #[test]
    fn parse_valign() {
        let ParsedModifiers { modifier, .. } = test_parse("[[valign=top]]abc123");
        assert_eq!(modifier.vertical_align, Some(VerticalAlign::Top));
    }
}
