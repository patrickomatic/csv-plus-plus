use super::{Cell, Error, ParsedCells, Result, SourcePosition};

#[derive(Debug)]
pub struct Config {
    pub(super) separator: char,
}

impl Default for Config {
    fn default() -> Self {
        Self { separator: ',' }
    }
}

/// Parse a given string and return the `Cell`s.
///
/// # Errors
///
/// Will return an `Error::ParseError` if it is unable to parse the given `input`
pub fn parse(input: &str, config: &Config) -> Result<ParsedCells> {
    // XXX turn into a reduce
    let mut rows = vec![];
    let mut row = vec![];
    let mut current_value = String::new();
    let mut line_number = 0;
    let mut line_offset = 0;
    let mut in_quote = false;
    let mut offset_positions = vec![];

    for c in input.chars() {
        let position = SourcePosition::new(line_offset, line_number);
        offset_positions.push(position);

        if in_quote {
        } else if c == '"' {
            in_quote = true;
            if in_quote {
                //
            } else {
                in_quote = true;
            }
        } else if c == config.separator {
            row.push(Cell {
                value: current_value.trim().to_string(),
                offset_positions,
            });
            current_value = String::new();
            offset_positions = vec![];
        } else if c == '\r' {
            // for windows the next line will be the `\n` and we'll advance the row. for any other
            // systems we don't care about carriage returns
            continue;
        } else if c == '\n' {
            if in_quote {
                return Err(Error::ParseError {
                    message: "Unterminated quoted cell".to_string(),
                    position,
                });
            }
            row.push(Cell {
                value: current_value.trim().to_string(),
                offset_positions,
            });

            // XXX needs to handle if it's a wrapped row
            rows.push(row);
            line_number += 1;

            // reset everything
            row = vec![];
            line_offset = 0;
            in_quote = false;
            current_value = String::new();
            offset_positions = vec![];

            continue;
        } else {
            current_value.push(c);
        }

        line_offset += 1;
    }

    if !current_value.is_empty() {
        row.push(Cell {
            value: current_value,
            // XXX
            offset_positions: vec![],
        });
    }

    if !row.is_empty() {
        rows.push(row);
    }

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let cells = parse("foo,bar,baz", &Config::default()).unwrap();

        dbg!(&cells);
        assert_eq!(cells.len(), 1);
        assert_eq!(cells[0].len(), 3);
        assert_eq!(cells[0][0].value, "foo");
        assert_eq!(cells[0][1].value, "bar");
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

        assert_eq!(cells.len(), 0);
        assert_eq!(cells[0].len(), 2);
        assert_eq!(cells[0][0].value, "this, is, a, quoted, sentence");
        assert_eq!(cells[0][1].value, "bar");
    }
}
