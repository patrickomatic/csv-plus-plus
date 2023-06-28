//! # BuiltinVariable
//!
use std::any;
use std::collections::HashMap;
use std::fmt;

use crate::{A1, Error, Result};
use super::{Ast, Integer, Node, NodeId, NodeWithId, Reference, VariableName, Variables};

pub struct BuiltinVariable { 
    pub eval: Box<dyn Fn(&A1) -> Result<Ast>>,
    pub name: VariableName, 
}

impl BuiltinVariable {
    // TODO: 
    //
    // * Add:
    //   * colref
    //   * colleft
    //   * colright
    pub fn all() -> Variables {
        let mut vars = HashMap::new();

        // `colnum` - The number of the current column.  
        vars = Self::def_var(vars, "colnum", |a1| {
            if let Some(x) = a1.x() {
                Ok(Box::new(Integer((x as i64) + 1)))
            } else {
                Err(Error::CodeSyntaxError {
                    bad_input: a1.to_string(),
                    line_number: 0, // XXX
                    message: "Expected a cell reference with a column component".to_owned(),
                })
            }
        });

        // `cellref` - A reference to the current cell.  
        vars = Self::def_var(vars, "cellref", |a1| {
            Ok(Box::new(Reference(a1.to_string())))
        });

        // `rowabove` - A (row-relative) reference to the row above the current cell.
        vars = Self::def_var(vars, "rowabove", |a1| {
            if let Some(y) = a1.y() {
                let a1_above = A1::builder().y((y - 1).max(0)).build()?;
                Ok(Box::new(Reference(a1_above.to_string())))
            } else {
                Err(Error::CodeSyntaxError {
                    bad_input: a1.to_string(),
                    line_number: 0, // XXX
                    message: "Expected a cell reference with a row component".to_owned(),
                })
            }
        });
        
        // `rowbelow` - A (row-relative) reference to the row below the current cell.
        vars = Self::def_var(vars, "rowbelow", |a1| {
            if let Some(y) = a1.y() {
                let a1_below = A1::builder().y(y + 1).build()?;
                Ok(Box::new(Reference(a1_below.to_string())))
            } else {
                Err(Error::CodeSyntaxError {
                    bad_input: a1.to_string(),
                    line_number: 0, // XXX
                    message: "Expected a cell reference with a row component".to_owned(),
                })
            }
        });
        
        // `rownum` - The number of the current row.  Starts at 1.
        vars = Self::def_var(vars, "rownum", |a1| {
            if let Some(y) = a1.y() {
                Ok(Box::new(Integer((y as i64) + 1)))
            } else {
                Err(Error::CodeSyntaxError {
                    bad_input: a1.to_string(),
                    line_number: 0, // XXX
                    message: "Expected a cell reference with a row component".to_owned(),
                })
            }
        });

        // `rowref` - A reference to the current row.  
        vars = Self::def_var(vars, "rowref", |a1| {
            if let Some(y) = a1.y() {
                let row_a1 = A1::builder().y(y).build()?;
                Ok(Box::new(Reference(row_a1.to_string())))
            } else {
                Err(Error::CodeSyntaxError {
                    bad_input: a1.to_string(),
                    line_number: 0, // XXX
                    message: "Expected a cell reference with a row component".to_owned(),
                })
            }
        });

        vars
    }

    fn def_var<F>(
        mut vars: Variables, 
        name: &str, 
        eval: F,
    ) -> Variables
    where F: Fn(&A1) -> Result<Ast> + 'static {
        let ast: Ast = Box::new( 
            Self {
                name: name.to_string(),
                eval: Box::new(eval),
            },
        );

        vars.insert(name.to_string(), ast);
        vars
    }
}

impl NodeWithId for BuiltinVariable {
    fn id(&self) -> NodeId {
        self.name.clone()
    }
}

impl Node for BuiltinVariable {
    fn as_any(&self) -> &dyn any::Any { self }

    fn eval_var(&self, a1: &A1) -> Result<Option<Ast>> {
        Ok(Some((*self.eval)(a1)?))
    }

    fn node_eq(&self, other: &dyn any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, { |f| 
            self.name == f.name
        })
    }
}

impl fmt::Debug for BuiltinVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BuiltinVariable")
            .field("name", &self.name)
            .field("eval", &"...".to_string())
            .finish()
    }
}

impl fmt::Display for BuiltinVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} := ...", self.name)
    }
}

impl Clone for BuiltinVariable {
    fn clone(&self) -> BuiltinVariable {
        // TODO I dunno what's reasonable here, I'm not sure that this will happen in real
        // code........
        panic!("Cannot clone a built-in variable")
    }
}
