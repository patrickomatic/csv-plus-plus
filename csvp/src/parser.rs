//! # `csv+` Parser
//!
//! A recursive descent, single-threaded CSV parser. This aims to be a CSV-compliant
//! implementation of RFC 4180, however there are some additional features:
//!
//! * Multiline-field support
//! * Record comments. (A `#` at the beginning of the line comments out the whole record.
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
use super::{Config, Error, Field, PartialField, Record, Records, Result, SourcePosition};
use std::iter;
use std::str;

#[derive(Debug)]
struct Parser<'a> {
    config: &'a Config,
    chars: iter::Peekable<str::Chars<'a>>,
    source_line: usize,
    source_offset: usize,
}

enum FieldResult {
    Some(Field),
    Last(Field),
    Eof,
}

enum RecordResult {
    Comment,
    Eof,
    Some(Record),
}

impl From<&mut Parser<'_>> for SourcePosition {
    fn from(p: &mut Parser) -> Self {
        SourcePosition::new(p.source_offset, p.source_line)
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
            source_offset: 0,
        }
    }

    pub(super) fn parse(&mut self) -> Result<Records> {
        let mut records = vec![];
        loop {
            match self.parse_record()? {
                RecordResult::Comment => continue,
                RecordResult::Some(r) => records.push(r),
                RecordResult::Eof => break,
            }
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

    fn parse_record(&mut self) -> Result<RecordResult> {
        if let Some('#') = self.chars.peek() {
            self.consume_and_ignore_line();
            return Ok(RecordResult::Comment);
        }

        let mut fields = vec![];
        loop {
            match self.parse_field(PartialField::default())? {
                FieldResult::Some(f) => fields.push(f),
                FieldResult::Last(f) => {
                    fields.push(f);
                    break;
                }
                FieldResult::Eof => return Ok(RecordResult::Eof),
            }
        }

        Ok(RecordResult::Some(fields))
    }

    fn parse_field(&mut self, mut pf: PartialField) -> Result<FieldResult> {
        match self.consume_char() {
            Some(c) if self.is_field_separator(c) => Ok(FieldResult::Some(pf.into())),
            Some(c) if is_record_terminator(c) => Ok(FieldResult::Eof),
            Some('\\') if self.next_is_newline() => {
                // consume the newline then continue parsing this field
                self.consume_char();
                self.parse_field(pf)
            }
            Some(c) if c.is_whitespace() => self.parse_field(pf),
            Some('"') => Ok(self.parse_quoted_field(pf, false)?),
            Some(c) => {
                pf.push(c, self.into());
                Ok(self.parse_unquoted_field(pf)?)
            }
            None => Ok(FieldResult::Eof),
        }
    }

    fn parse_unquoted_field(&mut self, mut pf: PartialField) -> Result<FieldResult> {
        match self.consume_char() {
            Some('\\') if self.next_is_newline() => {
                // consume the newline then continue parsing this field
                self.consume_char();
                self.parse_unquoted_field(pf)
            }
            Some(c) if self.is_field_separator(c) => Ok(FieldResult::Some(pf.into())),
            Some(c) if is_record_terminator(c) => Ok(FieldResult::Last(pf.into())),
            Some(c) => {
                pf.push(c, self.into());
                self.parse_unquoted_field(pf)
            }
            None => Ok(FieldResult::Last(pf.into())),
        }
    }

    fn parse_quoted_field(
        &mut self,
        mut pf: PartialField,
        escape_mode: bool,
    ) -> Result<FieldResult> {
        let c = self.consume_char();
        if escape_mode {
            if let Some(c) = c {
                pf.push(c, self.into());
                return self.parse_quoted_field(pf, false);
            }

            return Err(Error::ParseError {
                message: "Expected a quoted character but got EOF".to_string(),
                position: self.into(),
            });
        }

        match c {
            Some('"') => {
                // is it two quotes in a row? if so it's a quoted quote
                if let Some('"') = self.chars.peek() {
                    self.parse_quoted_field(pf, true)
                } else {
                    // otherwise it's a terminating quote. we need to also make sure
                    // we consume any trailing spaces and make sure there is a ','
                    self.parse_rest_of_quoted_field()?;

                    Ok(FieldResult::Some(pf.into()))
                }
            }
            Some(c) => {
                pf.push(c, self.into());
                self.parse_quoted_field(pf, false)
            }
            None => Ok(FieldResult::Some(pf.into())),
        }
    }

    /// At this point we've already seen the beginning and ending quotes and have consumed a field.
    /// But we need to consume any trailing spaces as well as the terminating comma.  Or throw an
    /// error if it'd not there.
    fn parse_rest_of_quoted_field(&mut self) -> Result<()> {
        loop {
            match self.consume_char() {
                Some(c) if c.is_whitespace() => continue,
                Some(',') => return Ok(()),
                Some(c) => {
                    // it's not whitespace or a comma
                    return Err(Error::ParseError {
                        message: format!("Invalid trailing character after quoted string: {c}"),
                        position: self.into(),
                    });
                }
                None => {
                    // got EOF but they never terminated the field...
                    return Err(Error::ParseError {
                        message: "Expected a quoted character but got EOF".to_string(),
                        position: self.into(),
                    });
                }
            }
        }
    }

    fn consume_char(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            if c == '\n' {
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
}

/// Parse a given string and return the `Cell`s.
/// # Errors
///
/// Will return an `Error::ParseError` if it is unable to parse the given `input`
pub fn parse<'a>(input: &'a str, config: &'a Config) -> Result<Records> {
    Parser::new(input, config).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(s: &str) -> Records {
        parse(s, &Config::default()).unwrap()
    }

    #[test]
    fn parse_simple() {
        let cells = test_parse("foo,bar,baz");
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][1].value, "bar");
        assert_eq!(cells[0][2].value, "baz");
    }

    #[test]
    fn parse_empty_cell() {
        let cells = test_parse("foo,,baz");
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][1].value, "");
        assert_eq!(cells[0][2].value, "baz");
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
        let cells = test_parse("   foo ,    bar   ,baz");
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][1].value, "bar");
        assert_eq!(cells[0][2].value, "baz");
    }

    #[test]
    fn parse_trailing_newline() {
        let cells = test_parse("foo\nbar\n");
        assert_eq!(cells.len(), 2);
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
}
