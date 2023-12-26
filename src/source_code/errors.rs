//! It's common that we want to create errors with references to data that the `SourceCode` owns.
//! In which case it makes sense to add some helper functions to do that
use super::{CharOffset, LineNumber, SourceCode};
use crate::error::{BadInput, EvalError, ParseError};
use crate::Error;
use colored::Colorize;
use std::cmp;

// how many lines above (and below) we'll show as context when highlighting error messages
const LINES_IN_ERROR_CONTEXT: LineNumber = 3;

impl SourceCode {
    pub(crate) fn code_syntax_error(&self, parse_error: ParseError) -> Error {
        Error::CodeSyntaxError {
            filename: self.filename.clone(),
            parse_error: Box::new(parse_error),
        }
    }

    pub(crate) fn cell_syntax_error(
        &self,
        parse_error: ParseError,
        position: a1_notation::Address,
    ) -> Error {
        Error::CellSyntaxError {
            filename: self.filename.clone(),
            parse_error: Box::new(parse_error),
            position,
        }
    }

    pub(crate) fn eval_error(
        &self,
        eval_error: EvalError,
        position: Option<a1_notation::Address>,
    ) -> Error {
        Error::EvalError {
            filename: self.filename.clone(),
            eval_error: Box::new(eval_error),
            position,
        }
    }

    pub(crate) fn parse_error<S: Into<String>>(
        &self,
        bad_input: impl BadInput,
        message: S,
    ) -> ParseError {
        let line_number = bad_input.line_number();
        let line_offset = bad_input.line_offset();
        let highlighted_lines = self.highlight_line(line_number, line_offset);

        ParseError {
            bad_input: bad_input.to_string(),
            highlighted_lines,
            message: message.into(),
            line_number,
            line_offset,
            possible_values: None,
        }
    }

    pub(crate) fn parse_error_with_possible_values<S: Into<String>>(
        &self,
        bad_input: impl BadInput,
        message: S,
        // TODO: make this a slice
        possible_values: Vec<String>,
    ) -> ParseError {
        let mut parse_error = self.parse_error(bad_input, message);
        parse_error.possible_values = Some(possible_values);
        parse_error
    }

    /// Given a line number and character offset, return an array of `String`s that can be rendered
    /// for a friendly message for debugging (that highlights the line and character in question).
    fn highlight_line(&self, line_number: LineNumber, line_offset: CharOffset) -> Vec<String> {
        let lines = self
            .original
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        // are they requesting a line totally outside of the range?
        if line_number >= lines.len() {
            return vec![];
        }

        let start_index = line_number.saturating_sub(LINES_IN_ERROR_CONTEXT);
        let end_index = cmp::min(lines.len(), line_number + LINES_IN_ERROR_CONTEXT + 1);

        // start with 3 lines before the highlighted line
        let mut lines_out: Vec<colored::ColoredString> = lines[start_index..line_number]
            .iter()
            .map(|l| l.dimmed())
            .collect();

        // and the highlighted line in bright red
        lines_out.push(lines[line_number].bright_red());

        // save the number of this line because we want to skip line-numbering it below
        let skip_numbering_on = lines_out.len();

        // draw something like this to highlight it:
        // ```
        //      foo!
        // --------^
        // ```
        lines_out.push(format!("{}^", "-".repeat(line_offset)).yellow());

        // and 3 lines after
        lines_out.append(
            &mut lines[(line_number + 1)..end_index]
                .iter()
                .map(|l| l.dimmed())
                .collect(),
        );

        // now format each line with line numbers
        let longest_line_number = (line_number + LINES_IN_ERROR_CONTEXT).to_string().len();
        let mut line_count = line_number.saturating_sub(LINES_IN_ERROR_CONTEXT);

        // now iterate over it and apply lines numbers like `XX: some_code( ...` where XX is the
        // line number
        lines_out
            .iter()
            .enumerate()
            .map(|(i, line)| {
                // don't increment the line *after* the line we're highlighting.  because it's the
                // ----^ thing and it doesn't correspond to a source code row, it's highlighting the
                // text above it
                if i == skip_numbering_on {
                    format!(" {: <width$}: {line}", " ", width = longest_line_number)
                } else {
                    line_count += 1;
                    format!(
                        " {: <width$}: {line}",
                        line_count,
                        width = longest_line_number
                    )
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlight_line() {
        let source_code = SourceCode::new(
            "
# A comment

var := 1
other_var := 42

something {
    foo: bar
}
---
foo,bar,baz
            ",
            "test.csvpp",
        )
        .unwrap();

        let highlighted_lines = source_code.highlight_line(7, 5);
        assert_eq!(highlighted_lines.len(), 8);
        assert!(highlighted_lines[3].contains("foo: bar"));
        assert!(highlighted_lines[4].contains("----^"));
    }

    #[test]
    fn highlight_line_at_top() {
        let source_code = SourceCode::new(
            "# A comment

var := 1
other_var := 42

something {
    foo: bar
}
---
foo,bar,baz
            ",
            "test.csvpp",
        )
        .unwrap();

        let highlighted_lines = source_code.highlight_line(0, 5);
        assert_eq!(highlighted_lines.len(), 5);
        assert!(highlighted_lines[0].contains("# A comment"));
        assert!(highlighted_lines[4].contains("other_var"));
    }
}
