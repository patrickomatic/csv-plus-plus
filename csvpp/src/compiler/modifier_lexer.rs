//! # ModifierLexer
//!
//! This is the lexer/tokenizer used for parsing csv++ modifiers - it's a little different than
//! most parsers which parse their entire input into tokens in one go. This tokenizes as the
//! parser goes since it is context-dependent.
//!
//! [https://en.wikipedia.org/wiki/Lexer_hack](See also: Lexer hack)
//!
//!
// TODO:
// * fix the row parsing logic (it doesn't apply row modifiers)
//
// * need to lowercase the input but we can't do it on the entire value because we don't want to
//     lowercase the stuff outside the modifier definition
//
// * get quoted strings working
use crate::{Error, Result};

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct ModifierLexer {
    input: String,
}

impl ModifierLexer {
    pub fn new(input: &str) -> Self {
        Self { input: input.to_owned() }
    }

    // TODO: can this just give over ownership?
    pub fn rest(&self) -> String {
        self.input.clone()
    }

    pub fn maybe_take_start_modifier(&mut self) -> Option<Token> {
        let input = self.input.trim();
        
        if let Some(without_match) = input.strip_prefix("[[") {
            self.input = without_match.to_string();
            Some(Token::StartCellModifier)
        } else if let Some(without_match) = input.strip_prefix("![[") {
            self.input = without_match.to_string();
            Some(Token::StartRowModifier)
        } else {
            None
        }
    }

    pub fn take_modifier_right_side(&mut self) -> Result<String> {
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

    fn maybe_take(&mut self, substring: &str) -> Option<String> {
        let input = self.input.trim();

        if let Some(without_match) = input.strip_prefix(substring) {
            self.input = without_match.to_string();
            Some(substring.to_string())
        } else {
            None
        }
    }

    pub fn take_token(&mut self, token: Token) -> Result<String> {
        match token {
            Token::Color =>             self.take_color(),
            Token::EndModifier =>       self.take("]]"),
            Token::Equals =>            self.take("="),
            Token::ModifierName =>      self.take_while(|ch| ch.is_alphanumeric()),
            Token::ModifierRightSide => self.take_while(|ch| ch.is_alphanumeric() || ch == '_'),
            Token::PositiveNumber =>    self.take_while(|ch| ch.is_ascii_digit()),
            Token::String =>            self.take_string(),
            Token::Slash =>             self.take("/"),
            Token::StartCellModifier => self.take("[["),
            Token::StartRowModifier =>  self.take("![["),
        }
    }

    fn take(&mut self, substring: &str) -> Result<String> {
        let input = self.input.trim();

        if let Some(without_match) = input.strip_prefix(substring) {
            self.input = without_match.to_string();
            Ok(substring.to_string())
        } else {
            Err(Error::ModifierSyntaxError {
                message: format!("Error parsing input, expected '{}'", substring),
                bad_input: input.to_string(),
                index: a1_notation::A1::builder().xy(0, 0).build()?, // XXX
            })
        }
    }

    fn take_color(&mut self) -> Result<String> {
        // XXX 
        todo!();
    }

    fn take_string(&mut self) -> Result<String> {
        let input = self.input.trim();

        if input.starts_with('\'') {
            Ok(self.take_single_quoted_string()?)
        } else {
            Ok(self.take_while(|ch| ch.is_alphanumeric())?)
        }
    }

    fn take_single_quoted_string(&mut self) -> Result<String> {
        // XXX
        todo!();
    }

    fn take_while<F>(
        &mut self, 
        while_fn: F,
    ) -> Result<String>
    where F: Fn(char) -> bool {
        let input = self.input.trim();
        let mut matched = "".to_string();

        for c in input.chars() {
            if while_fn(c) {
                matched.push(c);
            } else {
                break;
            }
        }

        if matched.is_empty() {
            Err(Error::ModifierSyntaxError {
                message: "Expected a modifier definition (i.e. format/halign/etc)".to_owned(),
                bad_input: input.to_string(),
                index: a1_notation::A1::builder().xy(0, 0).build()?, // XXX
            })
        } else {
            self.input = input[matched.len()..].to_string();
            Ok(matched)
        }
    }
}

#[cfg(test)]
mod tests {
    // user super::*;
    // TODO
}
