//! Module for lexing and parsing the modifiers on a cell.
//!
//! TODO:
//! * fix the row parsing logic (it doesn't apply them)
//! * need to lowercase the input but we can't do it on the entire value because we don't want to
//!     lowercase the stuff outside the modifier definition
//! * get quoted strings working
use std::str::FromStr;

use crate::{Error, Position, Rgb};
use crate::modifier::*;

#[derive(PartialEq)]
pub enum Token {
    Color,
    EndModifier,
    Equals,
    ModifierName,
    ModifierRightSide,
    PositiveNumber,
    String,
    Slash,
    StartCellModifier,
    StartRowModifier,
}

pub struct ModifierLexer {
    input: String,
}

type LexerTakeResult = Result<String, String>;

/// # ModifierLexer
///
/// This is the lexer/tokenizer used for parsing csv++ modifiers - it's a little different than
/// most parsers which parse their entire input into tokens in one go - this tokenizes as the
/// parser goes since it is context-dependent.
///
/// [https://en.wikipedia.org/wiki/Lexer_hack](See also: Lexer hack)
impl ModifierLexer {
    fn new(input: String) -> Self {
        Self { input }
    }

    pub fn rest(&self) -> String {
        self.input.clone()
    }

    pub fn maybe_take_start_modifier(&mut self) -> Option<Token> {
        let input = self.input.trim();
        
        if input.starts_with("[[") {
            self.input = input[2..].to_string();
            Some(Token::StartCellModifier)
        } else if input.starts_with("![[") {
            self.input = input[3..].to_string();
            Some(Token::StartRowModifier)
        } else {
            None
        }
    }

    pub fn take_modifier_right_side(&mut self) -> LexerTakeResult {
        self.take_token(Token::Equals)?;
        self.take_token(Token::ModifierRightSide)
    }

    pub fn maybe_take_token(&mut self, token: Token) -> Option<String> {
        match token {
            Token::Equals =>            self.maybe_take("="),
            Token::Slash =>             self.maybe_take("/"),
            _ => todo!(), // TODO
        }
    }

    fn maybe_take<'a>(&mut self, substring: &'a str) -> Option<String> {
        let input = self.input.trim();

        if input.starts_with(substring) {
            self.input = input[substring.len()..].to_string();
            Some(substring.to_string())
        } else {
            None
        }
    }

    pub fn take_token(&mut self, token: Token) -> LexerTakeResult {
        match token {
            Token::Color =>             self.take_color(),
            Token::EndModifier =>       self.take("]]"),
            Token::Equals =>            self.take("="),
            Token::ModifierName =>      self.take_while(|ch| ch.is_alphanumeric()),
            Token::ModifierRightSide => self.take_while(|ch| ch.is_alphanumeric() || ch == '_'),
            Token::PositiveNumber =>    self.take_while(|ch| ch.is_ascii_digit()),
            Token::String =>           self.take_string(),
            Token::Slash =>             self.take("/"),
            Token::StartCellModifier => self.take("[["),
            Token::StartRowModifier =>  self.take("![["),
        }
    }

    fn take<'a>(&mut self, substring: &'a str) -> LexerTakeResult {
        let input = self.input.trim();

        if input.starts_with(substring) {
            self.input = input[substring.len()..].to_string();
            Ok(substring.to_string())
        } else {
            Err(format!("Error parsing input, expected '{}'", substring))
        }
    }

    fn take_color<'a>(&mut self) -> LexerTakeResult {
        Ok(String::from("#FFF"))
    }

    fn take_string<'a>(&mut self) -> LexerTakeResult {
        let input = self.input.trim();

        if input.starts_with("'") {
            todo!(); // XXX single quote parsing logic
        } else {
            let unquoted_string = self.take_while(|ch| ch.is_alphanumeric())?;
            Ok(unquoted_string.to_string())
        }
    }

    // TODO need to lowercase this
    fn take_while<'a, F>(
        &'a mut self, 
        while_fn: F,
    ) -> Result<String, String> 
    where F: Fn(char) -> bool {
        let input = self.input.trim();

        let mut matched = String::from("");
        for c in input.chars() {
            if while_fn(c) {
                matched.push(c);
            } else {
                break;
            }
        }

        if matched == "" {
            Err(String::from("Expected a modifier definition (i.e. format/halign/etc)"))
        } else {
            self.input = input[matched.len()..].to_string();
            Ok(matched)
        }
    }
}

struct ModifierParser<'a> {
    lexer: &'a mut ModifierLexer,
    modifier: &'a mut Modifier,
}

type ParseResult = Result<(), String>;

impl<'a> ModifierParser<'a> {
    fn border_modifier(&mut self) -> ParseResult {
        self.modifier.borders.insert(BorderSide::from_str(&self.lexer.take_modifier_right_side()?)?);
        Ok(())
    }

    fn border_color_modifier(&mut self) -> ParseResult {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.border_color = Some(Rgb::from_str(&color)?);
        Ok(())
    }

    fn border_style_modifier(&mut self) -> ParseResult {
        self.modifier.border_style = Some(
            BorderStyle::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn color_modifier(&mut self) -> ParseResult {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.color = Some(Rgb::from_str(&color)?);

        Ok(())
    }

    fn expand_modifier(&mut self) -> ParseResult {
        let amount = if let Some(_) = self.lexer.maybe_take_token(Token::Equals) {
            let amount_string = self.lexer.take_token(Token::PositiveNumber)?;
            
            match amount_string.parse::<usize>() {
                Ok(n) => Some(n),
                Err(e) => return Err(format!("Error parsing expand= repetitions: {}", e)),
            }
        } else {
            None
        };

        self.modifier.expand = Some(Expand { amount });

        Ok(())
    }

    fn font_color_modifier(&mut self) -> ParseResult {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.font_color = Some(Rgb::from_str(&color)?);
        Ok(())
    }

    fn font_family_modifier(&mut self) -> ParseResult {
        self.lexer.take_token(Token::Equals)?;

        let color = self.lexer.take_token(Token::Color)?;
        self.modifier.font_color = Some(Rgb::from_str(&color)?);
        Ok(())
    }

    fn font_size_modifier(&mut self) -> ParseResult {
        self.lexer.take_token(Token::Equals)?;

        let font_size_string = self.lexer.take_token(Token::PositiveNumber)?;
        match font_size_string.parse::<u8>() {
            Ok(n) => self.modifier.font_size = Some(n),
            Err(e) => return Err(format!("Error parsing fontsize: {}", e)),
        }

        Ok(())
    }

    fn format_modifier(&mut self) -> ParseResult {
        self.modifier.formats.insert(TextFormat::from_str(&self.lexer.take_modifier_right_side()?)?);
        Ok(())
    }

    fn halign_modifier(&mut self) -> ParseResult {
        self.modifier.horizontal_align = Some(
            HorizontalAlign::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn note(&mut self) -> ParseResult {
        self.modifier.note = Some(self.lexer.take_token(Token::String)?);
        Ok(())
    }

    fn number_format(&mut self) -> ParseResult {
        self.modifier.number_format = Some(
            NumberFormat::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn valign_modifier(&mut self) -> ParseResult {
        self.modifier.vertical_align = Some(
            VerticalAlign::from_str(&self.lexer.take_modifier_right_side()?)?
        );
        Ok(())
    }

    fn var_modifier(&mut self) -> ParseResult {
        todo!();
    }

    fn modifier(&mut self) -> ParseResult {
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
            _ => return Err(format!("Unrecognized modifier: {}", modifier_name))
        }
    }

    fn modifiers(&mut self) -> Result<(), String> {
        loop {
            self.modifier()?;

            if let None = self.lexer.maybe_take_token(Token::Slash) {
                break
            }
        }

        self.lexer.take_token(Token::EndModifier)?;

        Ok(())
    }
}

/// returns (modifier, row_modifier)
pub fn parse_all_modifiers(
    lexer: &mut ModifierLexer,
    default_from: &Modifier,
) -> Result<(Option<Modifier>, Option<Modifier>), String> {
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
                return Err("You can only define one row modifier for a cell".to_string())
            } 

            row_modifier = Some(new_modifier)
        } else {
            if modifier.is_some() {
                return Err("You can only define one modifier for a cell".to_string())
            }

            modifier = Some(new_modifier)
        }
    }

    Ok((modifier, row_modifier))
}

#[derive(Clone)]
pub struct ParsedModifiers {
    pub modifier: Modifier,
    pub row_modifier: Modifier,
    pub value: String,
    pub index: Position,
}

pub fn parse<'a>(
    index: Position, 
    input: String, 
    default_from: Modifier,
) -> Result<ParsedModifiers, Error> {
    let lexer = &mut ModifierLexer::new(input);

    match parse_all_modifiers(lexer, &default_from) {
        Ok((modifier, row_modifier)) => {
            if row_modifier != None && index.is_first_cell() {
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
                message, 
            })
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_modifier() {
        let default_modifier = Modifier::new(true);
        let parsed_modifiers = parse(Position(0, 0), "abc123".to_string(), default_modifier).unwrap();

        assert_eq!(parsed_modifiers.value, "abc123");

        assert_eq!(parsed_modifiers.modifier.row_level, false);
        assert_eq!(parsed_modifiers.row_modifier.row_level, true);
    }

    #[test]
    fn parse_modifier() {
        let default_modifier = Modifier::new(true);
        let ParsedModifiers { 
            value,
            modifier,
            row_modifier: _row_modifier,
            index: _index,
        } = parse(
            Position(0, 0), 
            String::from("[[format=bold]]abc123"),
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
        } = parse(
            Position(0, 0), 
            String::from("[[format=italic/valign=top/expand]]abc123"),
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
        } = parse(
            Position(0, 0), 
            String::from("[[ha=l/va=c/f=u/fs=12]]abc123"),
            default_modifier,
        ).unwrap();

        assert_eq!(value, "abc123");

        assert_eq!(modifier.font_size, Some(12));
        assert_eq!(modifier.horizontal_align, Some(HorizontalAlign::Left));
        assert_eq!(modifier.vertical_align, Some(VerticalAlign::Center));
        assert!(modifier.formats.contains(&TextFormat::Underline));
    }
}
