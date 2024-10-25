//! Module for lexing and parsing a cell and it's options.
//!
mod validate;

use super::cell_lexer::{CellLexer, Token, TokenMatch};
use crate::error::{BadInput, ParseResult, Result};
use crate::{
    ArcSourceCode, BorderSide, BorderStyle, Cell, Fill, HorizontalAlign, NumberFormat, Rgb, Row,
    TextFormat, VerticalAlign,
};

pub(crate) struct CellParser<'a, 'b: 'a> {
    cell: Cell,
    is_row_options: bool,
    /// We re-use the lexer in some contexts so take a reference to an existing one (with it's own
    /// lifetime)
    lexer: &'a mut CellLexer<'b>,
    row: &'a mut Row,
    row_a1: a1::Address,
}

macro_rules! assign_option {
    ($self:ident, $attr:ident, $value:expr) => {{
        if $self.is_row_options {
            $self.row.$attr = Some($value);
        } else {
            $self.cell.$attr = Some($value);
        }
        Ok(())
    }};
}

macro_rules! insert_option {
    ($self:ident, $attr:ident, $value:expr) => {{
        if $self.is_row_options {
            $self.row.$attr.insert($value);
        } else {
            $self.cell.$attr.insert($value);
        }
        Ok(())
    }};
}

impl<'a, 'b> CellParser<'a, 'b>
where
    // we'll instantiate multiple parsers but the lexer will be re-used amongst them. so it's
    // important that it has it's own lifetime which is longer
    'b: 'a,
{
    pub(crate) fn parse(
        input: &'b str,
        position: a1::Address,
        row: &'b mut Row,
        source_code: ArcSourceCode,
    ) -> Result<Cell> {
        Self::parse_cell(input, position, row, source_code.clone())
            .map_err(move |e| source_code.cell_syntax_error(e, position))
    }

    fn parse_cell(
        input: &'b str,
        position: a1::Address,
        row: &'b mut Row,
        source_code: ArcSourceCode,
    ) -> ParseResult<Cell> {
        let mut lexer = CellLexer::new(input, position, source_code);
        let mut parsed_cell = None;

        while let Some(start_token) = lexer.maybe_take_start_options() {
            let is_row_options = start_token.token == Token::StartRowOptions;
            // they can only specify a `![[` on the first cell
            if is_row_options && position.column.x != 0 {
                return Err(start_token
                    .into_parse_error("You can only define a row options on the first cell"));
            }

            // you can only do a `[[`/`]]` once per-cell
            if !is_row_options && parsed_cell.is_some() {
                return Err(
                    start_token.into_parse_error("You can only define options once per cell")
                );
            };

            let mut parser = CellParser {
                cell: Cell::default_from(row.clone()),
                is_row_options,
                lexer: &mut lexer,
                row,
                row_a1: position,
            };
            parser.parse_cell_options()?;

            if !is_row_options {
                parsed_cell = Some(parser.cell);
            }
        }

        let mut cell = parsed_cell.unwrap_or_else(|| Cell::default_from(row.clone()));
        cell.value = lexer.rest().to_string();

        Ok(cell)
    }

    fn border_option(&mut self) -> ParseResult<()> {
        insert_option!(self, borders, {
            BorderSide::try_from(self.lexer.take_option_right_side()?)?
        })
    }

    fn border_color_option(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_option!(self, border_color, {
            Rgb::try_from(self.lexer.take_token(Token::Color)?)?
        })
    }

    fn border_style_option(&mut self) -> ParseResult<()> {
        assign_option!(self, border_style, {
            BorderStyle::try_from(self.lexer.take_option_right_side()?)?
        })
    }

    fn color_option(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_option!(self, color, {
            Rgb::try_from(self.lexer.take_token(Token::Color)?)?
        })
    }

    fn fill_option(&mut self, option: TokenMatch) -> ParseResult<()> {
        if !self.is_row_options {
            return Err(option.into_parse_error("`fill` can only be used in a `![[..]]`"));
        }

        let amount = if self.lexer.maybe_take_equals().is_some() {
            let amount_string = self.lexer.take_token(Token::PositiveNumber)?;
            Some(amount_string.str_match.parse::<usize>().map_err(|e| {
                amount_string.into_parse_error(format!("Error parsing fill= repetitions: {e}"))
            })?)
        } else {
            None
        };

        self.row.fill = Some(Fill::new(self.row_a1, amount));

        Ok(())
    }

    fn font_color_option(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_option!(self, font_color, {
            Rgb::try_from(self.lexer.take_token(Token::Color)?)?
        })
    }

    fn font_family_option(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_option!(self, font_family, {
            self.lexer.take_token(Token::String)?.str_match
        })
    }

    fn font_size_option(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_option!(self, font_size, {
            let font_size_match = self.lexer.take_token(Token::PositiveNumber)?;
            font_size_match.str_match.parse::<u8>().map_err(|e| {
                font_size_match.into_parse_error(format!("Error parsing fontsize: {e}"))
            })?
        })
    }

    fn halign_option(&mut self) -> ParseResult<()> {
        assign_option!(self, horizontal_align, {
            HorizontalAlign::try_from(self.lexer.take_option_right_side()?)?
        })
    }

    #[allow(clippy::unnecessary_wraps)]
    fn lock(&mut self) -> ParseResult<()> {
        if self.is_row_options {
            self.row.lock = true;
        } else {
            self.cell.lock = true;
        }

        Ok(())
    }

    fn note(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;
        assign_option!(self, note, {
            self.lexer.take_token(Token::String)?.str_match
        })
    }

    fn number_format(&mut self) -> ParseResult<()> {
        assign_option!(self, number_format, {
            NumberFormat::try_from(self.lexer.take_option_right_side()?)?
        })
    }

    fn text_option(&mut self) -> ParseResult<()> {
        insert_option!(self, text_formats, {
            TextFormat::try_from(self.lexer.take_option_right_side()?)?
        })
    }

    fn valign_option(&mut self) -> ParseResult<()> {
        assign_option!(self, vertical_align, {
            VerticalAlign::try_from(self.lexer.take_option_right_side()?)?
        })
    }

    fn var_option(&mut self) -> ParseResult<()> {
        assign_option!(self, var, {
            self.lexer.take_option_right_side()?.str_match
        })
    }

    fn option(&mut self) -> ParseResult<()> {
        let option_name = self.lexer.take_token(Token::OptionName)?;
        match option_name.str_match.as_str() {
            "b" | "border" => self.border_option(),
            "bc" | "bordercolor" => self.border_color_option(),
            "bs" | "borderstyle" => self.border_style_option(),
            "c" | "color" => self.color_option(),
            "dv" | "validate" => self.validate(),
            "f" | "fill" => self.fill_option(option_name),
            "fc" | "fontcolor" => self.font_color_option(),
            "ff" | "fontfamily" => self.font_family_option(),
            "fs" | "fontsize" => self.font_size_option(),
            "ha" | "halign" => self.halign_option(),
            "l" | "lock" => self.lock(),
            "n" | "note" => self.note(),
            "nf" | "numberformat" => self.number_format(),
            "t" | "text" => self.text_option(),
            "v" | "var" => self.var_option(),
            "va" | "valign" => self.valign_option(),
            _ => Err(option_name.into_parse_error("Expected a valid cell option")),
        }
    }

    fn parse_cell_options(&mut self) -> ParseResult<()> {
        loop {
            self.option()?;
            if self.lexer.maybe_take_slash().is_none() {
                break;
            }
        }

        self.lexer.take_token(Token::EndOptions)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn test_parse(input: &str, row: &mut Row) -> Cell {
        CellParser::parse(input, a1::Address::new(0, 0), row, build_source_code()).unwrap()
    }

    #[test]
    fn parse_no_option() {
        let cell = test_parse("abc123", &mut Row::default());
        assert_eq!(cell.value, "abc123");
    }

    #[test]
    fn parse_option() {
        let cell = test_parse("[[text=bold]]abc123", &mut Row::default());
        assert_eq!(cell.value, "abc123");
        assert!(cell.text_formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_multiple_options() {
        let mut row = Row::default();
        let cell = test_parse("![[text=italic/valign=top/fill]]abc123", &mut row);

        assert_eq!(cell.value, "abc123");

        assert!(row.text_formats.contains(&TextFormat::Italic));
        assert_eq!(row.vertical_align, Some(VerticalAlign::Top));
        assert_eq!(
            row.fill,
            Some(Fill {
                amount: None,
                start_row: 0.into()
            })
        );
    }

    #[test]
    fn parse_cell_options_inherit_from_row() {
        let mut row = Row::default();
        let cell = test_parse("![[t=u]]abc123", &mut row);

        assert!(row.text_formats.contains(&TextFormat::Underline));
        assert!(cell.text_formats.contains(&TextFormat::Underline));
    }

    #[test]
    fn parse_multiple_options_shorthand() {
        let cell = test_parse("[[ha=l/va=c/t=u/fs=12]]abc123", &mut Row::default());

        assert_eq!(cell.value, "abc123");
        assert_eq!(cell.font_size, Some(12));
        assert_eq!(cell.horizontal_align, Some(HorizontalAlign::Left));
        assert_eq!(cell.vertical_align, Some(VerticalAlign::Center));
        assert!(cell.text_formats.contains(&TextFormat::Underline));
    }

    #[test]
    fn parse_row_option() {
        let mut row = Row::default();
        test_parse("![[text=bold]]abc123", &mut row);

        assert!(row.text_formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_border() {
        let cell = test_parse("[[border=top]]abc123", &mut Row::default());
        assert!(cell.borders.contains(&BorderSide::Top));
    }

    #[test]
    fn parse_borderstyle() {
        let cell = test_parse("[[b=t/bs=dotted]]abc123", &mut Row::default());
        assert_eq!(cell.border_style, Some(BorderStyle::Dotted));
    }

    #[test]
    fn parse_color() {
        let cell = test_parse("[[color=#ABC]]abc123", &mut Row::default());
        assert!(cell.color.is_some());
    }

    #[test]
    fn parse_fill() {
        let mut row = Row::default();
        test_parse("![[fill=20]]abc123", &mut row);

        assert!(row.fill.is_some());
    }

    #[test]
    fn parse_fontcolor() {
        let cell = test_parse("[[fontcolor=#ABC]]abc123", &mut Row::default());
        assert!(cell.font_color.is_some());
    }

    #[test]
    fn parse_fontfamily() {
        let cell = test_parse("[[fontfamily=Helvetica]]abc123", &mut Row::default());
        assert_eq!(cell.font_family, Some("Helvetica".to_string()));
    }

    #[test]
    fn parse_fontsize() {
        let cell = test_parse("[[fontsize=20]]abc123", &mut Row::default());
        assert_eq!(cell.font_size, Some(20));
    }

    #[test]
    fn parse_halign() {
        let cell = test_parse("[[halign=left]]abc123", &mut Row::default());
        assert_eq!(cell.horizontal_align, Some(HorizontalAlign::Left));
    }

    #[test]
    fn parse_lock() {
        let cell = test_parse("[[lock]]abc123", &mut Row::default());
        assert!(cell.lock);
    }

    #[test]
    fn parse_note() {
        let cell = test_parse("[[note='foo']]abc123", &mut Row::default());
        assert_eq!(cell.note, Some("foo".to_string()));
    }

    #[test]
    fn parse_numberformat() {
        let cell = test_parse("[[numberformat=datetime]]abc123", &mut Row::default());
        assert_eq!(cell.number_format, Some(NumberFormat::DateTime));
    }

    #[test]
    fn parse_text() {
        let cell = test_parse("[[text=bold]]abc123", &mut Row::default());
        assert!(cell.text_formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_valign() {
        let cell = test_parse("[[valign=top]]abc123", &mut Row::default());
        assert_eq!(cell.vertical_align, Some(VerticalAlign::Top));
    }

    #[test]
    fn parse_var_option() {
        let cell = test_parse("[[var=foo]]abc123", &mut Row::default());
        assert_eq!(cell.var, Some("foo".to_string()));
    }
}
