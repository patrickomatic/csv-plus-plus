use crate::error::{BadInput, ParseError};
use crate::{ArcSourceCode, CharOffset, LineNumber};
use std::fmt;

#[derive(Debug)]
pub(crate) struct UnknownToken {
    pub(crate) bad_input: String,
    pub(crate) line_number: LineNumber,
    pub(crate) line_offset: CharOffset,
    pub(crate) source_code: ArcSourceCode,
}

impl fmt::Display for UnknownToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut shortened_bad_input = self.bad_input.clone();
        shortened_bad_input.truncate(50);
        write!(f, "{shortened_bad_input}")
    }
}

impl BadInput for UnknownToken {
    fn line_number(&self) -> LineNumber {
        self.line_number
    }

    fn line_offset(&self) -> CharOffset {
        self.line_offset
    }

    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
        self.source_code.clone().parse_error(self, message)
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
    use crate::test_utils::build_source_code;

    #[test]
    fn display() {
        let ut = UnknownToken {
            bad_input: "foo".to_string(),
            line_number: 10,
            line_offset: 1,
            source_code: build_source_code(),
        };

        assert_eq!(ut.to_string(), "foo");
    }

    #[test]
    fn display_long() {
        let ut = UnknownToken {
            bad_input: "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string(),
            line_number: 10,
            line_offset: 1,
            source_code: build_source_code(),
        };

        assert_eq!(
            ut.to_string(),
            "12345678901234567890123456789012345678901234567890"
        );
    }
}
