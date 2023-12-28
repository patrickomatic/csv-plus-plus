//! Some shared test utility functions.  These are only be used within tests and not in the in
//! final release executable.
//!
use crate::parser::ast_lexer;
use crate::parser::cell_lexer;
use crate::{ArcSourceCode, Compiler, DateTime, Module, ModulePath};

mod test_file;
mod test_source_code;

pub(crate) use test_file::TestFile;
pub(crate) use test_source_code::TestSourceCode;

pub(crate) fn build_ast_token_match(
    str_match: &str,
    source_code: ArcSourceCode,
) -> ast_lexer::TokenMatch {
    ast_lexer::TokenMatch {
        token: ast_lexer::Token::Reference,
        line_number: 0,
        line_offset: 0,
        position: None,
        source_code,
        str_match,
    }
}

pub(crate) fn build_cell_token_match(str_match: &str) -> cell_lexer::TokenMatch {
    cell_lexer::TokenMatch {
        token: cell_lexer::Token::Identifier,
        str_match: str_match.to_string(),
        position: a1_notation::Address::new(0, 0),
        cell_offset: 0,
        source_code: build_source_code(),
    }
}

pub(crate) fn build_date_time_ymd(y: i32, m: u32, d: u32) -> DateTime {
    DateTime::Date(chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap())
}

/// If the test just needs a compiler but doesn't care about it at all
pub(crate) fn build_compiler() -> Compiler {
    (&TestSourceCode::new("foo.xlsx", "foo,bar,baz")).into()
}

pub(crate) fn build_module() -> Module {
    Module {
        compiler_version: "v0.0.1".to_string(),
        source_code: build_source_code(),
        module_path: ModulePath::new("foo"),
        scope: Default::default(),
        spreadsheet: Default::default(),
        required_modules: Default::default(),
        is_dirty: false,
        needs_eval: true,
    }
}

/// If the test just needs a source code but doesn't care about it at all
pub(crate) fn build_source_code() -> ArcSourceCode {
    ArcSourceCode::new((&TestSourceCode::new("bar.xlsx", "foo,bar,baz")).into())
}

pub(crate) fn build_source_code_from_input(input: &str) -> ArcSourceCode {
    ArcSourceCode::new((&TestSourceCode::new("bar.xlsx", input)).into())
}
