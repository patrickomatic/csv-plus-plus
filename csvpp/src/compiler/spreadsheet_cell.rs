//! # SpreadsheetCell
//!
use a1_notation;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::{Expand, Modifier, Result, SourceCode};
use crate::ast::{Ast, Node};
use super::ast_parser::AstParser;
use super::modifier_parser::ModifierParser;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpreadsheetCell {
    pub ast: Option<Ast>,
    pub position: a1_notation::Address,
    pub modifier: Modifier,
    pub row_modifier: Option<Modifier>,
    pub value: String,
}

impl SpreadsheetCell {
    pub fn parse(
        input: &str,
        position: a1_notation::Address,
        row_modifier: &Modifier,
        source_code: &SourceCode,
    ) -> Result<SpreadsheetCell> {
        let parsed_modifiers = ModifierParser::parse(input, source_code, position, row_modifier)?;

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
        self.row_modifier.clone().and_then(|rm| rm.expand)
    }

    /// Clone the `SpreadsheetCell` keeping all of it's data the same, except it will reflect that
    /// it's been moved to `new_row`.  This involves updating `position` and `expand.start_row`
    pub fn clone_to_row(&self, new_row: a1_notation::Row) -> Self {
        let row_modifier = self.row_modifier.clone().map(|rm|
            Modifier {
                expand: rm.expand.map(|e| e.clone_to_row(new_row)),
                ..rm
            });

        let new_position = a1_notation::Address::new(self.position.column.x, new_row.y);

        Self {
            row_modifier,
            position: new_position,
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
            .map(|a| {
                match *a {
                    Node::FunctionCall { .. } | Node::InfixFunctionCall { .. } =>
                        format!("={}", a),
                    _ => a.to_string(),
                }
            })
            .unwrap_or_else(|| self.value.clone());

        write!(f, "{string_val}")
    }
}

#[cfg(test)]
mod tests {
    use a1_notation::Address;
    use crate::test_utils::TestFile;
    use super::*;

    #[test]
    fn clone_to_row() {
        let cell = SpreadsheetCell {
            ast: None,
            value: "foo".to_string(),
            position: Address::new(1, 4),
            modifier: Modifier::default(),
            row_modifier: None,
        };

        let cloned_row = cell.clone_to_row(10.into());
        assert_eq!(cloned_row.position.to_string(), "B11");
    }

    #[test]
    fn parse_no_ast() {
        let test_file = TestFile::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let cell = SpreadsheetCell::parse("foo", 
                                          Address::new(0, 4),
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
                                          Address::new(0, 4),
                                          &Modifier::default(),
                                          &source_code).unwrap();

        assert!(cell.ast.is_some());
    }

    #[test]
    fn display_function_call () {
        let cell = SpreadsheetCell {
            ast: Some(Box::new(Node::fn_call("foo", &[1.into(), 2.into()]))),
            value: "foo".to_string(),
            position: Address::new(0, 4),
            modifier: Modifier::default(),
            row_modifier: None,
        };
        
        assert_eq!(cell.to_string(), "=foo(1, 2)");
    }

    #[test]
    fn display_infix_function_call () {
        let cell = SpreadsheetCell {
            ast: Some(Box::new(Node::infix_fn_call(1.into(), "*", 2.into()))),
            value: "foo".to_string(),
            position: Address::new(0, 4),
            modifier: Modifier::default(),
            row_modifier: None,
        };
        
        assert_eq!(cell.to_string(), "=(1 * 2)");
    }

    #[test]
    fn display_number () {
        let cell = SpreadsheetCell {
            ast: Some(Box::new(1.into())),
            value: "foo".to_string(),
            position: Address::new(0, 4),
            modifier: Modifier::default(),
            row_modifier: None,
        };
        
        assert_eq!(cell.to_string(), "1");
    }
}
