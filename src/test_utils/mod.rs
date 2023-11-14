//! Some shared test utility functions.  These are only be used within tests and not in the in
//! final release executable.
//!
use crate::parser::ast_lexer;
use crate::parser::cell_lexer;
use crate::{DateTime, Runtime, SourceCode};
use std::sync;

mod test_file;
mod test_source_code;

pub(crate) use test_file::TestFile;
pub(crate) use test_source_code::TestSourceCode;

pub(crate) fn build_ast_token_match<'a>(
    str_match: &'a str,
    source_code: &'a SourceCode,
) -> ast_lexer::TokenMatch<'a> {
    ast_lexer::TokenMatch {
        token: ast_lexer::Token::Reference,
        str_match,
        line_number: 0,
        line_offset: 0,
        source_code,
    }
}

pub(crate) fn build_cell_token_match(str_match: &str) -> cell_lexer::TokenMatch {
    let source_code = build_source_code();
    cell_lexer::TokenMatch {
        token: cell_lexer::Token::Identifier,
        str_match: str_match.to_string(),
        position: a1_notation::Address::new(0, 0),
        cell_offset: 0,
        source_code: sync::Arc::new(source_code),
    }
}

pub(crate) fn build_date_time_ymd(y: i32, m: u32, d: u32) -> DateTime {
    DateTime::Date(chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap())
}

/// If the test just needs a runtime but doesn't care about it at all
pub(crate) fn build_runtime() -> Runtime {
    (&TestSourceCode::new("foo.xlsx", "foo,bar,baz")).into()
}

/// If the test just needs a source code but doesn't care about it at all
pub(crate) fn build_source_code() -> SourceCode {
    (&TestSourceCode::new("bar.xlsx", "foo,bar,baz")).into()
}
