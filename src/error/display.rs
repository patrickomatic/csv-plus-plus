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
                writeln!(f, "{message}")
            }

            Self::ObjectWriteError { filename, message } => {
                writeln!(f, "Error writing object file {}", filename.display())?;
                writeln!(f, "{message}")
            }

            Self::SourceCodeError { filename, message } => {
                writeln!(f, "Error reading source {}", filename.display())?;
                writeln!(f, "{message}")
            }

            Self::TargetWriteError { output, message } => {
                writeln!(f, "Error writing to {output}")?;
                writeln!(f, "{message}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Output, ParseError};
    use super::*;
    use std::path;

    fn build_parse_error() -> ParseError {
        ParseError {
            bad_input: "bar".to_string(),
            message: "it should be foo".to_string(),
            line_number: 3,
            line_offset: 5,
            possible_values: None,
            highlighted_lines: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
        }
    }

    #[test]
    fn display_cell_syntax_error() {
        let message = Error::CellSyntaxError {
            filename: path::PathBuf::from("a_file.csvpp"),
            position: a1_notation::Address::new(1, 5),
            parse_error: Box::new(build_parse_error()),
        };

        assert_eq!(
            message.to_string(),
            "Syntax error in cell B6 of a_file.csvpp
On line 4 it should be foo but saw bar

foo
bar
baz

",
        );
    }

    #[test]
    fn display_code_syntax_error() {
        let message = Error::CodeSyntaxError {
            filename: path::PathBuf::from("a_file.csvpp"),
            parse_error: Box::new(build_parse_error()),
        };

        assert_eq!(
            message.to_string(),
            "Syntax error in code section of a_file.csvpp
On line 4 it should be foo but saw bar

foo
bar
baz

"
        );
    }

    #[test]
    fn display_eval_error() {
        let message = Error::EvalError {
            filename: path::PathBuf::from("a_file.csvpp"),
            message: "foo".to_string(),
            position: a1_notation::Address::new(2, 2),
        };

        assert_eq!(
            message.to_string(),
            "Error evaluating formula in cell C3 (2, 2) of a_file.csvpp
foo
"
        );
    }

    #[test]
    fn display_modifier_syntax_error() {
        let message = Error::ModifierSyntaxError {
            filename: path::PathBuf::from("a_file.csvpp"),
            position: a1_notation::Address::new(0, 1),
            parse_error: Box::new(build_parse_error()),
        };

        assert_eq!(
            message.to_string(),
            "Invalid modifier definition in cell A2 (0, 1) of a_file.csvpp
On line 4 it should be foo but saw bar

foo
bar
baz

"
        );
    }

    #[test]
    fn display_init_error() {
        let message = Error::InitError("foo".to_string());

        assert_eq!("foo\n", message.to_string());
    }

    #[test]
    fn display_object_write_error() {
        let message = Error::ObjectWriteError {
            filename: path::PathBuf::from("bar.xlsx"),
            message: "foo".to_string(),
        };

        assert_eq!(
            message.to_string(),
            "Error writing object file bar.xlsx\nfoo\n",
        );
    }

    #[test]
    fn display_source_code_error() {
        let message = Error::SourceCodeError {
            filename: path::PathBuf::from("a_file.csvpp"),
            message: "foo".to_string(),
        };

        assert_eq!(
            message.to_string(),
            "Error reading source a_file.csvpp\nfoo\n",
        );
    }

    #[test]
    fn display_target_write_error() {
        let message = Error::TargetWriteError {
            output: Output::Excel(path::PathBuf::from("foo.xlsx")),
            message: "foo".to_string(),
        };

        assert_eq!(message.to_string(), "Error writing to foo.xlsx\nfoo\n",);
    }
}
