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
        match self {
            Self::BadInput { bad_input, message } => {
                writeln!(f, "{}", message)?;
                write!(f, "bad input: {}", bad_input)
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
    use super::*;

    #[test]
    fn display_bad_input() {
        let message = InnerError::BadInput {
            bad_input: "bar".to_string(),
            message: "it should be foo".to_string(),
        };

        assert_eq!("it should be foo
bad input: bar", message.to_string());
    }

    #[test]
    fn display_bad_input_with_possibilities() {
        let message = InnerError::BadInputWithPossibilities {
            bad_input: "bar".to_string(),
            message: "it should be foo".to_string(),
            possible_values: "foo | baz".to_string(),
        };

        assert_eq!("it should be foo
bad input: bar
possible values: foo | baz", message.to_string());
    }

    #[test]
    fn display_rgb_syntax_error() {
        let message = InnerError::RgbSyntaxError {
            bad_input: "bar".to_string(),
            message: "it should be foo".to_string(),
        };

        assert_eq!("Error parsing RGB value: it should be foo
bad input: bar", message.to_string());
    }
}
