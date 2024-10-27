//! # `csv+` Parser
//!
//! A recursive descent, single-threaded CSV parser. This aims to be a CSV-compliant
//! implementation of RFC 4180, however there are some additional features:
//!
//! * Multiline-field support.  A `\` character preceding a newline will parse as if the newline
//!   were not there.
//! * Record comments. (A `#` at the beginning of the line comments out the whole record.)
//! * Maintains a mapping of parsed positions to their original location in the source code.
//!
//! # Terminology
//!
//! * `record` - a row of data.  typically a single line, unless it's split across multiple lines
//! * `field` - a single value from the record.  
//!
//! ## References
//!
//! * [RFC 4180: Common Format and MIME Type for Comma-Separated Values (CSV) Files](https://www.ietf.org/rfc/rfc4180.txt)
//!
use super::{Config, Error, Field, FieldBuilder, Record, Records, Result, SourcePosition};
use std::{iter, str};

#[derive(Debug)]
pub struct Parser<'a> {
    config: &'a Config,
    chars: iter::Peekable<str::Chars<'a>>,
    source_line: usize,
    source_offset: isize,
}

#[derive(Debug)]
enum FieldResult {
    Some(Field),
    Last(Field),
    Eof,
}

impl FieldResult {
    fn some<F: Into<Field>>(f: F) -> Self {
        Self::Some(f.into())
    }

    fn last<F: Into<Field>>(f: F) -> Self {
        Self::Last(f.into())
    }
}

#[derive(Debug)]
enum RecordResult {
    Comment,
    Eof,
    Some(Record),
}

impl From<&mut Parser<'_>> for SourcePosition {
    fn from(p: &mut Parser) -> Self {
        if let Ok(o) = usize::try_from(p.source_offset) {
            Self::new(o, p.source_line + p.config.lines_above)
        } else {
            panic!("Attempted to create a SourcePosition before the parser has consumed any characters.")
        }
    }
}

fn is_record_terminator(c: char) -> bool {
    c == '\n'
}

impl<'a> Parser<'a> {
    pub(super) fn new(input: &'a str, config: &'a Config) -> Self {
        Parser {
            config,
            chars: input.chars().peekable(),
            source_line: 0,
            // -1 because we're going to increment as we consume characters
            source_offset: -1,
        }
    }

    pub(super) fn parse(&mut self) -> Result<Records> {
        while self.chars.peek() == Some(&'\n') {
            self.consume_char();
        }

        let mut records = vec![];
        let mut row = 0;
        loop {
            match self.parse_record(row)? {
                RecordResult::Comment => continue,
                RecordResult::Some(r) => records.push(r),
                RecordResult::Eof => break,
            }
            row += 1;
        }

        Ok(records)
    }

    fn is_field_separator(&self, c: char) -> bool {
        c == self.config.separator
    }

    fn consume_and_ignore_line(&mut self) {
        loop {
            match self.consume_char() {
                Some('\n') | None => break,
                Some(_) => continue,
            }
        }
    }

    fn next_is_newline(&mut self) -> bool {
        self.chars.peek() == Some(&'\n')
    }

    fn parse_record(&mut self, row: usize) -> Result<RecordResult> {
        if let Some('#') = self.chars.peek() {
            self.consume_and_ignore_line();
            return Ok(RecordResult::Comment);
        }

        let mut fields = vec![];
        let mut col = 0;
        loop {
            match self.parse_field(FieldBuilder::new((col, row)))? {
                FieldResult::Some(f) => fields.push(f),
                FieldResult::Last(f) => {
                    fields.push(f);
                    break;
                }
                FieldResult::Eof => {
                    return Ok(if fields.is_empty() {
                        RecordResult::Eof
                    } else {
                        RecordResult::Some(fields)
                    })
                }
            };
            col += 1;
        }

        Ok(RecordResult::Some(fields))
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if !is_record_terminator(*c) && c.is_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    fn parse_field(&mut self, mut fb: FieldBuilder) -> Result<FieldResult> {
        self.consume_whitespace();

        match self.consume_char() {
            Some(c) if self.is_field_separator(c) => Ok(FieldResult::some(fb)),
            Some(c) if is_record_terminator(c) => Ok(FieldResult::last(fb)),
            Some('\\') if self.next_is_newline() => {
                // consume the newline then continue parsing this field
                self.consume_char();
                self.parse_field(fb)
            }
            // Some(c) if c.is_whitespace() => self.parse_field(fb),
            Some('"') => Ok(self.parse_quoted_field(fb, false)?),
            Some(c) => {
                fb.push(c, &mut *self);
                Ok(self.parse_unquoted_field(fb)?)
            }
            None => Ok(FieldResult::Eof),
        }
    }

    fn parse_unquoted_field(&mut self, mut fb: FieldBuilder) -> Result<FieldResult> {
        match self.consume_char() {
            Some('\\') if self.next_is_newline() => {
                // consume the newline then continue parsing this field
                self.consume_char();
                self.parse_unquoted_field(fb)
            }
            Some(c) if self.is_field_separator(c) => Ok(FieldResult::some(fb)),
            Some(c) if is_record_terminator(c) => Ok(FieldResult::last(fb)),
            Some(c) => {
                fb.push(c, &mut *self);
                self.parse_unquoted_field(fb)
            }
            None => Ok(FieldResult::last(fb)),
        }
    }

    fn parse_quoted_field(
        &mut self,
        mut fb: FieldBuilder,
        escape_mode: bool,
    ) -> Result<FieldResult> {
        let c = self.consume_char();
        if escape_mode {
            if let Some(c) = c {
                fb.push(c, &mut *self);
                return self.parse_quoted_field(fb, false);
            }

            return Err(self.parse_error("Expected a quoted character but got EOF"));
        }

        match c {
            Some('"') => {
                // is it two quotes in a row? if so it's a quoted quote
                // TODO: or it could be a empty string? would we parse `foo,"",bar` correctly?
                if let Some('"') = self.chars.peek() {
                    self.parse_quoted_field(fb, true)
                } else {
                    // otherwise it's a terminating quote. we need to also make sure we consume any
                    // trailing spaces and make sure there is a ','
                    self.parse_rest_of_quoted_field(fb)
                }
            }
            Some(c) => {
                fb.push(c, &mut *self);
                self.parse_quoted_field(fb, false)
            }
            None => Ok(FieldResult::some(fb)),
        }
    }

    /// At this point we've already seen the beginning and ending quotes and have consumed a field.
    /// But we need to consume any trailing spaces as well as the terminating comma.  Or throw an
    /// error if it'd not there.
    fn parse_rest_of_quoted_field(&mut self, fb: FieldBuilder) -> Result<FieldResult> {
        loop {
            match self.consume_char() {
                Some(c) if c.is_whitespace() => continue,
                Some(c) if self.is_field_separator(c) => return Ok(FieldResult::some(fb)),
                Some(c) => {
                    // it's not whitespace or a comma
                    return Err(self.parse_error(format!(
                        "Invalid trailing character after quoted string: {c}"
                    )));
                }
                None => return Ok(FieldResult::last(fb)),
            }
        }
    }

    fn consume_char(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            if is_record_terminator(c) {
                self.source_line += 1;
                self.source_offset = 0;
            } else {
                self.source_offset += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn parse_error<S: Into<String>>(&mut self, message: S) -> Error {
        Error::ParseError {
            bad_input: self.chars.clone().take(10).collect::<String>(),
            message: message.into(),
            position: self.into(),
        }
    }
}

/// Parse a given string and return the `Cell`s.
///
/// # Errors
///
/// Will return an `Error::ParseError` if it is unable to parse the given `input`
pub fn parse<'a>(input: &'a str, config: &'a Config) -> Result<Records> {
    Parser::new(input, config).parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn test_parse(s: &str) -> Records {
        parse(s, &Config::default()).unwrap()
    }

    #[test]
    fn source_position_from_parser() {
        let config = Config::default();
        let mut parser = Parser::new("foo", &config);
        parser.parse().unwrap();
        let source_position: SourcePosition = (&mut parser).into();

        assert_eq!(source_position.line_number, 0);
        assert_eq!(source_position.line_offset, 2);
    }

    #[test]
    fn source_position_from_parser_lines_above() {
        let config = Config {
            lines_above: 100,
            ..Config::default()
        };
        let mut parser = Parser::new("foo", &config);
        parser.parse().unwrap();
        let source_position: SourcePosition = (&mut parser).into();

        assert_eq!(source_position.line_number, 100);
        assert_eq!(source_position.line_offset, 2);
    }

    #[test]
    fn parse_simple() {
        let cells = test_parse("foo,bar,baz");

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);

        let cell = &cells[0][0];
        assert_eq!(cell.value, "foo");
        assert_eq!(cell.address, (0, 0).into());
        assert_eq!(cell.positions[0].line_offset, 0);
        assert_eq!(cell.positions[1].line_offset, 1);
        assert_eq!(cell.positions[2].line_offset, 2);
        assert_eq!(cell.positions[0].line_number, 0);
        assert_eq!(cell.positions[1].line_number, 0);
        assert_eq!(cell.positions[2].line_number, 0);

        let cell = &cells[0][1];
        assert_eq!(cell.value, "bar");
        assert_eq!(cell.address, (1, 0).into());
        assert_eq!(cell.positions[0].line_offset, 4);
        assert_eq!(cell.positions[1].line_offset, 5);
        assert_eq!(cell.positions[2].line_offset, 6);
        assert_eq!(cell.positions[0].line_number, 0);
        assert_eq!(cell.positions[1].line_number, 0);
        assert_eq!(cell.positions[2].line_number, 0);

        let cell = &cells[0][2];
        assert_eq!(cell.value, "baz");
        assert_eq!(cell.address, (2, 0).into());
        assert_eq!(cell.positions[0].line_offset, 8);
        assert_eq!(cell.positions[1].line_offset, 9);
        assert_eq!(cell.positions[2].line_offset, 10);
        assert_eq!(cell.positions[0].line_number, 0);
        assert_eq!(cell.positions[1].line_number, 0);
        assert_eq!(cell.positions[2].line_number, 0);
    }

    #[test]
    fn parse_empty_cell() {
        let cells = test_parse("foo,,baz");

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][0].address, (0, 0).into());
        assert_eq!(cells[0][1].value, "");
        assert_eq!(cells[0][1].address, (1, 0).into());
        assert_eq!(cells[0][2].value, "baz");
        assert_eq!(cells[0][2].address, (2, 0).into());
    }

    #[test]
    fn parse_multiple_lines() {
        let cells = test_parse("foo,bar,baz\nfoos,bars,bazs");

        assert_eq!(cells.len(), 2);
        assert_eq!(cells[0].len(), 3);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][1].value, "bar");
        assert_eq!(cells[0][2].value, "baz");
        assert_eq!(cells[1][0].value, "foos");
        assert_eq!(cells[1][1].value, "bars");
        assert_eq!(cells[1][2].value, "bazs");
    }

    #[test]
    fn parse_spaces() {
        let cells = test_parse("   foo ,    bar   ,one two three");

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);

        let cell = &cells[0][0];
        assert_eq!(cell.value, "foo");
        assert_eq!(cell.positions[0].line_offset, 3);
        assert_eq!(cell.positions[1].line_offset, 4);
        assert_eq!(cell.positions[2].line_offset, 5);

        let cell = &cells[0][1];
        assert_eq!(cell.value, "bar");
        assert_eq!(cell.positions[0].line_offset, 12);
        assert_eq!(cell.positions[1].line_offset, 13);
        assert_eq!(cell.positions[2].line_offset, 14);

        let cell = &cells[0][2];
        assert_eq!(cell.value, "one two three");
        assert_eq!(cell.positions[0].line_offset, 19);
        assert_eq!(cell.positions[1].line_offset, 20);
        assert_eq!(cell.positions[2].line_offset, 21);
    }

    #[test]
    fn parse_trailing_newline() {
        let cells = test_parse("foo\nbar\n");

        assert_eq!(cells.len(), 2);
        assert_eq!(cells[0][0].address, (0, 0).into());
        assert_eq!(cells[1][0].address, (0, 1).into());
    }

    #[test]
    fn parse_leading_newline() {
        let cells = test_parse("\nfoo\nbar\n");

        assert_eq!(cells.len(), 2);
        assert_eq!(cells[0][0].address, (0, 0).into());
        assert_eq!(cells[1][0].address, (0, 1).into());
    }

    #[test]
    fn parse_windows_newline() {
        let cells = test_parse("foo\r\nbar\r\nbaz\r\n");

        assert_eq!(cells.len(), 3);
    }

    #[test]
    fn parse_quoted() {
        let cells = test_parse(r#""this, is, a, quoted, sentence",bar"#);

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 2);
        assert_eq!(cells[0][0].value, "this, is, a, quoted, sentence");
        assert_eq!(cells[0][1].value, "bar");
    }

    #[test]
    fn parse_quoted_newline() {
        let cells = test_parse("\"this field \n has a newline\",bar");

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 2);
        assert_eq!(cells[0][0].value, "this field \n has a newline");
    }

    #[test]
    fn parse_quoted_quote() {
        let cells = test_parse("\"this field has a quote \"\"\",bar");

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 2);
        assert_eq!(cells[0][0].value, "this field has a quote \"");
    }

    #[test]
    fn parse_comment() {
        let cells = test_parse("# this is a comment\nfoo,bar\n# another comment");

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 2);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][1].value, "bar");
    }

    #[test]
    fn parse_multiline_field() {
        let cells = test_parse("this \\\nspans \\\nmultiple lines");

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 1);
        assert_eq!(cells[0][0].value, "this spans multiple lines");
    }

    #[test]
    fn parse_trailing_comma_newline() {
        let cells = test_parse("foo  ,\n");

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 2);
    }

    #[test]
    fn parse_trailing_comma_no_newline() {
        let cells = test_parse(
            r"[[var=a1]]A1,foo,bar
![[f=10]],bar,=var2
foo
[[l]]test,
![[l]]test1,test2,test3,",
        );

        assert_eq!(cells.len(), 5);
    }

    #[test]
    fn parse_ending_quote() {
        let cells = test_parse("\"=profit\" ,\"=fees\"");

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 2);
    }
}
