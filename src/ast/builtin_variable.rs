//! # BuiltinVariable
//!
use super::{Node, VariableEval, VariableName};
use crate::ParseResult;
use a1_notation::{Address, RangeOrCell};
use std::collections;
use std::fmt;

pub struct BuiltinVariable {
    pub eval: VariableEval,
    pub name: VariableName,
}

impl BuiltinVariable {
    fn new<F: Fn(a1_notation::Address) -> ParseResult<Node> + 'static>(
        name: &str,
        eval: F,
    ) -> Self {
        Self {
            name: name.to_string(),
            eval: Box::new(eval),
        }
    }

    // TODO: better names for these!  I dunno what it is but I don't like these, it feels
    // unintuitive.
    pub fn all() -> collections::HashMap<String, BuiltinVariable> {
        let mut vars = collections::HashMap::new();

        // `colleft` - A (column-relative) reference to the column left of the current cell.
        vars = def_var(vars, "colleft", |a1| {
            let col: RangeOrCell = a1.shift_left(1).column.into();
            Ok(col.into())
        });

        // `colright` - A (column-relative) reference to the column right of the current cell.
        vars = def_var(vars, "colright", |a1| {
            let col: RangeOrCell = a1.shift_right(1).column.into();
            Ok(col.into())
        });

        // `colnum` - The number of the current column.
        vars = def_var(vars, "colnum", |a1| Ok(((a1.column.x as i64) + 1).into()));

        // `cellref` - A reference to the current cell.
        vars = def_var(vars, "cellref", |a1| Ok(a1.into()));

        // `colref` - A reference to the current column.
        vars = def_var(vars, "colref", |a1| {
            let col: RangeOrCell = a1.column.into();
            Ok(col.into())
        });

        // `rowabove` - A (row-relative) reference to the row above the current cell.
        vars = def_var(vars, "rowabove", |a1| {
            let row: RangeOrCell = a1.shift_up(1).row.into();
            Ok(row.into())
        });

        // `rowbelow` - A (row-relative) reference to the row below the current cell.
        vars = def_var(vars, "rowbelow", |a1| {
            let row: RangeOrCell = a1.shift_down(1).row.into();
            Ok(row.into())
        });

        // `rownum` - The number of the current row.  Starts at 1.
        vars = def_var(vars, "rownum", |a1| Ok(((a1.row.y as i64) + 1).into()));

        // `rowref` - A reference to the current row.
        vars = def_var(vars, "rowref", |a1| {
            let row: RangeOrCell = a1.row.into();
            Ok(row.into())
        });

        vars
    }
}

fn def_var<F>(
    mut vars: collections::HashMap<String, BuiltinVariable>,
    name: &str,
    eval: F,
) -> collections::HashMap<String, BuiltinVariable>
where
    F: Fn(Address) -> ParseResult<Node> + 'static,
{
    vars.insert(name.to_string(), BuiltinVariable::new(name, eval));
    vars
}

impl fmt::Debug for BuiltinVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuiltinVariable")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn colleft() {
        let vars = BuiltinVariable::all();
        let current = Address::new(4, 0);
        let colleft = vars.get("colleft").unwrap();

        assert_eq!((colleft.eval)(current).unwrap(), Node::reference("D:D"));
    }
}
