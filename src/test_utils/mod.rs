/// Some shared test utility functions.  These are only be used within tests and not in the in
/// final release executable.
use crate::{DateTime, Runtime, SourceCode};

mod test_file;
pub(crate) use test_file::TestFile;

pub(crate) fn build_ast_token_match(str_match: &str) -> crate::parser::ast_lexer::TokenMatch {
    crate::parser::ast_lexer::TokenMatch {
        token: crate::parser::ast_lexer::Token::Reference,
        str_match,
        line_number: 0,
        line_offset: 0,
    }
}

pub(crate) fn build_modifier_token_match(
    str_match: &str,
) -> crate::parser::modifier_lexer::TokenMatch {
    crate::parser::modifier_lexer::TokenMatch {
        token: crate::parser::modifier_lexer::Token::ModifierRightSide,
        str_match: str_match.to_string(),
        line_number: 0,
        line_offset: 0,
    }
}

pub(crate) fn build_date_time_ymd(y: i32, m: u32, d: u32) -> DateTime {
    DateTime::Date(chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap())
}

/// If the test just needs a runtime but doesn't care about it at all
pub(crate) fn build_runtime() -> Runtime {
    TestFile::new("foo.xlsx", "foo,bar,baz").into()
}

/// If the test just needs a source code but doesn't care about it at all
pub(crate) fn build_source_code() -> SourceCode {
    TestFile::new("bar.xlsx", "foo,bar,baz").into()
}
