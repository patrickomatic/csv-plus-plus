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
// * need to lowercase the input but we can't do it on the entire value because we don't want to
//     lowercase the stuff outside the modifier definition
use crate::{InnerError, InnerResult};

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
        Self {
            input: input.to_owned(),
        }
    }

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

    pub fn take_modifier_right_side(&mut self) -> InnerResult<String> {
        self.take_token(Token::Equals)?;
        self.take_token(Token::ModifierRightSide)
    }

    pub fn maybe_take_token(&mut self, token: Token) -> Option<String> {
        match token {
            Token::Equals => self.maybe_take("="),
            Token::Slash => self.maybe_take("/"),
            _ => panic!("Cannot maybe take: {:?}", token),
        }
    }

    pub fn take_token(&mut self, token: Token) -> InnerResult<String> {
        match token {
            Token::Color => self.take_color(),
            Token::EndModifier => self.take("]]"),
            Token::Equals => self.take("="),
            Token::ModifierName => self.take_while(|ch| ch.is_alphanumeric()),
            Token::ModifierRightSide => self.take_while(|ch| ch.is_alphanumeric() || ch == '_'),
            Token::PositiveNumber => self.take_while(|ch| ch.is_ascii_digit()),
            Token::String => self.take_string(),
            Token::Slash => self.take("/"),
            Token::StartCellModifier => self.take("[["),
            Token::StartRowModifier => self.take("![["),
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

    fn take(&mut self, substring: &str) -> InnerResult<String> {
        let input = self.input.trim();

        if let Some(without_match) = input.strip_prefix(substring) {
            self.input = without_match.to_string();
            Ok(substring.to_string())
        } else {
            Err(InnerError::bad_input(
                input,
                &format!("Error parsing input, expected '{}'", substring),
            ))
        }
    }

    fn take_color(&mut self) -> InnerResult<String> {
        let mut matched_alphas = 0;
        let mut saw_hash = false;
        let mut matched = "".to_string();

        for c in self.input.trim().chars() {
            if c == '#' && !saw_hash {
                saw_hash = true;
                matched.push(c);
            } else if c.is_alphanumeric() {
                if matched_alphas > 6 {
                    return Err(InnerError::bad_input(
                        &self.input,
                        &format!("Unexpected RGB color character: '{}'", c),
                    ));
                }

                matched.push(c);
                matched_alphas += 1;
            } else {
                // either we're done or it's a syntax error
                if matched_alphas == 3 || matched_alphas == 6 {
                    break;
                }

                return Err(InnerError::bad_input(
                    &self.input,
                    &format!("Invalid character when parsing RGB color: '{}'", c),
                ));
            }
        }

        self.input = self.input[matched.len()..].to_string();
        Ok(matched)
    }

    fn take_string(&mut self) -> InnerResult<String> {
        let input = self.input.trim();

        if input.starts_with('\'') {
            Ok(self.take_single_quoted_string()?)
        } else {
            Ok(self.take_while(|ch| ch.is_alphanumeric())?)
        }
    }

    #[allow(clippy::explicit_counter_loop)]
    fn take_single_quoted_string(&mut self) -> InnerResult<String> {
        let mut escape_mode = false;
        let mut matched = "".to_string();
        let mut start_quote = false;
        let mut end_quote = false;
        let mut consumed = 0;

        for c in self.input.trim().chars() {
            // due to escaping rules, we don't always put what we consume on `matched`.  so we need
            // to keep track of it separately
            consumed += 1;

            if start_quote {
                if escape_mode {
                    matched.push(c);
                    escape_mode = false;
                } else if c == '\\' {
                    escape_mode = true;
                } else if c == '\'' {
                    end_quote = true;
                    break;
                } else {
                    matched.push(c);
                }
            } else if c == '\'' {
                start_quote = true;
            } else {
                return Err(InnerError::bad_input(
                    &self.input,
                    "Expected a starting single quote",
                ));
            }
        }

        if start_quote && end_quote {
            self.input = self.input[consumed..].to_string();
            Ok(matched)
        } else {
            Err(InnerError::bad_input(
                &self.input,
                "Expected a start and ending quote",
            ))
        }
    }

    fn take_while<F>(&mut self, while_fn: F) -> InnerResult<String>
    where
        F: Fn(char) -> bool,
    {
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
            // TODO this message is misleading I think
            Err(InnerError::bad_input(
                input,
                "Expected a modifier definition (i.e. format/halign/etc)",
            ))
        } else {
            self.input = input[matched.len()..].to_string();
            Ok(matched)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maybe_take_start_modifier_modifier() {
        let mut lexer = ModifierLexer::new("[[");
        assert_eq!(
            Some(Token::StartCellModifier),
            lexer.maybe_take_start_modifier()
        );
    }

    #[test]
    fn maybe_take_start_modifier_row_modifier() {
        let mut lexer = ModifierLexer::new("![[");
        assert_eq!(
            Some(Token::StartRowModifier),
            lexer.maybe_take_start_modifier()
        );
    }

    #[test]
    fn maybe_take_start_modifier_none() {
        let mut lexer = ModifierLexer::new("foo");
        assert_eq!(None, lexer.maybe_take_start_modifier());
    }

    #[test]
    fn take_modifier_right_side() {
        let mut lexer = ModifierLexer::new("=foo_bar");
        assert_eq!("foo_bar", lexer.take_modifier_right_side().unwrap());
    }

    #[test]
    fn take_modifier_right_side_invalid() {
        let mut lexer = ModifierLexer::new("foo");
        assert!(lexer.take_modifier_right_side().is_err());
    }

    #[test]
    fn maybe_take_token_equals() {
        let mut lexer = ModifierLexer::new("=");
        assert_eq!(Some("=".to_string()), lexer.maybe_take_token(Token::Equals));
    }

    #[test]
    fn maybe_take_token_slash() {
        let mut lexer = ModifierLexer::new("/");
        assert_eq!(Some("/".to_string()), lexer.maybe_take_token(Token::Slash));
    }

    #[test]
    fn maybe_take_token_none() {
        let mut lexer = ModifierLexer::new("foo");
        assert_eq!(None, lexer.maybe_take_token(Token::Slash));
    }

    #[test]
    fn take_token_color() {
        let mut lexer = ModifierLexer::new("#ABC123");
        assert_eq!("#ABC123", lexer.take_token(Token::Color).unwrap());
    }

    #[test]
    fn take_token_color_shorthand() {
        let mut lexer = ModifierLexer::new("#ABC");
        assert_eq!("#ABC", lexer.take_token(Token::Color).unwrap());
    }

    #[test]
    fn take_token_color_no_hash() {
        let mut lexer = ModifierLexer::new("ABC123");
        assert_eq!("ABC123", lexer.take_token(Token::Color).unwrap());
    }

    #[test]
    fn take_token_end_modifier() {
        let mut lexer = ModifierLexer::new("]]");
        assert_eq!("]]", lexer.take_token(Token::EndModifier).unwrap());
    }

    #[test]
    fn take_token_equals() {
        let mut lexer = ModifierLexer::new(" = ");
        assert_eq!("=", lexer.take_token(Token::Equals).unwrap());
    }

    #[test]
    fn take_token_modifier_name() {
        let mut lexer = ModifierLexer::new("foo");
        assert_eq!("foo", lexer.take_token(Token::ModifierName).unwrap());
    }

    #[test]
    fn take_token_positive_number() {
        let mut lexer = ModifierLexer::new("15");
        assert_eq!("15", lexer.take_token(Token::PositiveNumber).unwrap());
    }

    #[test]
    fn take_token_string() {
        let mut lexer = ModifierLexer::new("string");
        assert_eq!("string", lexer.take_token(Token::String).unwrap());
    }

    #[test]
    fn take_token_string_double_quoted() {
        let mut lexer = ModifierLexer::new("'this is \\' a quoted string\\''");
        assert_eq!(
            "this is ' a quoted string'",
            lexer.take_token(Token::String).unwrap()
        );
        // make sure it consumed `input` given the quoting rules
        assert_eq!("", lexer.input);
    }

    #[test]
    fn take_token_slash() {
        let mut lexer = ModifierLexer::new(" / ");
        assert_eq!("/", lexer.take_token(Token::Slash).unwrap());
    }

    #[test]
    fn take_token_invalid() {
        let mut lexer = ModifierLexer::new("foo");
        assert!(lexer.take_token(Token::PositiveNumber).is_err());
    }

    #[test]
    fn rest() {
        let mut lexer = ModifierLexer::new(" / = rest");
        lexer.take_token(Token::Slash).unwrap();
        lexer.take_token(Token::Equals).unwrap();

        assert_eq!(" rest", lexer.rest());
    }
}
