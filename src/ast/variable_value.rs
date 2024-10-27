use super::Ast;
use crate::{compiler_error, Fill};

/// The variable a value can have will depend on a variety of contexts
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum VariableValue {
    /// Points directly at a cell
    ///
    /// ```csvpp
    /// ---
    /// foo,[[var=bar]],baz
    /// ```
    Absolute(a1::Address),

    /// If a variable is defined in the code section it will have an AST as a value.
    ///
    /// ```csvpp
    /// foo := 42
    /// ---
    /// =foo,bar,baz
    /// ```
    Ast(Ast),

    /// It's scoped to a column relative to a fill.
    ///
    /// ```csvpp
    /// ---
    /// ![[fill]]foo,[[var=foo]],baz
    /// ```
    ColumnRelative { column: a1::Column, fill: Fill },

    /// References a row (outside of a fill)
    ///
    /// ```csvpp
    /// ---
    /// ![[var=foo]]foo,bar,baz
    /// ```
    Row(a1::Row),

    /// A row within a fill.
    ///
    /// ```csvpp
    /// ---
    /// ![[var=foo fill=20]]foo,bar,baz
    /// ```
    RowRelative { row: a1::Row, fill: Fill },
}

impl VariableValue {
    pub(crate) fn into_ast(self, position: Option<a1::Address>) -> Ast {
        if let Some(position) = position {
            match self {
                // absolute value, just turn it into a Ast
                VariableValue::Absolute(address) => Ast::new(address.into()),

                // already an AST, just return it
                VariableValue::Ast(ast) => ast,

                // it's relative to a fill - so if it's referenced inside the
                // fill, it's the value at that location.  If it's outside the fill
                // it's the range that it represents
                VariableValue::ColumnRelative { fill, column } => {
                    let fill_a1: a1::A1 = fill.into();

                    Ast::new(if fill_a1.contains(&position.into()) {
                        position.with_x(column.x).into()
                    } else {
                        let row_range: a1::A1 = fill.into();
                        row_range.with_x(column.x).into()
                    })
                }

                VariableValue::Row(row) => {
                    let a1: a1::A1 = row.into();
                    Ast::new(a1.into())
                }

                VariableValue::RowRelative { fill, .. } => {
                    let fill_a1: a1::A1 = fill.into();

                    Ast::new(if fill_a1.contains(&position.into()) {
                        // we're within the scope (fill) so it's the row we're on
                        let row_a1: a1::A1 = position.row.into();
                        row_a1.into()
                    } else {
                        // we're outside the scope (fill), so it represents the entire
                        // range contained by it (the scope)
                        let row_range: a1::A1 = fill.into();
                        row_range.into()
                    })
                }
            }
        } else {
            match self {
                VariableValue::Absolute(address) => Ast::new(address.into()),
                VariableValue::Ast(ast) => ast,
                VariableValue::Row(row) => {
                    let a1: a1::A1 = row.into();
                    Ast::new(a1.into())
                }
                _ => compiler_error(
                    "Attempted to load a spreadsheet-relative value in a non-spreadsheet context",
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::*;
    use std::panic;

    fn build_position() -> a1::Address {
        a1::Address::new(0, 0)
    }

    #[test]
    fn into_ast_absolute_some_position() {
        assert_eq!(
            VariableValue::Absolute(build_position()).into_ast(Some(build_position())),
            a1::cell(0, 0).into()
        );
    }

    #[test]
    fn into_ast_absolute_none_position() {
        assert_eq!(
            VariableValue::Absolute(a1::Address::new(0, 0)).into_ast(None),
            a1::cell(0, 0).into()
        );
    }

    #[test]
    fn into_ast_ast_some_position() {
        assert_eq!(
            VariableValue::Ast(1.into()).into_ast(Some(build_position())),
            1.into()
        );
    }

    #[test]
    fn into_ast_ast_none_position() {
        assert_eq!(VariableValue::Ast(1.into()).into_ast(None), 1.into());
    }

    #[test]
    fn into_ast_column_relative_none_position() {
        assert!(panic::catch_unwind(|| {
            VariableValue::ColumnRelative {
                column: 5.into(),
                fill: Fill::new(0, Some(20)),
            }
            .into_ast(None)
        })
        .is_err());
    }

    #[test]
    fn into_ast_column_relative_some_position_in_fill() {
        assert_eq!(
            VariableValue::ColumnRelative {
                column: 5.into(),
                fill: Fill::new(0, Some(20)),
            }
            .into_ast(Some(build_position())),
            Node::reference("F1").into()
        );
    }

    #[test]
    fn into_ast_column_relative_some_position_outside_fill() {
        assert_eq!(
            VariableValue::ColumnRelative {
                column: 5.into(),
                fill: Fill::new(20, Some(20)),
            }
            .into_ast(Some(build_position())),
            Node::reference("F21:F40").into()
        );
    }

    #[test]
    fn into_ast_row_none_position() {
        assert_eq!(
            VariableValue::Row(1.into()).into_ast(None),
            Node::reference("2:2").into()
        );
    }

    #[test]
    fn into_ast_row_some_position() {
        assert_eq!(
            VariableValue::Row(1.into()).into_ast(Some(build_position())),
            Node::reference("2:2").into()
        );
    }

    #[test]
    fn into_ast_row_relative_none_position() {
        assert!(panic::catch_unwind(|| {
            VariableValue::RowRelative {
                row: 5.into(),
                fill: Fill::new(0, Some(20)),
            }
            .into_ast(None)
        })
        .is_err());
    }

    #[test]
    fn into_ast_row_relative_some_position_in_fill() {
        assert_eq!(
            VariableValue::RowRelative {
                row: 5.into(),
                fill: Fill::new(0, Some(20)),
            }
            .into_ast(Some(build_position())),
            Node::reference("1:1").into()
        );
    }

    #[test]
    fn into_ast_row_relative_some_position_outside_fill() {
        assert_eq!(
            VariableValue::RowRelative {
                row: 5.into(),
                fill: Fill::new(20, Some(20)),
            }
            .into_ast(Some(build_position())),
            Node::reference("21:40").into()
        );
    }
}
