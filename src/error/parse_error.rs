//! # ParseError
//! `ParseError`s are errors that lack an outer context such as `line_number` or `index: A1`.
//! They should be caught and wrapped into an `Error`.
use crate::{CharOffset, LineNumber};
use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    /// The offending text
    pub bad_input: String,

    /// A set of lines that can be rendered to the user for helpful debugging.
    // TODO: it's kinda odd to me the way I did this, I would prefer to instead just store the line
    // number and offset and do all the calculations in the `fmt::Display` trait when it's rendered.
    // However it was a real struggle to do that because since the error must contain everything it
    // needs to render itself, that means it would need a reference to the `SourceCode` or
    // `Compiler`.  Both of which were really tough to do from a lifetime-perspective, but maybe
    // it's possible by someone smarter.
    pub highlighted_lines: Vec<String>,

    /// A message to the user why the input is unacceptable.  The `message` does not belong to the
    /// `BadInput` trait because conceptually, the bad input is the token and the message is the
    /// reason it's bad.  The token doesn't necessarily own the reason it was used in the wrong
    /// place.
    pub message: String,

    pub line_number: LineNumber,

    pub line_offset: CharOffset,

    pub possible_values: Option<Vec<String>>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ParseError {
            bad_input,
            highlighted_lines,
            line_number,
            message,
            possible_values,
            ..
        } = self;

        writeln!(
            f,
            "On line {} {message} but saw {bad_input}",
            line_number + 1
        )?;

        if let Some(pv) = possible_values {
            writeln!(f, "Possible values: {}", pv.join(" | "))?;
        }

        if !highlighted_lines.is_empty() {
            writeln!(f)?;
        }

        // `highlighted_lines` is already formatted for output
        for line in highlighted_lines {
            writeln!(f, "{line}")?;
        }

        Ok(())
    }
}

impl error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_parse_error() {
        let message = ParseError {
            bad_input: "bar".to_string(),
            message: "it should be foo".to_string(),
            line_number: 3,
            line_offset: 5,
            possible_values: None,
            highlighted_lines: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
        };

        assert_eq!(
            "On line 4 it should be foo but saw bar

foo
bar
baz
",
            message.to_string()
        );
    }

    #[test]
    fn display_bad_input_with_possibilities() {
        let message = ParseError {
            bad_input: "bar".to_string(),
            message: "it should be foo".to_string(),
            line_number: 3,
            line_offset: 5,
            possible_values: Some(vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
            ]),
            highlighted_lines: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
        };

        assert_eq!(
            "On line 4 it should be foo but saw bar
Possible values: one | two | three

foo
bar
baz
",
            message.to_string()
        );
    }
}
