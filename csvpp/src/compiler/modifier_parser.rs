//! Module for lexing and parsing the modifiers on a cell.
//!
//! TODO:
//! * fix the row parsing logic (it doesn't apply them)
//! * need to lowercase the input but we can't do it on the entire value because we don't want to
//!     lowercase the stuff outside the modifier definition
//! * get quoted strings working
use std::str::FromStr;

use crate::a1::A1;
use crate::{Error, Result, Rgb};
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
    pub index: A1,
}

impl<'a> ModifierParser<'a> {
    pub fn parse(
        index: A1, 
        input: &str, 
        default_from: Modifier,
    ) -> Result<ParsedModifiers> {
        let lexer = &mut ModifierLexer::new(input);

        match Self::parse_all_modifiers(lexer, &default_from) {
            Ok((modifier, row_modifier)) => {
                // XXX fix the row modifier logic
                // if row_modifier.is_some() && !index.can_have_row_modifier() {
                if row_modifier.is_some() {
                    Err(Error::ModifierSyntaxError { 
                        bad_input: lexer.rest(),
                        index,
                        message: "You can only define a row modifier on the first cell of a row".to_string(), 
                    })
                } else {
                    Ok(ParsedModifiers {
                        modifier: modifier.unwrap_or_else(|| Modifier::from(&default_from)),
                        row_modifier: row_modifier.unwrap_or(default_from.clone()),
                        value: lexer.rest(),
                        index
                    })
                }
            },
            Err(message) => {
                Err(Error::ModifierSyntaxError { 
                    bad_input: lexer.rest(),
                    index,
                    message: message.to_string(), 
                })
            },
        }
    }

    /// returns (modifier, row_modifier)
    pub fn parse_all_modifiers(
        lexer: &mut ModifierLexer,
        default_from: &Modifier,
    ) -> Result<(Option<Modifier>, Option<Modifier>)> {
        let mut modifier: Option<Modifier> = None;
        let mut row_modifier: Option<Modifier> = None;

        while let Some(start_token) = lexer.maybe_take_start_modifier() {
            let is_row_modifier = start_token == Token::StartRowModifier;
            let mut new_modifier = Modifier::from(default_from);
            let mut modifier_parser = ModifierParser { 
                lexer,
                modifier: &mut new_modifier,
            };

            modifier_parser.modifiers()?;

            if is_row_modifier {
                if row_modifier.is_some() {
                    return Err(Error::ModifierSyntaxError {
                        bad_input: "![[".to_owned(),
                        index: A1::builder().xy(0, 0).build()?, // XXX
                        message: "You can only define one row modifier for a cell".to_string(),
                    })
                } 

                row_modifier = Some(new_modifier)
            } else {
                if modifier.is_some() {
                    return Err(Error::ModifierSyntaxError {
                        bad_input: "[[".to_owned(),
                        index: A1::builder().xy(0, 0).build()?, // XXX
                        message: "You can only define one modifier for a cell".to_string(),
                    })
                }

                modifier = Some(new_modifier)
            }
        }

        Ok((modifier, row_modifier))
    }

    fn border_modifier(&mut self) -> Result<()> {
        self.modifier.borders.insert(BorderSide::from_str(&self.lexer.take_modifier_right_side()?)?);
        Ok(())
    }

    fn border_color_modifier(&mut self) -> Result<()> {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.border_color = Some(Rgb::from_str(&color)?);
        Ok(())
    }

    fn border_style_modifier(&mut self) -> Result<()> {
        self.modifier.border_style = Some(
            BorderStyle::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn color_modifier(&mut self) -> Result<()> {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.color = Some(Rgb::from_str(&color)?);

        Ok(())
    }

    fn expand_modifier(&mut self) -> Result<()> {
        let amount = if self.lexer.maybe_take_token(Token::Equals).is_some() {
            let amount_string = self.lexer.take_token(Token::PositiveNumber)?;
            
            match amount_string.parse::<usize>() {
                Ok(n) => Some(n),
                Err(e) => return Err(Error::ModifierSyntaxError {
                    message: format!("Error parsing expand= repetitions: {}", e),
                    bad_input: amount_string,
                    index: A1::builder().xy(0, 0).build()?, // XXX
                }),
            }
        } else {
            None
        };

        self.modifier.expand = Some(Expand { amount });

        Ok(())
    }

    fn font_color_modifier(&mut self) -> Result<()> {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.font_color = Some(Rgb::from_str(&color)?);
        Ok(())
    }

    fn font_family_modifier(&mut self) -> Result<()> {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.font_color = Some(Rgb::from_str(&color)?);
        Ok(())
    }

    fn font_size_modifier(&mut self) -> Result<()> {
        self.lexer.take_token(Token::Equals)?;

        let font_size_string = self.lexer.take_token(Token::PositiveNumber)?;
        match font_size_string.parse::<u8>() {
            Ok(n) => self.modifier.font_size = Some(n),
            Err(e) => return Err(Error::ModifierSyntaxError {
                message: format!("Error parsing fontsize: {}", e),
                bad_input: font_size_string,
                index: A1::builder().xy(0, 0).build()?, // XXX
            }),
        }

        Ok(())
    }

    fn format_modifier(&mut self) -> Result<()> {
        self.modifier.formats.insert(TextFormat::from_str(&self.lexer.take_modifier_right_side()?)?);
        Ok(())
    }

    fn halign_modifier(&mut self) -> Result<()> {
        self.modifier.horizontal_align = Some(
            HorizontalAlign::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn note(&mut self) -> Result<()> {
        self.modifier.note = Some(self.lexer.take_token(Token::String)?);
        Ok(())
    }

    fn number_format(&mut self) -> Result<()> {
        self.modifier.number_format = Some(
            NumberFormat::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn valign_modifier(&mut self) -> Result<()> {
        self.modifier.vertical_align = Some(
            VerticalAlign::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn var_modifier(&mut self) -> Result<()> {
        todo!();
    }

    fn modifier(&mut self) -> Result<()> {
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
            _ => Err(Error::ModifierSyntaxError {
                bad_input: modifier_name.to_string(),
                index: A1::builder().xy(0, 0).build()?, // XXX
                message: format!("Unrecognized modifier: {}", &modifier_name),
            }),
        }
    }

    fn modifiers(&mut self) -> Result<()> {
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

    #[test]
    fn parse_no_modifier() {
        let default_modifier = Modifier::new(true);
        let parsed_modifiers = ModifierParser::parse(
            A1::builder().xy(0, 0).build().unwrap(),
            "abc123",
            default_modifier,
        ).unwrap();

        assert_eq!(parsed_modifiers.value, "abc123");

        assert!(!parsed_modifiers.modifier.row_level);
        assert!(parsed_modifiers.row_modifier.row_level);
    }

    #[test]
    fn parse_modifier() {
        let default_modifier = Modifier::new(true);
        let ParsedModifiers { 
            value,
            modifier,
            row_modifier: _row_modifier,
            index: _index,
        } = ModifierParser::parse(
            A1::builder().xy(0, 0).build().unwrap(),
            "[[format=bold]]abc123",
            default_modifier,
        ).unwrap();

        assert_eq!(value, "abc123");

        assert!(modifier.formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_multiple_modifiers() {
        let default_modifier = Modifier::new(true);
        let ParsedModifiers { 
            value,
            modifier,
            row_modifier: _row_modifier,
            index: _index,
        } = ModifierParser::parse(
            A1::builder().xy(0, 0).build().unwrap(),
            "[[format=italic/valign=top/expand]]abc123",
            default_modifier,
        ).unwrap();

        assert_eq!(value, "abc123");

        assert!(modifier.formats.contains(&TextFormat::Italic));
        assert_eq!(modifier.vertical_align, Some(VerticalAlign::Top));
        assert_eq!(modifier.expand, Some(Expand { amount: None }));
    }

    #[test]
    fn parse_multiple_modifiers_shorthand() {
        let default_modifier = Modifier::new(true);
        let ParsedModifiers { 
            value,
            modifier,
            row_modifier: _row_modifier,
            index: _index,
        } = ModifierParser::parse(
            A1::builder().xy(0, 0).build().unwrap(),
            "[[ha=l/va=c/f=u/fs=12]]abc123",
            default_modifier,
        ).unwrap();

        assert_eq!(value, "abc123");

        assert_eq!(modifier.font_size, Some(12));
        assert_eq!(modifier.horizontal_align, Some(HorizontalAlign::Left));
        assert_eq!(modifier.vertical_align, Some(VerticalAlign::Center));
        assert!(modifier.formats.contains(&TextFormat::Underline));
    }
}
