use rgb::RGB16;
use std::collections::HashSet;
use std::str::FromStr;

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

impl FromStr for BorderSide {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "all"       => Ok(Self::All),
            "top"       => Ok(Self::Top),
            "bottom"    => Ok(Self::Bottom),
            "left"      => Ok(Self::Left),
            "right"     => Ok(Self::Right),
            _           => Err(format!("Invalid border= value: {}", input)),
        }
    }
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

impl FromStr for BorderStyle {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dashed"        => Ok(Self::Dashed),
            "dotted"        => Ok(Self::Dotted),
            "double"        => Ok(Self::Double),
            "solid"         => Ok(Self::Solid),
            "solid_medium"  => Ok(Self::SolidMedium),
            "solid_thick"   => Ok(Self::SolidThick),
            _               => Err(format!("Invalid borderstyle= value: {}", input)),
        }
    }
}

/// The possible values for aligning a cell horizontally.
#[derive(Clone, Debug, PartialEq)]
enum HorizontalAlign {
    Center,
    Left,
    Right,
}

impl FromStr for HorizontalAlign {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "center"        => Ok(Self::Center),
            "left"          => Ok(Self::Left),
            "right"         => Ok(Self::Right),
            _               => Err(format!("Invalid halign= value: {}", input)),
        }
    }
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

impl FromStr for NumberFormat {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "currency"      => Ok(Self::Currency),
            "date"          => Ok(Self::Date),
            "date_time"     => Ok(Self::DateTime),
            "number"        => Ok(Self::Number),
            "percent"       => Ok(Self::Percent),
            "text"          => Ok(Self::Text),
            "time"          => Ok(Self::Time),
            "scientific"    => Ok(Self::Scientific),
            _               => Err(format!("Invalid numberformat= value: {}", input)),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum TextFormat {
    Bold,
    Italic,
    Strikethrough,
    Underline,
}

impl FromStr for TextFormat {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "bold"          => Ok(Self::Bold),
            "italic"        => Ok(Self::Italic),
            "strikethrough" => Ok(Self::Strikethrough),
            "underline"     => Ok(Self::Underline),
            _               => Err(format!("Invalid format= value: {}", input)),
        }
    }
}

/// The possible values for aligning a cell vertically.
#[derive(Clone, Debug, PartialEq)]
enum VerticalAlign {
    Bottom,
    Center,
    Top,
}

impl FromStr for VerticalAlign {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "bottom"    => Ok(Self::Bottom),
            "center"    => Ok(Self::Center),
            "top"       => Ok(Self::Top),
            _           => Err(format!("Invalid valign= value: {}", input)),
        }
    }
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
    pub index: Position,
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


#[derive(PartialEq)]
pub enum Token {
    Color,
    EndModifier,
    Equals,
    ModifierName,
    ModifierRightSide,
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

    pub fn rest(&mut self) -> String {
        self.input.trim().to_string()
    }

    pub fn take_start_modifier(&mut self) -> Option<Token> {
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

    pub fn take_modifier_right_side(&mut self) -> Result<String, String> {
        self.take_token(Token::Equals)?;
        self.take_token(Token::ModifierRightSide)
    }

    pub fn take_token(&mut self, token: Token) -> Result<String, String> {
        match token {
            Token::Color =>             self.take_color(),
            Token::Equals =>            self.take("="),
            Token::EndModifier =>       self.take("]]"),
            Token::ModifierName =>      self.take_while(|ch| { ch.is_alphanumeric() }),
            Token::ModifierRightSide => self.take_while(|ch| { ch.is_alphanumeric() || ch == '_' }),
            Token::Slash =>             self.take("/"),
            Token::StartCellModifier => self.take("[["),
            Token::StartRowModifier =>  self.take("![["),
        }
    }

    fn take<'a>(&mut self, substring: &'a str) -> Result<String, String> {
        let input = self.input.trim();

        if input.starts_with(substring) {
            self.input = input[substring.len()..].to_string();
            Ok(substring.to_string())
        } else {
            Err(format!("Error parsing input, expected '{}'", substring))
        }
    }

    fn take_color<'a>(&mut self) -> Result<String, String> {
        Ok(String::from("#FFF"))
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

fn parse_border_modifier(lexer: &mut ModifierLexer, modifier: &mut Modifier) -> Result<(), String> {
    modifier.borders.insert(BorderSide::from_str(&lexer.take_modifier_right_side()?)?);
    Ok(())
}

fn parse_border_color_modifier(lexer: &mut ModifierLexer, _modifier: &mut Modifier) -> Result<(), String> {
    lexer.take_token(Token::Equals)?;
    let _color = lexer.take_token(Token::Color)?;
    // TODO
    // modifier.border_color = RGB16 { 
    Ok(())
}

fn parse_border_style_modifier(lexer: &mut ModifierLexer, modifier: &mut Modifier) -> Result<(), String> {
    modifier.border_style = Some(BorderStyle::from_str(&lexer.take_modifier_right_side()?)?);
    Ok(())
}

fn parse_format_modifier(lexer: &mut ModifierLexer, modifier: &mut Modifier) -> Result<(), String> {
    modifier.formats.insert(TextFormat::from_str(&lexer.take_modifier_right_side()?)?);
    Ok(())
}

fn parse_halign_modifier(lexer: &mut ModifierLexer, modifier: &mut Modifier) -> Result<(), String> {
    modifier.horizontal_align = Some(HorizontalAlign::from_str(&lexer.take_modifier_right_side()?)?);
    Ok(())
}

fn parse_valign_modifier(lexer: &mut ModifierLexer, modifier: &mut Modifier) -> Result<(), String> {
    modifier.vertical_align = Some(VerticalAlign::from_str(&lexer.take_modifier_right_side()?)?);
    Ok(())
}

fn parse_modifier(lexer: &mut ModifierLexer, modifier: &mut Modifier) -> Result<(), String> {
    let modifier_name = lexer.take_token(Token::ModifierName)?;

    match modifier_name.as_str() {
        "f" | "format"          => parse_format_modifier(lexer, modifier),
        "b" | "border"          => parse_border_modifier(lexer, modifier),
        "bc" | "bordercolor"    => parse_border_color_modifier(lexer, modifier),
        "bs" | "borderstyle"    => parse_border_style_modifier(lexer, modifier),
        "ha" | "halign"         => parse_halign_modifier(lexer, modifier),
        "va" | "valign"         => parse_valign_modifier(lexer, modifier),
        _ => return Err(format!("Unrecognized modifier: {}", modifier_name))
    }
}

fn parse_modifiers(lexer: &mut ModifierLexer, modifier: &mut Modifier) -> Result<(), String> {
    parse_modifier(lexer, modifier)?;
        // XXX handle if there are multiple
    // while let Some(_) = lexer.maybe_take(Token::Slash) {
    // }
    lexer.take_token(Token::EndModifier)?;
    Ok(())
}

/// returns (modifier, row_modifier)
pub fn parse_all_modifiers(
    lexer: &mut ModifierLexer,
    default_from: &Modifier,
) -> Result<(Option<Modifier>, Option<Modifier>), String> {
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
    index: Position, 
    input: String, 
    default_from: Modifier,
) -> Result<ParsedModifiers, CsvppError<'a>> {
    let lexer = &mut ModifierLexer::new(input);

    match parse_all_modifiers(lexer, &default_from) {
        Ok((modifier, row_modifier)) => {
            if row_modifier != None && index.is_first_cell() {
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
                    value: lexer.rest(),
                    index
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
        let parsed_modifiers = parse(Position(0, 0), String::from("abc123"), default_modifier).unwrap();

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

        assert!(modifier.formats.contains(&TextFormat::Bold));
        assert_eq!(value, "abc123");
    }
}
