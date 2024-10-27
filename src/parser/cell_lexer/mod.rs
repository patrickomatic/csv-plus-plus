//! # `CellLexer`
//!
//! This is the lexer/tokenizer used for parsing csv++ cells - rather than tokenizing all in one
//! go, this lexer works by extracting a token one at a time as we parse.
//!
use super::TokenMatcher;
use crate::error::{BadInput, ParseError, ParseResult};
use crate::{ArcSourceCode, CharOffset};
use csvp::Field;

mod token;
mod token_library;
mod token_match;
mod unknown_token;

pub(crate) use token::Token;
pub(crate) use token_library::TokenLibrary;
pub(crate) use token_match::TokenMatch;
pub(crate) use unknown_token::UnknownToken;

#[derive(Debug)]
pub(crate) struct CellLexer<'a> {
    cell_offset: CharOffset,
    pub(crate) field: &'a Field,
    pub(crate) input: &'a str,
    source_code: ArcSourceCode,
    token_library: &'static TokenLibrary,
}

impl<'a> CellLexer<'a> {
    pub(super) fn new(field: &'a Field, source_code: ArcSourceCode) -> Self {
        Self {
            cell_offset: 0,
            field,
            input: &field.value,
            source_code,
            token_library: TokenLibrary::library(),
        }
    }

    /// The rest of the input that has not been consumed
    pub(super) fn rest(&self) -> String {
        self.input.trim().to_string()
    }

    pub(super) fn maybe_take_start_options(&mut self) -> Option<TokenMatch> {
        self.take_whitespace();

        if let Some(without_match) = self.input.strip_prefix("[[") {
            let token_match = self.match_token(Token::StartCellOptions, "[[");
            self.replace_input(without_match, 2);
            Some(token_match)
        } else if let Some(without_match) = self.input.strip_prefix("![[") {
            let token_match = self.match_token(Token::StartRowOptions, "![[");
            self.replace_input(without_match, 3);
            Some(token_match)
        } else {
            None
        }
    }

    // TODO: this name is kinda misleading since it also takes an equal first
    // maybe just rename to take_identifier
    pub(super) fn take_option_right_side(&mut self) -> ParseResult<TokenMatch> {
        self.take_token(Token::Equals)?;
        self.take_token(Token::Identifier)
    }

    pub(super) fn maybe_take_date(&mut self) -> Option<TokenMatch> {
        self.maybe_take_regex(&self.token_library.date)
    }

    pub(super) fn maybe_take_equals(&mut self) -> Option<TokenMatch> {
        self.maybe_take(Token::Equals, "=")
    }

    pub(super) fn maybe_take_number(&mut self) -> Option<TokenMatch> {
        self.maybe_take_regex(&self.token_library.number)
    }

    pub(super) fn maybe_take_identifier(&mut self) -> Option<TokenMatch> {
        self.maybe_take_regex(&self.token_library.identifier)
    }

    pub(super) fn maybe_take_single_quoted_string(&mut self) -> ParseResult<Option<TokenMatch>> {
        Ok(
            if self
                .token_library
                .single_quoted_string
                .try_match(self.input)
                .is_some()
            {
                Some(self.take_single_quoted_string()?)
            } else {
                None
            },
        )
    }

    pub(super) fn maybe_take_slash(&mut self) -> Option<TokenMatch> {
        self.maybe_take(Token::Slash, "/")
    }

    pub(super) fn maybe_take_end_options(&mut self) -> Option<TokenMatch> {
        self.maybe_take(Token::EndOptions, "]]")
    }

    // is the next token to be consumed a `)`? (this will not consume it)
    pub(super) fn peek_close_parenthesis(&mut self) -> bool {
        self.token_library
            .close_parenthesis
            .try_match(self.input)
            .is_some()
    }

    pub(super) fn take_token(&mut self, token: Token) -> ParseResult<TokenMatch> {
        // spaces can be anywhere, so take any leading space
        self.take_whitespace();

        match token {
            Token::A1 => self.take_while(token, |ch| {
                // TODO: make a list of valid A1 characters somewhere
                ch.is_alphanumeric() || ch == '!' || ch == '\'' || ch == ':' || ch == '$'
            }),
            Token::CloseParenthesis => self.take(token, ")"),
            Token::Color => self.take_color(),
            Token::Date => self.take_date(),
            Token::EndOptions => self.take(token, "]]"),
            Token::Equals => self.take(token, "="),
            Token::OptionName => self.take_while(token, char::is_alphanumeric),
            Token::Identifier => self.take_while(token, |ch| ch.is_alphanumeric() || ch == '_'),
            Token::Number => {
                // TODO: I could do a little better (enforce only one starting - and one .)
                self.take_while(token, |ch| ch.is_ascii_digit() || ch == '-' || ch == '.')
            }
            Token::OpenParenthesis => self.take(token, "("),
            Token::PositiveNumber => self.take_while(token, |ch| ch.is_ascii_digit()),
            Token::String => self.take_string(),
            Token::Slash => self.take(token, "/"),
            Token::StartCellOptions => self.take(token, "[["),
            Token::StartRowOptions => self.take(token, "![["),
        }
    }

    pub(super) fn take_date(&mut self) -> ParseResult<TokenMatch> {
        self.take_whitespace();

        if let Some(m) = self.token_library.date.try_match(self.input) {
            let str_match = m.str_match;

            // move the offset past any observed whitespace, then build the token
            self.move_input(m.len_leading_whitespace);
            let token = self.match_token(Token::Date, str_match);
            self.move_input(str_match.len());
            Ok(token)
        } else {
            Err(self.unknown_string("Expected a date"))
        }
    }

    pub fn take_whitespace(&mut self) {
        let new_input = self.input.trim_start();
        self.move_input(self.input.len() - new_input.len());
    }

    fn match_token(&self, token: Token, str_match: &'a str) -> TokenMatch {
        TokenMatch {
            token,
            str_match: str_match.to_string(),
            field: self.field.clone(),
            cell_offset: self.current_cell_offset(),
            source_code: self.source_code.clone(),
        }
    }

    pub(super) fn unknown_string<S: Into<String>>(&self, message: S) -> ParseError {
        UnknownToken {
            bad_input: self.input.to_string(),
            field: self.field.clone(),
            cell_offset: self.current_cell_offset(),
            source_code: self.source_code.clone(),
        }
        .into_parse_error(message)
    }

    fn maybe_take(&mut self, token: Token, substring: &'a str) -> Option<TokenMatch> {
        self.take_whitespace();

        if let Some(without_match) = self.input.strip_prefix(substring) {
            let token_match = self.match_token(token, substring);
            self.replace_input(without_match, substring.len());
            Some(token_match)
        } else {
            None
        }
    }

    fn maybe_take_regex(&mut self, tm: &TokenMatcher<Token>) -> Option<TokenMatch> {
        tm.try_match(self.input).map(|m| {
            let str_match = m.str_match;

            // move the offset past any observed whitespace, then build the token
            self.move_input(m.len_leading_whitespace);
            let token = self.match_token(tm.0, str_match);
            self.move_input(str_match.len());
            token
        })
    }

    fn take(&mut self, token: Token, substring: &'a str) -> ParseResult<TokenMatch> {
        self.take_whitespace();

        if let Some(without_match) = self.input.strip_prefix(substring) {
            let token_match = self.match_token(token, substring);
            self.replace_input(without_match, substring.len());
            Ok(token_match)
        } else {
            Err(self.unknown_string(format!("Error parsing input, expected '{substring}'")))
        }
    }

    fn take_color(&mut self) -> ParseResult<TokenMatch> {
        let mut matched_alphas = 0;
        let mut saw_hash = false;
        let mut matched = String::new();

        self.take_whitespace();

        for c in self.input.chars() {
            if c == '#' && !saw_hash {
                saw_hash = true;
                matched.push(c);
            } else if c.is_alphanumeric() {
                if matched_alphas > 6 {
                    return Err(
                        self.unknown_string(format!("Unexpected RGB color character: '{c}'"))
                    );
                }

                matched.push(c);
                matched_alphas += 1;
            } else {
                // either we're done or it's a syntax error
                if matched_alphas == 3 || matched_alphas == 6 {
                    break;
                }

                return Err(
                    self.unknown_string(format!("Invalid character when parsing RGB color: '{c}'"))
                );
            }
        }

        let token_match = self.match_token(Token::Color, &matched);
        self.move_input(matched.len());
        Ok(token_match)
    }

    fn take_string(&mut self) -> ParseResult<TokenMatch> {
        self.take_whitespace();

        if self.input.starts_with('\'') {
            Ok(self.take_single_quoted_string()?)
        } else {
            Ok(self.take_while(Token::String, char::is_alphanumeric)?)
        }
    }

    fn take_single_quoted_string(&mut self) -> ParseResult<TokenMatch> {
        let mut escape_mode = false;
        let mut matched = String::new();
        let mut start_quote = false;
        let mut end_quote = false;
        // TODO: pretty sure we can just use .enumerate() and get rid of the clippy allow above...
        // but I remember this code being tricky.  just make sure it's unit tested before removing
        let mut consumed = 0;

        self.take_whitespace();

        for c in self.input.chars() {
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
                return Err(self.unknown_string("Expected a starting single quote"));
            }
        }

        if start_quote && end_quote {
            let token_match = self.match_token(Token::String, &matched);
            self.move_input(consumed);
            Ok(token_match)
        } else {
            Err(self.unknown_string("Unterminated single-quoted string"))
        }
    }

    fn take_while<F>(&mut self, token: Token, while_fn: F) -> ParseResult<TokenMatch>
    where
        F: Fn(char) -> bool,
    {
        self.take_whitespace();

        let mut matched = String::new();

        for c in self.input.chars() {
            if while_fn(c) {
                matched.push(c);
            } else {
                break;
            }
        }

        if matched.is_empty() {
            Err(self.unknown_string(format!("Expected a {token}")))
        } else {
            self.move_input(matched.len());
            Ok(self.match_token(token, &matched))
        }
    }

    // our parser has the `cell_offset` set to one position ahead of where we last read
    fn current_cell_offset(&self) -> CharOffset {
        self.cell_offset.saturating_sub(1)
    }

    fn move_input(&mut self, amount: CharOffset) {
        self.input = &self.input[amount..];
        self.cell_offset += amount;
    }

    fn replace_input(&mut self, new_input: &'a str, amount: CharOffset) {
        self.input = new_input;
        self.cell_offset += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn test_lexer(field: &Field) -> CellLexer {
        CellLexer::new(field, build_source_code_from_input(&field.value))
    }

    #[test]
    fn maybe_take_date() {
        let field = build_field("  11/2/2024, 1, 2", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!(lexer.maybe_take_date().unwrap().token, Token::Date);
        assert_eq!(lexer.input, ", 1, 2");
        assert_eq!(lexer.cell_offset, 11);
    }

    #[test]
    fn maybe_take_identifier() {
        let field = build_field("     foo bar baz", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!(
            lexer.maybe_take_identifier().unwrap().token,
            Token::Identifier
        );
        assert_eq!(lexer.input, " bar baz");
        assert_eq!(lexer.cell_offset, 8);
    }

    #[test]
    fn maybe_take_single_quoted_string() {
        let field = build_field("     'foo bar' baz", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!(
            lexer
                .maybe_take_single_quoted_string()
                .unwrap()
                .unwrap()
                .token,
            Token::String
        );
        assert_eq!(lexer.input, " baz");
        assert_eq!(lexer.cell_offset, 14);
    }

    #[test]
    fn maybe_take_start_cell_options() {
        let field = build_field("[[", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!(
            Token::StartCellOptions,
            lexer.maybe_take_start_options().unwrap().token
        );
    }

    #[test]
    fn maybe_take_start_row_options() {
        let field = build_field("![[", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!(
            Token::StartRowOptions,
            lexer.maybe_take_start_options().unwrap().token
        );
    }

    #[test]
    fn maybe_take_start_options_none() {
        let field = build_field("foo", (0, 0));
        let mut lexer = test_lexer(&field);

        assert!(lexer.maybe_take_start_options().is_none());
    }

    #[test]
    fn take_option_right_side() {
        let field = build_field("=foo_bar", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!("foo_bar", lexer.take_option_right_side().unwrap().str_match);
    }

    #[test]
    fn take_option_right_side_invalid() {
        let field = build_field("foo", (0, 0));
        let mut lexer = test_lexer(&field);

        assert!(lexer.take_option_right_side().is_err());
    }

    #[test]
    fn maybe_take_equals() {
        let field = build_field("=", (0, 0));
        let mut lexer = test_lexer(&field);

        assert!(lexer.maybe_take_equals().is_some());
    }

    #[test]
    fn maybe_take_end_options() {
        let field = build_field("]]", (0, 0));
        let mut lexer = test_lexer(&field);

        assert!(lexer.maybe_take_end_options().is_some());
    }

    #[test]
    fn maybe_take_slash() {
        let field = build_field("/", (0, 0));
        let mut lexer = test_lexer(&field);

        assert!(lexer.maybe_take_slash().is_some());
    }

    #[test]
    fn take_token_color() {
        let field = build_field("#ABC123", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!("#ABC123", lexer.take_token(Token::Color).unwrap().str_match);
    }

    #[test]
    fn take_token_color_shorthand() {
        let field = build_field("#ABC", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!("#ABC", lexer.take_token(Token::Color).unwrap().str_match);
    }

    #[test]
    fn take_token_color_no_hash() {
        let field = build_field("ABC123", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!("ABC123", lexer.take_token(Token::Color).unwrap().str_match);
    }

    #[test]
    fn take_token_date() {
        let field = build_field(" 2022-01-02, foo", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!(
            "2022-01-02",
            lexer.take_token(Token::Date).unwrap().str_match
        );
        assert_eq!(lexer.input, ", foo");
        assert_eq!(lexer.cell_offset, 11);
    }

    #[test]
    fn take_token_end_options() {
        let field = build_field("]]", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!("]]", lexer.take_token(Token::EndOptions).unwrap().str_match);
    }

    #[test]
    fn take_token_equals() {
        let field = build_field(" = ", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!("=", lexer.take_token(Token::Equals).unwrap().str_match);
    }

    #[test]
    fn take_token_option_name() {
        let field = build_field("foo", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!(
            "foo",
            lexer.take_token(Token::OptionName).unwrap().str_match
        );
    }

    #[test]
    fn take_token_positive_number() {
        let field = build_field("15", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!(
            "15",
            lexer.take_token(Token::PositiveNumber).unwrap().str_match
        );
    }

    #[test]
    fn take_token_string() {
        let field = build_field("string", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!("string", lexer.take_token(Token::String).unwrap().str_match);
    }

    #[test]
    fn take_token_string_double_quoted() {
        let field = build_field("'this is \\' a quoted string\\''", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!(
            "this is ' a quoted string'",
            lexer.take_token(Token::String).unwrap().str_match
        );
        // make sure it consumed `input` given the quoting rules
        assert_eq!("", lexer.input);
    }

    #[test]
    fn take_token_slash() {
        let field = build_field(" / ", (0, 0));
        let mut lexer = test_lexer(&field);

        assert_eq!("/", lexer.take_token(Token::Slash).unwrap().str_match);
    }

    #[test]
    fn take_token_invalid() {
        let field = build_field("foo", (0, 0));
        let mut lexer = test_lexer(&field);

        assert!(lexer.take_token(Token::PositiveNumber).is_err());
    }

    #[test]
    fn rest() {
        let field = build_field(" / = rest", (0, 0));
        let mut lexer = test_lexer(&field);

        lexer.take_token(Token::Slash).unwrap();
        lexer.take_token(Token::Equals).unwrap();

        assert_eq!("rest", lexer.rest());
    }
}
