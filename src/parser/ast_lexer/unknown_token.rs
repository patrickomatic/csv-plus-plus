use crate::error::{BadInput, ParseError};
use crate::ArcSourceCode;
use csvp::{Field, SourcePosition};
use std::fmt;

#[derive(Debug)]
pub(crate) struct UnknownToken {
    pub(crate) bad_input: String,
    pub(crate) position: SourcePosition,
    pub(crate) source_code: ArcSourceCode,
    pub(crate) field: Option<Field>,
}

impl fmt::Display for UnknownToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut shortened_bad_input = self.bad_input.clone();
        shortened_bad_input.truncate(50);
        write!(f, "{shortened_bad_input}")
    }
}

impl BadInput for UnknownToken {
    // TODO: this is duplicated in TokenMatch
    fn position(&self) -> SourcePosition {
        if let Some(field) = self.field.clone() {
            if let Some(position) = field.position_for_offset(self.position.line_offset) {
                return position;
            }
        }

        self.position
    }

    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
        self.source_code.parse_error(&self, message)
    }
}

impl From<UnknownToken> for ParseError {
    fn from(u: UnknownToken) -> Self {
        u.into_parse_error("Error parsing input - invalid token")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn display() {
        let ut = UnknownToken {
            bad_input: "foo".to_string(),
            position: (1, 10).into(),
            field: None,
            source_code: build_source_code(),
        };

        assert_eq!(ut.to_string(), "foo");
    }

    #[test]
    fn display_long() {
        let ut = UnknownToken {
            bad_input: "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string(),
            position: (1, 10).into(),
            field: None,
            source_code: build_source_code(),
        };

        assert_eq!(
            ut.to_string(),
            "12345678901234567890123456789012345678901234567890"
        );
    }
}
