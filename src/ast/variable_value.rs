use super::Ast;
use crate::Fill;

/// The variable a value can have will depend on a variety of contexts
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum VariableValue {
    /// Points directly at a cell
    ///
    /// ```csvpp
    /// ---
    /// foo,[[var=bar]],baz
    /// ```
    Absolute(a1_notation::Address),

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
    ColumnRelative {
        column: a1_notation::Column,
        scope: Fill,
    },

    /// References a row (outside of a fill)
    ///
    /// ```csvpp
    /// ---
    /// ![[var=foo]]foo,bar,baz
    /// ```
    Row(a1_notation::Row),

    /// A row within a fill.
    ///
    /// ```csvpp
    /// ---
    /// ![[var=foo / fill=20]]foo,bar,baz
    /// ```
    RowRelative { row: a1_notation::Row, scope: Fill },
}

impl VariableValue {
    pub(crate) fn into_ast(self, position: a1_notation::Address) -> Ast {
        match self {
            // absolute value, just turn it into a Ast
            VariableValue::Absolute(address) => Ast::new(address.into()),

            // already an AST, just return it
            VariableValue::Ast(ast) => ast,

            // it's relative to an fill - so if it's referenced inside the
            // fill, it's the value at that location.  If it's outside the fill
            // it's the range that it represents
            VariableValue::ColumnRelative { scope, column } => {
                let scope_a1: a1_notation::A1 = scope.into();

                Ast::new(if scope_a1.contains(&position.into()) {
                    position.with_x(column.x).into()
                } else {
                    let row_range: a1_notation::A1 = scope.into();
                    row_range.with_x(column.x).into()
                })
            }

            VariableValue::Row(row) => {
                let a1: a1_notation::A1 = row.into();
                Ast::new(a1.into())
            }

            VariableValue::RowRelative { scope, .. } => {
                let scope_a1: a1_notation::A1 = scope.into();

                Ast::new(if scope_a1.contains(&position.into()) {
                    // we're within the scope (fill) so it's the row we're on
                    let row_a1: a1_notation::A1 = position.row.into();
                    row_a1.into()
                } else {
                    // we're outside the scope (fill), so it represents the entire
                    // range contained by it (the scope)
                    let row_range: a1_notation::A1 = scope.into();
                    row_range.into()
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_ast_absolute() {
        // XXX
    }
}
