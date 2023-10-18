use crate::error::BadInput;
use crate::{CharOffset, LineNumber};
use std::fmt;

#[derive(Debug)]
pub(crate) struct UnknownToken {
    pub(crate) bad_input: String,
    pub(crate) line_number: LineNumber,
    pub(crate) line_offset: CharOffset,
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
}
