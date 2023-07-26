//! Error handling functions
use std::error;
use std::fmt;
use std::path::PathBuf;
use crate::Output;

#[derive(Clone, Debug)]
pub enum Error {
    /// A syntax error in a formula in a cell.
    CellSyntaxError {
        // TODO
        // highlighted_lines: Vec<String>,
        line_number: usize,
        message: String,
        position: a1_notation::A1,
    },

    /// A syntax error in the code section.
    CodeSyntaxError {
        highlighted_lines: Vec<String>,
        line_number: usize,
        message: String,
        position: usize,
    },
    
    /// An error encountered when evaluating the formulas in a cell.  For example if a builtin
    /// funciton is called with the wrong number of arguments.
    EvalError {
        // TODO
        // highlighted_lines: Vec<String>,
        line_number: usize,
        message: String,
        position: a1_notation::A1,
    },

    /// An error while building the runtime or reading the source code.  These are typically not
    /// due to user error.
    InitError(String),

    /// A syntax error encountered while parsing the modifiers of a cell.
    ModifierSyntaxError {
        // TODO
        // highlighted_lines: Vec<String>,
        message: String,
        position: a1_notation::A1,
        line_number: usize,
    },

    /// An error encountered while serializing the compiled template to an object file.
    ObjectWriteError {
        filename: PathBuf,
        message: String,
    },

    /// An error ecountered reaading or doing an initial parse of the source code.
    SourceCodeError {
        filename: PathBuf,
        message: String,
    },

    /// An error encountered while writing to the target.
    TargetWriteError {
        message: String,
        output: Output,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let highlighted_lines = match self {
            Self::CellSyntaxError { line_number, message, position } => {
                writeln!(f, "Syntax error in cell {} on line {}: {}", position, line_number, message)?;
                None
            },
            Self::CodeSyntaxError { line_number, message, highlighted_lines, .. } => {
                writeln!(f, "Syntax error on line {}: {}", line_number, message)?;
                Some(highlighted_lines)
            },
            Self::EvalError { message, line_number, position, .. } => {
                writeln!(f, "Error evaluating cell {} on line {}: {}", position, line_number, message)?;
                None
            },
            Self::ModifierSyntaxError { line_number, position, message } => {
                writeln!(f, "Invalid modifier in cell {} on line {}: {}", position, line_number, message)?;
                None
            },
            Self::InitError(message) => {
                writeln!(f, "Error initializing: {}", message)?;
                None
            },
            Self::ObjectWriteError { filename, message } => {
                writeln!(f, "Error writing object file {}: {}", filename.display(), message)?;
                None
            },
            Self::SourceCodeError { filename, message } => {
                writeln!(f, "Error reading source {}: {}", filename.display(), message)?;
                None
            },
            Self::TargetWriteError { output, message } => {
                writeln!(f, "Error writing to {}: {}", output, message)?;
                None
            },
        };

        if let Some(lines) = highlighted_lines {
            for line in lines {
                writeln!(f, "{}", line)?;
            }
        }

        Ok(())
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use std::path;
    use std::str::FromStr;
    use super::*;

    #[test]
    fn display_cell_syntax_error() {
        let message = Error::CellSyntaxError {
            line_number: 8,
            position: a1_notation::A1::builder().xy(1, 5).build().unwrap(),
            message: "foo".to_string(),
        };

        assert_eq!("Syntax error in cell B6 on line 8: foo\n", message.to_string());
    }

    #[test]
    fn display_code_syntax_error() {
        let message = Error::CodeSyntaxError {
            position: 2,
            line_number: 1,
            message: "foo".to_string(),
            highlighted_lines: vec!["foo".to_string(), "bar".to_string()],
        };

        assert_eq!("Syntax error on line 1: foo\nfoo\nbar\n", message.to_string());
    }

    #[test]
    fn display_eval_error() {
        let message = Error::EvalError {
            position: a1_notation::A1::from_str("C3").unwrap(),
            line_number: 1,
            message: "foo".to_string(),
        };

        assert_eq!("Error evaluating cell C3 on line 1: foo\n", message.to_string());
    }

    #[test]
    fn display_modifier_syntax_error() {
        let message = Error::ModifierSyntaxError {
            line_number: 5,
            position: a1_notation::A1::builder().xy(0, 1).build().unwrap(),
            message: "foo".to_string(),
        };

        assert_eq!("Invalid modifier in cell A2 on line 5: foo\n", message.to_string());
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

        assert_eq!("Error writing object file bar.xlsx: foo\n", message.to_string());
    }

    #[test]
    fn display_source_code_error() {
        let message = Error::SourceCodeError {
            filename: path::PathBuf::from("a_file.csvpp"),
            message: "foo".to_string(),
        };

        assert_eq!("Error reading source a_file.csvpp: foo\n", message.to_string());
    }

    #[test]
    fn display_target_write_error() {
        let message = Error::TargetWriteError {
            output: Output::Excel(path::PathBuf::from("foo.csvpp")),
            message: "foo".to_string(),
        };

        assert_eq!("Error writing to foo.csvpp: foo\n", message.to_string());
    }
}
