//! # BuiltinVariable
//!
use std::collections;
use std::fmt;
use crate::{InnerError, InnerResult};
use super::{Node, VariableEval, VariableName};

pub struct BuiltinVariable {
    pub eval: VariableEval,
    pub name: VariableName, 
}

impl BuiltinVariable {
    fn new<F: Fn(&a1_notation::A1) -> InnerResult<Node> + 'static>(name: &str, eval: F) -> Self {
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
        vars = Self::def_var(vars, "colleft", |a1| {
            if let Some(c) = a1.clone().shift_left(1).column() {
                Ok(c.into())
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a column component"))
            }
        });
        
        // `colright` - A (column-relative) reference to the column right of the current cell.
        vars = Self::def_var(vars, "colright", |a1| {
            if let Some(c) = a1.clone().shift_right(1).column() {
                Ok(c.into())
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a column component"))
            }
        });

        // `colnum` - The number of the current column.  
        vars = Self::def_var(vars, "colnum", |a1| {
            if let Some(x) = a1.x() {
                Ok(((x as i64) + 1).into())
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a column component"))
            }
        });

        // `cellref` - A reference to the current cell.  
        vars = Self::def_var(vars, "cellref", |a1| {
            Ok(a1.clone().into())
        });
        
        // `colref` - A reference to the current column.  
        vars = Self::def_var(vars, "colref", |a1| {
            if let Some(x) = a1.x() {
                Ok(a1_notation::column(x).into())
            } else {
                Err(InnerError::bad_input(
                        &a1.to_string(), 
                        "Expected a cell reference with a column component"))
            }
        });

        // `rowabove` - A (row-relative) reference to the row above the current cell.
        vars = Self::def_var(vars, "rowabove", |a1| {
            if let Some(c) = a1.clone().shift_up(1).column() {
                Ok(c.into())
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a row component"))
            }
        });
        
        // `rowbelow` - A (row-relative) reference to the row below the current cell.
        vars = Self::def_var(vars, "rowbelow", |a1| {
            if let Some(c) = a1.clone().shift_down(1).column() {
                Ok(c.into())
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a row component"))
            }
        });
        
        // `rownum` - The number of the current row.  Starts at 1.
        vars = Self::def_var(vars, "rownum", |a1| {
            if let Some(y) = a1.y() {
                Ok(((y as i64) + 1).into())
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a row component"))
            }
        });

        // `rowref` - A reference to the current row.  
        vars = Self::def_var(vars, "rowref", |a1| {
            if let Some(y) = a1.y() {
                Ok(a1_notation::row(y).into())
            } else {
                Err(InnerError::bad_input(
                        &a1.to_string(), 
                        "Expected a cell reference with a row component"))
            }
        });

        vars
    }

    fn def_var<F>(
        mut vars: collections::HashMap<String, BuiltinVariable>, 
        name: &str, 
        eval: F,
    ) -> collections::HashMap<String, BuiltinVariable>
    where F: Fn(&a1_notation::A1) -> InnerResult<Node> + 'static {
        vars.insert(name.to_string(), Self::new(name, eval));
        vars
    }
}

impl fmt::Debug for BuiltinVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuiltinVariable")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

