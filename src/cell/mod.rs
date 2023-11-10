//! # Cell
//!
use crate::ast::Ast;
use crate::parser::ast_parser::AstParser;
use crate::parser::modifier_parser::ModifierParser;
use crate::{Modifier, Result, RowModifier, Runtime};
use a1_notation::Address;
use serde::{Deserialize, Serialize};

mod display;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Cell {
    pub ast: Option<Ast>,
    pub modifier: Modifier,
    pub value: String,
}

impl Cell {
    pub fn parse(
        input: &str,
        position: Address,
        row_modifier: &RowModifier,
        runtime: &Runtime,
    ) -> Result<(Self, Option<RowModifier>)> {
        let parsed_modifiers = ModifierParser::parse(input, position, row_modifier, runtime)?;
        let cell = Self {
            ast: Self::parse_ast(&parsed_modifiers.value, runtime)?,
            modifier: parsed_modifiers.modifier.unwrap_or_else(|| {
                parsed_modifiers
                    .row_modifier
                    .clone()
                    .unwrap_or(row_modifier.clone())
                    .into_without_var()
            }),
            value: parsed_modifiers.value.to_string(),
        };

        Ok((cell, parsed_modifiers.row_modifier))
    }

    fn parse_ast(input: &str, runtime: &Runtime) -> Result<Option<Ast>> {
        if let Some(without_equals) = input.strip_prefix('=') {
            Ok(Some(AstParser::parse(without_equals, false, runtime)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestFile;
    use crate::*;

    #[test]
    fn parse_no_ast() {
        let test_file = TestFile::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let (cell, _) = Cell::parse(
            "foo",
            Address::new(0, 4),
            &RowModifier::default(),
            &source_code,
        )
        .unwrap();

        assert_eq!(cell.value, "foo");
        assert_eq!(cell.ast, None);
    }

    #[test]
    fn parse_ast() {
        let test_file = TestFile::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let (cell, _) = Cell::parse(
            "=1 + foo",
            Address::new(0, 4),
            &RowModifier::default(),
            &source_code,
        )
        .unwrap();

        assert!(cell.ast.is_some());
    }
}
