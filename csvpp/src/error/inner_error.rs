//! # InnerError
//! `InnerError`s are errors that lack an outer context such as `line_number` or `index: A1`.
//! They should be caught and wrapped into an `Error`.
use std::error;
use std::fmt;

#[derive(Clone, Debug)]
pub enum InnerError {
    BadInput {
        bad_input: String,
        message: String,
    },
    BadInputWithPossibilities {
        message: String,
        bad_input: String,
        possible_values: String,
    },
    RgbSyntaxError {
        bad_input: String,
        message: String,
    },
}

impl InnerError {
    pub fn bad_input(bad_input: &str, message: &str) -> Self {
        Self::BadInput { 
            bad_input: bad_input.to_owned(),
            message: message.to_owned(),
        }
    }

    pub fn bad_input_with_possibilities(bad_input: &str, message: &str, possible_values: &str) -> Self {
        Self::BadInputWithPossibilities { 
            bad_input: bad_input.to_owned(),
            message: message.to_owned(),
            possible_values: possible_values.to_owned(),
        }
    }

    pub fn rgb_syntax_error(bad_input: &str, message: &str) -> Self {
        Self::RgbSyntaxError {
            bad_input: bad_input.to_owned(),
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for InnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: anything else to do when in verbose mode?
        match self {
            Self::BadInput { bad_input, message } => {
                writeln!(f, "{}", message)?;
                writeln!(f, "bad input: {}", bad_input)
            },
            Self::BadInputWithPossibilities { message, bad_input, possible_values } => {
                writeln!(f, "{}", message)?;
                writeln!(f, "bad input: {}", bad_input)?;
                write!(f, "possible values: {}", possible_values)
            },
            Self::RgbSyntaxError { bad_input, message } => {
                writeln!(f, "Error parsing RGB value: {}", message)?;
                write!(f, "bad input: {}", bad_input)
            },
        }
    }
}

impl From<a1_notation::Error> for InnerError {
    fn from(err: a1_notation::Error) -> Self {
        match err {
            a1_notation::Error::A1BuilderError(m) =>
                InnerError::bad_input(&m, &format!("Error building parsing A1 format: {}", m)),
            a1_notation::Error::A1ParseError { bad_input, message } =>
                InnerError::bad_input(&bad_input, &message),
        }
    }
}

impl error::Error for InnerError {}

#[cfg(test)]
mod tests {
    /* TODO
    use super::*;

    #[test]
    fn display_cell_syntax_error() {
        let message = Error::CellSyntaxError {
            index: a1_notation::A1::builder().xy(1, 5).build().unwrap(),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->B6: foo", message.to_string());
    }

    #[test]
    fn display_code_syntax_error() {
        let message = Error::CodeSyntaxError {
            line_number: 1,
            message: "foo".to_string(),
            bad_input: "bar".to_string(),
        };

        assert_eq!("1: foo\nbad input: bar", message.to_string());
    }

    #[test]
    fn display_modifier_syntax_error() {
        let message = Error::ModifierSyntaxError {
            bad_input: "bad_input".to_string(),
            index: a1_notation::A1::builder().xy(0, 1).build().unwrap(),
            message: "foo".to_string(),
        };

        assert_eq!("Cell->A2: foo\nbad input: bad_input", message.to_string());
    }
    */
}
