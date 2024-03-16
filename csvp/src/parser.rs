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
use super::{Config, Error, Field, Record, Records, Result, SourcePosition};
use std::iter;
use std::str;

#[derive(Debug)]
struct Parser<'a> {
    config: &'a Config,
    chars: iter::Peekable<str::Chars<'a>>,
    source_line: usize,
    source_offset: usize,
}

#[derive(Debug, Default)]
struct PartialField {
    field: String,
    positions: Vec<SourcePosition>,
}

impl From<PartialField> for Field {
    fn from(pf: PartialField) -> Self {
        Field {
            value: pf.field.trim().to_string(),
            positions: pf.positions,
        }
    }
}

// XXX better name... position is already kinda used
enum FieldPosition {
    Some(Field),
    Last(Field),
    None,
}

impl From<&mut Parser<'_>> for SourcePosition {
    fn from(p: &mut Parser) -> Self {
        SourcePosition::new(p.source_offset, p.source_line)
    }
}

impl PartialField {
    fn push(&mut self, c: char, position: SourcePosition) {
        self.field.push(c);
        self.positions.push(position);
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
        while let Some(record) = self.parse_record()? {
            records.push(record);
        }

        Ok(records)
    }

    fn is_field_separator(&self, c: char) -> bool {
        c == self.config.separator
    }

    fn parse_record(&mut self) -> Result<Option<Record>> {
        let mut fields = vec![];
        loop {
            match self.parse_field(PartialField::default())? {
                FieldPosition::Some(f) => fields.push(f),
                FieldPosition::Last(f) => {
                    fields.push(f);
                    break;
                }
                FieldPosition::None => break,
            }
        }

        if fields.is_empty() {
            return Ok(None);
        }

        Ok(Some(fields))
    }

    fn parse_field(&mut self, mut pf: PartialField) -> Result<FieldPosition> {
        match self.consume_char() {
            Some(c) if self.is_field_separator(c) => Ok(FieldPosition::Some(pf.into())),
            Some(c) if is_record_terminator(c) => Ok(FieldPosition::None),
            Some(c) if c.is_whitespace() => self.parse_field(pf),
            Some('"') => Ok(self.parse_quoted_field(pf, false)?),
            Some(c) => {
                pf.push(c, self.into());
                Ok(self.parse_unquoted_field(pf)?)
            }
            None => Ok(FieldPosition::None),
        }
    }

    fn parse_unquoted_field(&mut self, mut pf: PartialField) -> Result<FieldPosition> {
        match self.consume_char() {
            Some(c) if self.is_field_separator(c) => Ok(FieldPosition::Some(pf.into())),
            Some(c) if is_record_terminator(c) => Ok(FieldPosition::Last(pf.into())),
            Some(c) => {
                pf.push(c, self.into());
                self.parse_unquoted_field(pf)
            }
            None => Ok(FieldPosition::Last(pf.into())),
        }
    }

    fn parse_quoted_field(
        &mut self,
        mut pf: PartialField,
        escape_mode: bool,
    ) -> Result<FieldPosition> {
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

                    Ok(FieldPosition::Some(pf.into()))
                }
            }
            Some(c) => {
                pf.push(c, self.into());
                self.parse_quoted_field(pf, false)
            }
            None => Ok(FieldPosition::Some(pf.into())),
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

    #[test]
    fn parse_simple() {
        let cells = parse("foo,bar,baz", &Config::default()).unwrap();

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][1].value, "bar");
        assert_eq!(cells[0][2].value, "baz");
    }

    #[test]
    fn parse_empty_cell() {
        let cells = parse("foo,,baz", &Config::default()).unwrap();

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][1].value, "");
        assert_eq!(cells[0][2].value, "baz");
    }

    #[test]
    fn parse_multiple_lines() {
        let cells = parse("foo,bar,baz\nfoos,bars,bazs", &Config::default()).unwrap();

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
        let cells = parse("   foo ,    bar   ,baz", &Config::default()).unwrap();

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][1].value, "bar");
        assert_eq!(cells[0][2].value, "baz");
    }

    #[test]
    fn parse_trailing_newline() {
        let cells = parse("foo\nbar\n", &Config::default()).unwrap();

        assert_eq!(cells.len(), 2);
    }

    #[test]
    fn parse_windows_newline() {
        let cells = parse("foo\r\nbar\r\nbaz\r\n", &Config::default()).unwrap();

        assert_eq!(cells.len(), 3);
    }

    #[test]
    fn parse_quoted() {
        let cells = parse(r#""this, is, a, quoted, sentence",bar"#, &Config::default()).unwrap();

        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 2);
        assert_eq!(cells[0][0].value, "this, is, a, quoted, sentence");
        assert_eq!(cells[0][1].value, "bar");
    }
}
