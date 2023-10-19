//! # BadInput
//!
//! "Bad input" comes in two forms: from the AST parser & lexer or from the modifier parser &
//! lexer. The two are not quite the same and have their own use-cases, but when it comes to error
//! handling there are some features we'll want to re-use:
//!
//! * What line it happened on
//! * Where in the line it happened (char offset)
//! * A relevant error message (via `fmt::Display`)
//!
use crate::error::ParseError;
use crate::{CharOffset, LineNumber};
use std::fmt;

pub(crate) trait BadInput: fmt::Debug + fmt::Display {
    /// Given a `message`, turn this input into an error
    fn into_parse_error(self, message: &str) -> ParseError;

    /// The line number where the bad input occurred.
    fn line_number(&self) -> LineNumber;

    /// The character-offset within the line where the bad input occurred.
    fn line_offset(&self) -> CharOffset;
}
