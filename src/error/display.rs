use super::Error;
use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CellSyntaxError {
                filename,
                parse_error,
                position,
            } => {
                writeln!(
                    f,
                    "Syntax error in cell {position} of {}",
                    filename.display()
                )?;
                writeln!(f, "{parse_error}")
            }

            Self::CodeSyntaxError {
                parse_error,
                filename,
            } => {
                writeln!(f, "Syntax error in code section of {}", filename.display())?;
                writeln!(f, "{parse_error}")
            }

            Self::EvalError {
                message,
                position,
                filename,
            } => {
                writeln!(
                    f,
                    "Error evaluating formula in cell {position} ({}, {}) of {}",
                    position.column.x,
                    position.row.y,
                    filename.display()
                )?;
                writeln!(f, "{message}")
            }

            Self::ModifierSyntaxError {
                position,
                parse_error,
                filename,
            } => {
                writeln!(
                    f,
                    "Invalid modifier definition in cell {position} ({}, {}) of {}",
                    position.column.x,
                    position.row.y,
                    filename.display()
                )?;
                writeln!(f, "{parse_error}")
            }

            Self::InitError(message) => {
                writeln!(f, "Error initializing: {message}")
            }

            Self::ObjectWriteError { filename, message } => {
                writeln!(
                    f,
                    "Error writing object file {}: {message}",
                    filename.display()
                )
            }

            Self::SourceCodeError { filename, message } => {
                writeln!(f, "Error reading source {}: {message}", filename.display())
            }

            Self::TargetWriteError { output, message } => {
                writeln!(f, "Error writing to {output}: {message}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::super::{Output, ParseError};
    use super::*;
    use std::path;

    #[test]
    fn display_cell_syntax_error() {
        let message = Error::CellSyntaxError {
            line_number: 8,
            position: a1_notation::Address::new(1, 5),
            parse_error: Box::new(ParseError::BadInput {
                bad_input: "foo".to_string(),
                message: "You did a foo".to_string(),
            }),
        };

        assert_eq!(
            "Syntax error in cell B6 on line 8
    You did a foo
    bad input: foo
    ",
            message.to_string()
        );
    }

    #[test]
    fn display_code_syntax_error() {
        let message = Error::CodeSyntaxError {
            position: 2,
            line_number: 1,
            message: "foo".to_string(),
            highlighted_lines: vec!["foo".to_string(), "bar".to_string()],
        };

        assert_eq!(
            "Syntax error on line 1: foo
    foo
    bar
    ",
            message.to_string()
        );
    }

    #[test]
    fn display_eval_error() {
        let message = Error::EvalError {
            position: a1_notation::Address::new(2, 2),
            line_number: 1,
            message: "foo".to_string(),
        };

        assert_eq!(
            "Error evaluating formula in cell C3 on line 1\nfoo\n",
            message.to_string()
        );
    }

    #[test]
    fn display_modifier_syntax_error() {
        let message = Error::ModifierSyntaxError {
            line_number: 5,
            position: a1_notation::Address::new(0, 1),
            parse_error: Box::new(ParseError::BadInputWithPossibilities {
                bad_input: "foo".to_string(),
                message: "You did a foo".to_string(),
                possible_values: "bar | baz".to_string(),
            }),
        };

        assert_eq!(
            "Invalid modifier definition in cell A2 on line 5
    You did a foo
    bad input: foo
    possible values: bar | baz
    ",
            message.to_string()
        );
    }

    #[test]
    fn display_init_error() {
        let message = Error::InitError("foo".to_string());

        assert_eq!("Error initializing: foo\n", message.to_string());
    }

    #[test]
    fn display_object_write_error() {
        let message = Error::ObjectWriteError {
            filename: path::PathBuf::from("bar.xlsx"),
            message: "foo".to_string(),
        };

        assert_eq!(
            "Error writing object file bar.xlsx: foo\n",
            message.to_string()
        );
    }

    #[test]
    fn display_source_code_error() {
        let message = Error::SourceCodeError {
            filename: path::PathBuf::from("a_file.csvpp"),
            message: "foo".to_string(),
        };

        assert_eq!(
            "Error reading source a_file.csvpp: foo\n",
            message.to_string()
        );
    }

    #[test]
    fn display_target_write_error() {
        let message = Error::TargetWriteError {
            output: Output::Excel(path::PathBuf::from("foo.csvpp")),
            message: "foo".to_string(),
        };

        assert_eq!(
            "Error writing to Excel: foo.csvpp: foo\n",
            message.to_string()
        );
    }
    */
}
