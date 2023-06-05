use lazy_static::lazy_static;
use rgb::RGB16;
// use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::error::CsvppError;
use crate::Position;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum BorderSide {
    All,
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
enum BorderStyle {
    Dashed,
    Dotted,
    Double,
    Solid,
    SolidMedium,
    SolidThick,
}

/// The possible values for aligning a cell horizontally.
#[derive(Clone, Debug, PartialEq)]
enum HorizontalAlign {
    Center,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
enum NumberFormat {
    Currency,
    Date,
    DateTime,
    Number,
    Percent,
    Text,
    Time,
    Scientific,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum TextFormat {
    Bold,
    Italic,
    Strikethrough,
    Underline,
}

/// The possible values for aligning a cell vertically.
#[derive(Clone, Debug, PartialEq)]
enum VerticalAlign {
    Bottom,
    Center,
    Top,
}

#[derive(Clone, Debug, PartialEq)]
struct Expand {
    amount: Option<u16>,
}

/// # Modifier
#[derive(Clone, Debug, PartialEq)]
pub struct Modifier {
    border_color: Option<RGB16>,
    border_style: Option<BorderStyle>,
    borders: HashSet<BorderSide>,
    color: Option<RGB16>,
    expand: Option<Expand>,
    font_color: Option<RGB16>,
    font_family: Option<String>,
    font_size: Option<u8>,
    formats: HashSet<TextFormat>,
    horizontal_align: Option<HorizontalAlign>,
    note: Option<String>,
    number_format: Option<NumberFormat>,
    row_level: bool,
    var: Option<String>,
    vertical_align: Option<VerticalAlign>,
}

impl Default for Modifier {
    fn default() -> Self {
        // XXX maybe make this lazy static?
        Self {
            border_color: None,
            border_style: None,
            borders: HashSet::new(),
            color: None,
            expand: None,
            font_color: None,
            font_family: None,
            font_size: None,
            formats: HashSet::new(),
            horizontal_align: None,
            note: None,
            number_format: None,
            row_level: false,
            var: None,
            vertical_align: None,
        }
    }
}

#[derive(Clone)]
pub struct ParsedModifiers {
    pub modifier: Modifier,
    pub row_modifier: Modifier,
    pub value: String,
}

// TODO: maybe make this plural?
impl Modifier {
    pub fn new(row_level: bool) -> Self {
        let default = Self::default();
        Self {
            row_level,
            ..default
        }
    }

    pub fn from(modifier: &Modifier) -> Self {
        Self {
            row_level: false,
            ..modifier.clone()
        }
    }
}

fn parse_format(_input: &str, mut modifier: Modifier) -> Result<Modifier, CsvppError> {
    modifier.formats.insert(TextFormat::Bold);
    Ok(modifier)
}

type ModifierParseFn = &'static (dyn Fn(&str, Modifier) -> Result<Modifier, CsvppError> + Sync);

lazy_static! {
    static ref MODIFIER_DISPATCH: HashMap<&'static str, ModifierParseFn> = {
        let mut m: HashMap<&'static str, ModifierParseFn> = HashMap::new();
        m.insert("f", &parse_format);
        m.insert("format", &parse_format);
        m
    };
}

#[derive(PartialEq)]
pub enum Token {
    EndModifier,
    Equals,
    ModifierName,
    Slash,
    StartCellModifier,
    StartRowModifier,
}

pub struct ModifierLexer {
    pub input: String,
    // pub chars: std::str::Chars<'a>,
}

/// # ModifierLexer
///
/// This is the lexer/tokenizer used for parsing csv++ modifiers - it's a little different than
/// most parsers which parse their entire input into tokens in one go - this tokenizes as the
/// parser goes since it is context-dependent.
///
/// [https://en.wikipedia.org/wiki/Lexer_hack](See also: Lexer hack)
impl ModifierLexer {
    fn new(input: String) -> ModifierLexer {
        ModifierLexer { input }
    }

    /*
    pub fn take_token(&mut self, token: Token) -> Result<&str, CsvppError> {
        match token {
            Token::Equals =>            self.take_while(|ch| { ch == '=' }),
            Token::EndModifier =>       self.take("]]"),
            Token::ModifierName =>      self.take_while(|ch| { ch.is_alphanumeric() }),
            Token::Slash =>             self.take_while(|ch| { ch == '/' }),
            Token::StartCellModifier => self.take("[["),
            Token::StartRowModifier =>  self.take("![["),
        }
    }
    */

    /*
    fn maybe_take(&self, match_str: &'a str) -> Option<&'a str> {
        for (i, c) in self.chars.enumerate() {
            if c.is_whitespace() {
                continue;
            } else {
                break;
            }
        }

        let first = self.chars.next();
        
        let next_chunk = loop {
        }

        let next_chunk = self.chars.next_chunk(match_str.len());
        if next_chunk == match_str {
            // move `input` past the match
            // self.input.replace_with(|&mut old| &old[..match_str.len()]);
            Some(next_chunk)
        } else {
            None
        }

    }

        */


    /*
    fn skip_whitespace(&self) -> () {
        let mut iter = self.chars.peekable();
        loop {
            let next_char = iter.peek();
            if next_char.unwrap().is_whitespace() {
                iter.next();
            } else {
                break;
            }
        }
    }
    */

    pub fn rest(&mut self) -> String {
        self.input.trim().to_string()
    }

    pub fn take_start_modifier(&mut self) -> Option<Token> {
        let input = self.input.trim();
        
        if input.starts_with("[[") {
            self.input = input[2..].to_string();
            Some(Token::StartCellModifier)
        } else if input.starts_with("![[") {
            Some(Token::StartRowModifier)
        } else {
            None
        }
    }
}

fn parse_modifier(_lexer: &mut ModifierLexer, modifier: &mut Modifier) -> Result<(), String> {
    modifier.formats.insert(TextFormat::Bold);

    // XXX look in the hashmap
    Ok(())
}

fn parse_modifiers(lexer: &mut ModifierLexer, modifier: &mut Modifier) -> Result<(), String> {
    Ok(parse_modifier(lexer, modifier)?)
        // XXX handle if there are multiple
}

fn can_define_row_index(index: &Position) -> bool {
    index.0 == 0
}

fn has_modifiers() -> bool {
    // self.input.
    true
}

/// returns (modifier, row_modifier)
pub fn parse_all_modifiers(
    lexer: &mut ModifierLexer,
    default_from: &Modifier,
) -> Result<(Option<Modifier>, Option<Modifier>), String> {
    /*
    if !self.has_modifiers() {
        return Ok((None, None));
    }
    */

    let mut modifier: Option<Modifier> = None;
    let mut row_modifier: Option<Modifier> = None;

    while let Some(start_token) = lexer.take_start_modifier() {
        let is_row_modifier = start_token == Token::StartRowModifier;
        let mut new_modifier = Modifier::from(default_from);

        parse_modifiers(lexer, &mut new_modifier)?;

        if is_row_modifier {
            if row_modifier != None {
                return Err("You can only define one row modifier for a cell".to_string())
            } 

            row_modifier = Some(new_modifier)
        } else {
            if modifier != None {
                return Err("You can only define one modifier for a cell".to_string())
            }

            modifier = Some(new_modifier)
        }
    }

    Ok((modifier, row_modifier))
}

pub fn parse<'a>(
    index: crate::Position, 
    // XXX make this &'a and make ParsedModifiers.value be &'a str to avoid some copies
    input: String, 
    default_from: Modifier,
) -> Result<ParsedModifiers, CsvppError<'a>> {
    let lexer = &mut ModifierLexer::new(input);

    match parse_all_modifiers(lexer, &default_from) {
        Ok((modifier, row_modifier)) => {
            // if row_modifier != None && !self.can_define_row_index(&) {
            if row_modifier != None {
                Err(CsvppError::ModifierSyntaxError { 
                    // bad_input: &self.lexer.input_without_modifiers(),
                    bad_input: "",
                    index,
                    message: "You can only define a row modifier on the first cell of a row".to_string(), 
                })
            } else {
                Ok(ParsedModifiers {
                    modifier: modifier.unwrap_or_else(|| Modifier::from(&default_from)),
                    row_modifier: row_modifier.unwrap_or(default_from.clone()),
                    value: lexer.rest().to_string(),
                })
            }
        },
        Err(message) => {
            Err(CsvppError::ModifierSyntaxError { 
                // bad_input: &self.lexer.input_without_modifiers(),
                bad_input: "",
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
        let parsed_modifiers = parse((0, 0), "abc123".to_string(), default_modifier).unwrap();

        assert_eq!(parsed_modifiers.value, "abc123");
        assert_eq!(parsed_modifiers.modifier.row_level, false);
        assert_eq!(parsed_modifiers.row_modifier.row_level, true);
    }

    #[test]
    fn parse_modifier() {
        let default_modifier = Modifier::new(true);
        let parsed_modifiers = parse(
            (0, 0), 
            "[[format=bold]]abc123".to_string(),
            default_modifier,
        ).unwrap();

        assert_eq!(parsed_modifiers.value, "abc123")
    }
}
