//! # Builtins
//!
use std::str::FromStr;
use std::collections::HashMap;

use crate::a1::A1;
use crate::{Error, Node, Integer, Reference, Result};

pub struct BuiltinFunction(Box<dyn Fn(A1, Vec<Box<dyn Node>>) -> Result<Box<dyn Node>>>);

pub struct BuiltinVariable(Box<dyn Fn(A1) -> Result<Box<dyn Node>>>);

impl BuiltinFunction {
    pub fn functions() -> HashMap<String, BuiltinFunction> {
        let mut fns = HashMap::new();

        fns.insert("cellabove".to_string(), Self(Box::new(|_a1, args| {
            let r = Self::verify_one_arg("cellabove", args)?;
            let p = A1::from_str(&r)?;

            Ok(Box::new(Reference(p.to_string())))
        })));

        fns
    }

    /// For now all functions take a Reference as a single arg - we can elaborate on this in the
    /// future.
    fn verify_one_arg(fn_name: &str, args: Vec<Box<dyn Node>>) -> Result<String> {
        if args.len() != 1 {
            return Err(Error::CodeSyntaxError {
                bad_input: args.len().to_string(), // XXX figure out a way to format this
                message: format!("Expected a single argument to `{}`", fn_name),
                line_number: 0, // XXX
            })
        } 

        if let Some(id) = args[0].id_ref() {
            Ok(id)
        } else {
            return Err(Error::CodeSyntaxError {
                bad_input: args[0].to_string(),
                line_number: 0, // XXX
                message: format!("Expected a cell reference as the only argumnent to `{}`", fn_name),
            })
        }
    }
}

impl BuiltinVariable {
    // TODO: 
    //
    // * Add:
    //   * colref
    //   * colleft
    //   * colright
    pub fn variables() -> HashMap<String, BuiltinVariable> {
        let mut vars = HashMap::new();

        // `colnum` - The number of the current column.  
        vars.insert("colnum".to_owned(), Self(Box::new(|a1| {
            if let Some(x) = a1.x() {
                Ok(Box::new(Integer((x as i64) + 1)))
            } else {
                Err(Error::CodeSyntaxError {
                    bad_input: a1.to_string(),
                    line_number: 0, // XXX
                    message: "Expected a cell reference with a column component".to_owned(),
                })
            }
        })));

        // `cellref` - A reference to the current cell.  
        vars.insert("cellref".to_owned(), Self(Box::new(|a1| {
            Ok(Box::new(Reference(a1.to_string())))
        })));
 
        // `rowabove` - A (row-relative) reference to the row above the current cell.
        vars.insert("rowabove".to_owned(), Self(Box::new(|a1| {
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
        })));

        // `rowbelow` - A (row-relative) reference to the row below the current cell.
        vars.insert("rowbelow".to_owned(), Self(Box::new(|a1| {
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
        })));

        // `rownum` - The number of the current row.  Starts at 1.
        vars.insert("rownum".to_owned(), Self(Box::new(|a1| {
            if let Some(y) = a1.y() {
                Ok(Box::new(Integer((y as i64) + 1)))
            } else {
                Err(Error::CodeSyntaxError {
                    bad_input: a1.to_string(),
                    line_number: 0, // XXX
                    message: "Expected a cell reference with a row component".to_owned(),
                })
            }
        })));

        // `rowref` - A reference to the current row.  
        vars.insert("rowref".to_owned(), Self(Box::new(|a1| {
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
        })));

        vars
    }
}


#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn 
    // TODO
}
