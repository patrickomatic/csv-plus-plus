//! Module for lexing and parsing the modifiers on a cell.
//!
use super::modifier_lexer::{ModifierLexer, Token, TokenMatch};
use crate::error::{BadInput, ParseResult, Result};
use crate::modifier::*;
use crate::{Fill, Rgb, Runtime};
use a1_notation::{Address, Row};

mod validate;

pub(crate) struct ModifierParser<'a, 'b: 'a> {
    /// We re-use the lexer in some contexts so take a reference to an existing one (with it's own
    /// lifetime)
    lexer: &'a mut ModifierLexer<'b>,

    /// While `Modifier` and `RowModifier` are two separate structs, parsing-wise the logic is
    /// the same. And since `RowModifier` is a superset of `Modifier`, we use the former for
    /// parsing into then cast it via `into()` if it's really a `Modifier`.
    modifier: &'a mut RowModifier,

    is_row_modifier: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedCell {
    pub(crate) modifier: Option<Modifier>,
    pub(crate) row_modifier: Option<RowModifier>,
    pub(crate) value: String,
}

macro_rules! assign_modifier {
    ($self:ident, $modifier:ident, $value:tt) => {{
        $self.modifier.$modifier = Some($value);
        Ok(())
    }};
}

macro_rules! insert_modifier {
    ($self:ident, $modifier:ident, $value:tt) => {{
        $self.modifier.$modifier.insert($value);
        Ok(())
    }};
}

impl<'a, 'b> ModifierParser<'a, 'b>
where
    // we'll instantiate multiple parsers but the lexer will be re-used amongst them. so it's
    // important that it has it's own lifetime which is longer
    'b: 'a,
{
    pub(crate) fn parse(
        input: &'b str,
        position: Address,
        row_modifier: &'b RowModifier,
        runtime: &'b Runtime,
    ) -> Result<ParsedCell> {
        Self::parse_all_modifiers(input, position, row_modifier, runtime)
            .map_err(move |e| runtime.source_code.modifier_syntax_error(e, position))
    }

    fn parse_all_modifiers(
        input: &'b str,
        position: Address,
        row_modifier: &'b RowModifier,
        runtime: &'b Runtime,
    ) -> ParseResult<ParsedCell> {
        let mut lexer = ModifierLexer::new(input, position, runtime);
        let mut new_modifier: Option<Modifier> = None;
        let mut new_row_modifier: Option<RowModifier> = None;

        while let Some(start_token) = lexer.maybe_take_start_modifier() {
            let is_row_modifier = start_token.token == Token::StartRowModifier;
            if is_row_modifier && position.column.x != 0 {
                return Err(start_token
                    .into_parse_error("You can only define a row modifier in the first cell"));
            }

            let mut row_modifier = new_row_modifier
                .clone()
                .unwrap_or_else(|| row_modifier.clone());

            ModifierParser {
                lexer: &mut lexer,
                modifier: &mut row_modifier,
                is_row_modifier,
            }
            .parse_modifiers(position.row)?;

            if is_row_modifier {
                new_row_modifier = Some(row_modifier)
            } else {
                new_modifier = Some(row_modifier.into())
            }
        }

        Ok(ParsedCell {
            modifier: new_modifier,
            row_modifier: new_row_modifier,
            value: lexer.rest().to_string(),
        })
    }

    fn border_modifier(&mut self) -> ParseResult<()> {
        insert_modifier!(self, borders, {
            BorderSide::try_from(self.lexer.take_modifier_right_side()?)?
        })
    }

    fn border_color_modifier(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_modifier!(self, border_color, {
            Rgb::try_from(self.lexer.take_token(Token::Color)?)?
        })
    }

    fn border_style_modifier(&mut self) -> ParseResult<()> {
        assign_modifier!(self, border_style, {
            BorderStyle::try_from(self.lexer.take_modifier_right_side()?)?
        })
    }

    fn color_modifier(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_modifier!(self, color, {
            Rgb::try_from(self.lexer.take_token(Token::Color)?)?
        })
    }

    fn fill_modifier(&mut self, modifier: TokenMatch, row: a1_notation::Row) -> ParseResult<()> {
        if !self.is_row_modifier {
            return Err(
                modifier.into_parse_error("`fill` modifiers can only be used in a `![[..]]`")
            );
        }

        assign_modifier!(self, fill, {
            let amount = if self.lexer.maybe_take_equals().is_some() {
                let amount_string = self.lexer.take_token(Token::PositiveNumber)?;
                Some(amount_string.str_match.parse::<usize>().map_err(|e| {
                    amount_string.into_parse_error(format!("Error parsing fill= repetitions: {e}"))
                })?)
            } else {
                None
            };

            Fill::new(row, amount)
        })
    }

    fn font_color_modifier(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_modifier!(self, font_color, {
            Rgb::try_from(self.lexer.take_token(Token::Color)?)?
        })
    }

    fn font_family_modifier(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_modifier!(self, font_family, {
            self.lexer.take_token(Token::String)?.str_match
        })
    }

    fn font_size_modifier(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_modifier!(self, font_size, {
            let font_size_match = self.lexer.take_token(Token::PositiveNumber)?;
            font_size_match.str_match.parse::<u8>().map_err(|e| {
                font_size_match.into_parse_error(format!("Error parsing fontsize: {e}"))
            })?
        })
    }

    fn halign_modifier(&mut self) -> ParseResult<()> {
        assign_modifier!(self, horizontal_align, {
            HorizontalAlign::try_from(self.lexer.take_modifier_right_side()?)?
        })
    }

    fn lock(&mut self) -> ParseResult<()> {
        self.modifier.lock = true;
        Ok(())
    }

    fn note(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_modifier!(self, note, {
            self.lexer.take_token(Token::String)?.str_match
        })
    }

    fn number_format(&mut self) -> ParseResult<()> {
        assign_modifier!(self, number_format, {
            NumberFormat::try_from(self.lexer.take_modifier_right_side()?)?
        })
    }

    fn text_modifier(&mut self) -> ParseResult<()> {
        insert_modifier!(self, formats, {
            TextFormat::try_from(self.lexer.take_modifier_right_side()?)?
        })
    }

    fn valign_modifier(&mut self) -> ParseResult<()> {
        assign_modifier!(self, vertical_align, {
            VerticalAlign::try_from(self.lexer.take_modifier_right_side()?)?
        })
    }

    fn var_modifier(&mut self) -> ParseResult<()> {
        assign_modifier!(self, var, {
            self.lexer.take_modifier_right_side()?.str_match
        })
    }

    fn modifier(&mut self, row: Row) -> ParseResult<()> {
        let modifier_name = self.lexer.take_token(Token::ModifierName)?;
        match modifier_name.str_match.as_str() {
            "b" | "border" => self.border_modifier(),
            "bc" | "bordercolor" => self.border_color_modifier(),
            "bs" | "borderstyle" => self.border_style_modifier(),
            "c" | "color" => self.color_modifier(),
            "dv" | "validate" => self.validate(),
            "f" | "fill" => self.fill_modifier(modifier_name, row),
            "fc" | "fontcolor" => self.font_color_modifier(),
            "ff" | "fontfamily" => self.font_family_modifier(),
            "fs" | "fontsize" => self.font_size_modifier(),
            "ha" | "halign" => self.halign_modifier(),
            "l" | "lock" => self.lock(),
            "n" | "note" => self.note(),
            "nf" | "numberformat" => self.number_format(),
            "t" | "text" => self.text_modifier(),
            "v" | "var" => self.var_modifier(),
            "va" | "valign" => self.valign_modifier(),
            _ => Err(modifier_name.into_parse_error("Expected a modifier")),
        }
    }

    fn parse_modifiers(&mut self, row: Row) -> ParseResult<()> {
        loop {
            self.modifier(row)?;
            if self.lexer.maybe_take_slash().is_none() {
                break;
            }
        }

        self.lexer.take_token(Token::EndModifier)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn test_parse(input: &str) -> ParsedCell {
        ModifierParser::parse(
            input,
            Address::new(0, 0),
            &RowModifier::default(),
            &build_runtime(),
        )
        .unwrap()
    }

    #[test]
    fn parse_no_modifier() {
        let parsed_modifiers = test_parse("abc123");

        assert_eq!(parsed_modifiers.value, "abc123");
        assert!(parsed_modifiers.modifier.is_none());
        assert!(parsed_modifiers.row_modifier.is_none());
    }

    #[test]
    fn parse_modifier() {
        let ParsedCell {
            value, modifier, ..
        } = test_parse("[[text=bold]]abc123");

        assert_eq!(value, "abc123");
        assert!(modifier.unwrap().formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_multiple_modifiers() {
        let ParsedCell {
            value,
            row_modifier,
            ..
        } = test_parse("![[text=italic/valign=top/fill]]abc123");

        assert_eq!(value, "abc123");

        let m = row_modifier.unwrap();
        assert!(m.formats.contains(&TextFormat::Italic));
        assert_eq!(m.vertical_align, Some(VerticalAlign::Top));
        assert_eq!(
            m.fill,
            Some(Fill {
                amount: None,
                start_row: 0.into()
            })
        );
    }

    #[test]
    fn parse_multiple_modifiers_shorthand() {
        let ParsedCell {
            value, modifier, ..
        } = test_parse("[[ha=l/va=c/t=u/fs=12]]abc123");

        assert_eq!(value, "abc123");

        let m = modifier.unwrap();
        assert_eq!(m.font_size, Some(12));
        assert_eq!(m.horizontal_align, Some(HorizontalAlign::Left));
        assert_eq!(m.vertical_align, Some(VerticalAlign::Center));
        assert!(m.formats.contains(&TextFormat::Underline));
    }

    #[test]
    fn parse_row_modifier() {
        let ParsedCell { row_modifier, .. } = test_parse("![[text=bold]]abc123");

        assert!(row_modifier.unwrap().formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_border() {
        let ParsedCell { modifier, .. } = test_parse("[[border=top]]abc123");

        assert!(modifier.unwrap().borders.contains(&BorderSide::Top));
    }

    #[test]
    fn parse_borderstyle() {
        let ParsedCell { modifier, .. } = test_parse("[[b=t/bs=dotted]]abc123");

        assert_eq!(modifier.unwrap().border_style, Some(BorderStyle::Dotted));
    }

    #[test]
    fn parse_color() {
        let ParsedCell { modifier, .. } = test_parse("[[color=#ABC]]abc123");

        assert!(modifier.unwrap().color.is_some());
    }

    #[test]
    fn parse_fill() {
        let ParsedCell { row_modifier, .. } = test_parse("![[fill=20]]abc123");

        assert!(row_modifier.unwrap().fill.is_some());
    }

    #[test]
    fn parse_fontcolor() {
        let ParsedCell { modifier, .. } = test_parse("[[fontcolor=#ABC]]abc123");

        assert!(modifier.unwrap().font_color.is_some());
    }

    #[test]
    fn parse_fontfamily() {
        let ParsedCell { modifier, .. } = test_parse("[[fontfamily=Helvetica]]abc123");

        assert_eq!(modifier.unwrap().font_family, Some("Helvetica".to_string()));
    }

    #[test]
    fn parse_fontsize() {
        let ParsedCell { modifier, .. } = test_parse("[[fontsize=20]]abc123");

        assert_eq!(modifier.unwrap().font_size, Some(20));
    }

    #[test]
    fn parse_halign() {
        let ParsedCell { modifier, .. } = test_parse("[[halign=left]]abc123");

        assert_eq!(
            modifier.unwrap().horizontal_align,
            Some(HorizontalAlign::Left)
        );
    }

    #[test]
    fn parse_lock() {
        let ParsedCell { modifier, .. } = test_parse("[[lock]]abc123");

        assert!(modifier.unwrap().lock);
    }

    #[test]
    fn parse_note() {
        let ParsedCell { modifier, .. } = test_parse("[[note='foo']]abc123");

        assert_eq!(modifier.unwrap().note, Some("foo".to_string()));
    }

    #[test]
    fn parse_numberformat() {
        let ParsedCell { modifier, .. } = test_parse("[[numberformat=datetime]]abc123");

        assert_eq!(
            modifier.unwrap().number_format,
            Some(NumberFormat::DateTime)
        );
    }

    #[test]
    fn parse_text() {
        let ParsedCell { modifier, .. } = test_parse("[[text=bold]]abc123");

        assert!(modifier.unwrap().formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_valign() {
        let ParsedCell { modifier, .. } = test_parse("[[valign=top]]abc123");

        assert_eq!(modifier.unwrap().vertical_align, Some(VerticalAlign::Top));
    }

    #[test]
    fn parse_var_modifier() {
        let ParsedCell { modifier, .. } = test_parse("[[var=foo]]abc123");

        assert_eq!(modifier.unwrap().var, Some("foo".to_string()));
    }
}
