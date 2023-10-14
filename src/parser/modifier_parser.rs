//! Module for lexing and parsing the modifiers on a cell.
//!
//! TODO:
//! * need to lowercase the input but we can't do it on the entire value because we don't want to
//!     lowercase the stuff outside the modifier definition
//!
use super::modifier_lexer::{ModifierLexer, Token, TokenMatch};
use crate::error::ModifierParseError;
use crate::modifier::*;
use crate::{Expand, ParseResult, Result, Rgb, Runtime};
use a1_notation::{Address, Row};

pub(crate) struct ModifierParser<'a, 'b> {
    /// We re-use the lexer in some contexts so take a reference to an existing one (with it's own
    /// lifetime)
    lexer: &'a mut ModifierLexer<'b>,

    /// While `Modifier` and `RowModifier` are two separate structs, parsing-wise the logic is
    /// the same. And since `RowModifier` is a superset of `Modifier`, we use the former for
    /// parsing into then cast it via `into()` if it's really a `Modifier`.
    modifier: &'a mut RowModifier,

    runtime: &'a Runtime,

    is_row_modifier: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedCell {
    pub(crate) modifier: Option<Modifier>,
    pub(crate) row_modifier: Option<RowModifier>,
    pub(crate) value: String,
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
                return Err(runtime.source_code.parse_error(
                    start_token,
                    "You can only define a row modifier in the first cell",
                ));
            }

            let mut row_modifier = new_row_modifier
                .clone()
                .unwrap_or_else(|| row_modifier.clone());

            ModifierParser {
                lexer: &mut lexer,
                modifier: &mut row_modifier,
                runtime,
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
            value: lexer.rest(),
        })
    }

    fn border_modifier(&mut self) -> ParseResult<()> {
        self.modifier.borders.insert(
            BorderSide::try_from(self.lexer.take_modifier_right_side()?)
                .map_err(|e| e.into_parse_error(&self.runtime.source_code))?,
        );

        Ok(())
    }

    fn border_color_modifier(&mut self) -> ParseResult<()> {
        self.equals()?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.border_color = Some(
            Rgb::try_from(color)
                .map_err(|e| e.into_parse_error("bordercolor", &self.runtime.source_code))?,
        );

        Ok(())
    }

    fn border_style_modifier(&mut self) -> ParseResult<()> {
        self.modifier.border_style = Some(
            BorderStyle::try_from(self.lexer.take_modifier_right_side()?)
                .map_err(|e| e.into_parse_error(&self.runtime.source_code))?,
        );
        Ok(())
    }

    fn color_modifier(&mut self) -> ParseResult<()> {
        self.equals()?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.color = Some(
            Rgb::try_from(color)
                .map_err(|e| e.into_parse_error("color", &self.runtime.source_code))?,
        );

        Ok(())
    }

    fn data_validate(&mut self) -> ParseResult<()> {
        self.equals()?;

        let dv_name = self.lexer.take_modifier_right_side()?;
        match dv_name.str_match.to_lowercase().as_str() {
            "custom" => self.data_validate_custom()?,
            "date_after" => self.data_validate_date_after()?,
            "date_before" => self.data_validate_date_before()?,
            "date_between" => self.data_validate_date_between()?,
            "date_equal_to" => self.data_validate_date_equal_to()?,
            "date_is_valid" => self.data_validate_date_is_valid()?,
            "date_not_between" => self.data_validate_date_not_between()?,
            "date_on_or_after" => self.data_validate_date_on_or_after()?,
            "date_on_or_before" => self.data_validate_date_on_or_before()?,
            "number_between" => self.data_validate_number_between()?,
            "number_equal_to" => self.data_validate_number_equal_to()?,
            "number_greater_than_or_equal_to" => {
                self.data_validate_number_greater_than_or_equal_to()?
            }
            "number_greater_than" => self.data_validate_number_greater_than()?,
            "number_less_than_or_equal_to" => self.data_validate_number_less_than_or_equal_to()?,
            "number_less_than" => self.data_validate_number_less_than()?,
            "number_not_between" => self.data_validate_number_not_between()?,
            "number_not_equal_to" => self.data_validate_number_not_equal_to()?,
            "text_contains" => self.data_validate_text_contains()?,
            "text_does_not_contain" => self.data_validate_text_does_not_contain()?,
            "text_equal_to" => self.data_validate_text_equal_to()?,
            "text_is_valid_email" => self.data_validate_text_is_valid_email()?,
            "text_is_valid_url" => self.data_validate_text_is_valid_url()?,
            "value_in_list" => self.data_validate_value_in_list()?,
            "value_in_range" => self.data_validate_value_in_range()?,
            _ => {
                return Err(ModifierParseError::new(
                    "validate",
                    dv_name,
                    &[
                        "custom(FORMULA)",
                        "date_after(DATE)",
                        "date_before(DATE)",
                        "date_between(DATE, DATE)",
                        "date_equal_to(DATE)",
                        "date_is_valid",
                        "date_not_between(DATE, DATE)",
                        "date_on_or_after(DATE)",
                        "date_on_or_before(DATE)",
                        "number_between(NUMBER, NUMBER)",
                        "number_equal_to(NUMBER)",
                        "number_greater_than_or_equal_to(NUMBER)",
                        "number_greater_than(NUMBER)",
                        "number_less_than_or_equal_to(NUMBER)",
                        "number_less_than(NUMBER)",
                        "number_not_between(NUMBER, NUMBER)",
                        "number_not_equal_to(NUMBER)",
                        "text_contains(TEXT)",
                        "text_does_not_contain(TEXT)",
                        "text_equal_to(TEXT)",
                        "text_is_valid_email",
                        "text_is_valid_url",
                        "value_in_list(ANY, ...)",
                        "value_in_range(A1)",
                    ],
                )
                .into_parse_error(&self.runtime.source_code));
            }
        }

        Ok(())
    }

    fn data_validate_custom(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_date_after(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_date_before(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_date_between(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_date_equal_to(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_date_is_valid(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::DateIsValid);
        Ok(())
    }

    fn data_validate_date_not_between(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_date_on_or_after(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_date_on_or_before(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_number_between(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_number_equal_to(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_number_greater_than(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_number_greater_than_or_equal_to(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_number_not_between(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_number_not_equal_to(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_number_less_than(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_number_less_than_or_equal_to(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_text_contains(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_text_does_not_contain(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_text_equal_to(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_text_is_valid_email(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::TextIsValidEmail);
        Ok(())
    }

    fn data_validate_text_is_valid_url(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::TextIsValidUrl);
        Ok(())
    }

    fn data_validate_value_in_list(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_value_in_range(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn equals(&mut self) -> ParseResult<()> {
        self.lexer.take_whitespace();
        self.lexer.take_token(Token::Equals)?;
        self.lexer.take_whitespace();
        Ok(())
    }

    fn expand_modifier(&mut self, modifier: TokenMatch, row: a1_notation::Row) -> ParseResult<()> {
        if !self.is_row_modifier {
            return Err(self.runtime.source_code.parse_error(
                modifier,
                "`expand` modifiers can only be used in a `![[..]]`",
            ));
        }

        let amount = if self.lexer.maybe_take_equals().is_some() {
            let amount_string = self.lexer.take_token(Token::PositiveNumber)?;

            match amount_string.str_match.parse::<usize>() {
                Ok(n) => Some(n),
                Err(e) => {
                    return Err(self.runtime.source_code.parse_error(
                        amount_string,
                        &format!("Error parsing expand= repetitions: {e}"),
                    ))
                }
            }
        } else {
            None
        };

        self.modifier.expand = Some(Expand {
            amount,
            start_row: row,
        });

        Ok(())
    }

    fn font_color_modifier(&mut self) -> ParseResult<()> {
        self.equals()?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.font_color = Some(
            Rgb::try_from(color)
                .map_err(|e| e.into_parse_error("fontcolor", &self.runtime.source_code))?,
        );
        Ok(())
    }

    fn font_family_modifier(&mut self) -> ParseResult<()> {
        self.equals()?;

        let font_family = self.lexer.take_token(Token::String)?;
        self.modifier.font_family = Some(font_family.str_match);
        Ok(())
    }

    fn font_size_modifier(&mut self) -> ParseResult<()> {
        self.equals()?;

        let font_size_string = self.lexer.take_token(Token::PositiveNumber)?;
        match font_size_string.str_match.parse::<u8>() {
            Ok(n) => self.modifier.font_size = Some(n),
            Err(e) => {
                return Err(self
                    .runtime
                    .source_code
                    .parse_error(font_size_string, &format!("Error parsing fontsize: {e}")))
            }
        }
        Ok(())
    }

    fn format_modifier(&mut self) -> ParseResult<()> {
        self.modifier.formats.insert(
            TextFormat::try_from(self.lexer.take_modifier_right_side()?)
                .map_err(|e| e.into_parse_error(&self.runtime.source_code))?,
        );
        Ok(())
    }

    fn halign_modifier(&mut self) -> ParseResult<()> {
        self.modifier.horizontal_align = Some(
            HorizontalAlign::try_from(self.lexer.take_modifier_right_side()?)
                .map_err(|e| e.into_parse_error(&self.runtime.source_code))?,
        );
        Ok(())
    }

    fn lock(&mut self) -> ParseResult<()> {
        self.modifier.lock = true;
        Ok(())
    }

    fn note(&mut self) -> ParseResult<()> {
        self.equals()?;
        self.modifier.note = Some(self.lexer.take_token(Token::String)?.str_match);
        Ok(())
    }

    fn number_format(&mut self) -> ParseResult<()> {
        self.modifier.number_format = Some(
            NumberFormat::try_from(self.lexer.take_modifier_right_side()?)
                .map_err(|e| e.into_parse_error(&self.runtime.source_code))?,
        );
        Ok(())
    }

    fn valign_modifier(&mut self) -> ParseResult<()> {
        self.modifier.vertical_align = Some(
            VerticalAlign::try_from(self.lexer.take_modifier_right_side()?)
                .map_err(|e| e.into_parse_error(&self.runtime.source_code))?,
        );
        Ok(())
    }

    fn var_modifier(&mut self) -> ParseResult<()> {
        self.modifier.var = Some(self.lexer.take_modifier_right_side()?.str_match);
        Ok(())
    }

    fn modifier(&mut self, row: Row) -> ParseResult<()> {
        let modifier_name = self.lexer.take_token(Token::ModifierName)?;
        match modifier_name.str_match.as_str() {
            "b" | "border" => self.border_modifier(),
            "bc" | "bordercolor" => self.border_color_modifier(),
            "bs" | "borderstyle" => self.border_style_modifier(),
            "c" | "color" => self.color_modifier(),
            "dv" | "validate" => self.data_validate(),
            "e" | "expand" => self.expand_modifier(modifier_name, row),
            "f" | "format" => self.format_modifier(),
            "fc" | "fontcolor" => self.font_color_modifier(),
            "ff" | "fontfamily" => self.font_family_modifier(),
            "fs" | "fontsize" => self.font_size_modifier(),
            "ha" | "halign" => self.halign_modifier(),
            "l" | "lock" => self.lock(),
            "n" | "note" => self.note(),
            "nf" | "numberformat" => self.number_format(),
            "v" | "var" => self.var_modifier(),
            "va" | "valign" => self.valign_modifier(),
            _ => Err(self
                .runtime
                .source_code
                .parse_error(modifier_name, "Unrecognized modifier")),
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
        let runtime: Runtime = TestFile::new("xlsx", input).into();
        ModifierParser::parse(input, Address::new(0, 0), &RowModifier::default(), &runtime).unwrap()
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
        } = test_parse("[[format=bold]]abc123");

        assert_eq!(value, "abc123");
        assert!(modifier.unwrap().formats.contains(&TextFormat::Bold));
    }

    #[test]
    fn parse_multiple_modifiers() {
        let ParsedCell {
            value,
            row_modifier,
            ..
        } = test_parse("![[format=italic/valign=top/expand]]abc123");

        assert_eq!(value, "abc123");

        let m = row_modifier.unwrap();
        assert!(m.formats.contains(&TextFormat::Italic));
        assert_eq!(m.vertical_align, Some(VerticalAlign::Top));
        assert_eq!(
            m.expand,
            Some(Expand {
                amount: None,
                start_row: 0.into()
            })
        );
    }

    #[test]
    fn parse_multiple_modifiers_shorthand() {
        let ParsedCell {
            value, modifier, ..
        } = test_parse("[[ha=l/va=c/f=u/fs=12]]abc123");

        assert_eq!(value, "abc123");

        let m = modifier.unwrap();
        assert_eq!(m.font_size, Some(12));
        assert_eq!(m.horizontal_align, Some(HorizontalAlign::Left));
        assert_eq!(m.vertical_align, Some(VerticalAlign::Center));
        assert!(m.formats.contains(&TextFormat::Underline));
    }

    #[test]
    fn parse_row_modifier() {
        let ParsedCell { row_modifier, .. } = test_parse("![[format=bold]]abc123");
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
    fn parse_expand() {
        let ParsedCell { row_modifier, .. } = test_parse("![[expand=20]]abc123");
        assert!(row_modifier.unwrap().expand.is_some());
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
    fn parse_format() {
        let ParsedCell { modifier, .. } = test_parse("[[format=bold]]abc123");
        assert!(modifier.unwrap().formats.contains(&TextFormat::Bold));
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
