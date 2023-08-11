//! # SpreadsheetCell
//!
use a1_notation;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::{Modifier, Result, SourceCode};
use crate::ast::Ast;
use crate::modifier::Expand;
use super::ast_parser::AstParser;
use super::modifier_parser::ModifierParser;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpreadsheetCell {
    pub ast: Option<Ast>,
    pub position: a1_notation::A1,
    pub modifier: Modifier,
    pub row_modifier: Option<Modifier>,
    pub value: String,
}

impl SpreadsheetCell {
    pub fn parse(
        input: &str,
        position: a1_notation::A1,
        row_modifier: &Modifier,
        source_code: &SourceCode,
    ) -> Result<SpreadsheetCell> {
        let parsed_modifiers = ModifierParser::parse(input, source_code, &position, row_modifier)?;

        Ok(SpreadsheetCell {
            ast: Self::parse_ast(&parsed_modifiers.value, source_code)?,
            position,
            modifier: parsed_modifiers.modifier
                .unwrap_or_else(|| parsed_modifiers.row_modifier.clone().unwrap_or(row_modifier.clone())),
            row_modifier: parsed_modifiers.row_modifier,
            value: parsed_modifiers.value,
        })
    }

    pub fn expand(&self) -> Option<Expand> {
        if let Some(rm) = &self.row_modifier {
            rm.expand
        } else {
            None
        }
    }

    pub fn clone_to_row(&self, new_row: usize) -> Self {
        Self {
            position: a1_notation::A1::builder()
                .xy(self.position.x().unwrap(), new_row)
                .build()
                .unwrap(),

            ..self.clone()
        }
    }

    fn parse_ast(input: &str, source_code: &SourceCode) -> Result<Option<Ast>> {
        if let Some(without_equals) = input.strip_prefix('=') {
            Ok(Some(AstParser::parse(without_equals, false, Some(source_code))?))
        } else {
            Ok(None)
        }
    }
}

// TODO we might want a more dedicated function like to_formula
impl fmt::Display for SpreadsheetCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_val = self
            .ast
            .clone()
            .map(|a| a.to_string())
            .unwrap_or_else(|| self.value.clone());

        write!(f, "{string_val}")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::test_utils::TestFile;
    use super::*;

    #[test]
    fn clone_to_row() {
        let cell = SpreadsheetCell {
            ast: None,
            value: "foo".to_string(),
            position: a1_notation::A1::from_str("B5").unwrap(),
            modifier: Modifier::default(),
            row_modifier: None,
        };

        let cloned_row = cell.clone_to_row(10);
        assert_eq!(cloned_row.position.to_string(), "B11");
    }

    #[test]
    fn parse_no_ast() {
        let test_file = TestFile::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let cell = SpreadsheetCell::parse("foo", 
                                          a1_notation::A1::from_str("A5").unwrap(),
                                          &Modifier::default(),
                                          &source_code).unwrap();

        assert_eq!(cell.value, "foo");
        assert_eq!(cell.ast, None);
    }

    #[test]
    fn parse_ast() {
        let test_file = TestFile::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let cell = SpreadsheetCell::parse("=1 + foo", 
                                          a1_notation::A1::from_str("A5").unwrap(),
                                          &Modifier::default(),
                                          &source_code).unwrap();

        assert!(cell.ast.is_some());
    }
}
