//! # BuiltinFunction
//!
use std::collections;
use std::fmt;
use std::str::FromStr;
use crate::{A1, Error, Result};
use super::{FunctionEval, FunctionName, Node};

pub struct BuiltinFunction {
    pub eval: FunctionEval,
    pub name: FunctionName, 
}

impl BuiltinFunction {
    pub fn all() -> collections::HashMap<String, BuiltinFunction> {
        let mut fns = collections::HashMap::new();

        let cellabove_ast = Self {
            name: "cellabove".to_owned(),
            eval: Box::new(|_a1, args| {
                let r = Self::verify_one_arg("cellabove", args)?;
                let p = A1::from_str(&r)?;

                Ok(Node::Reference(p.to_string()))
            })
        };

        fns.insert("cellabove".to_owned(), cellabove_ast);

        // XXX other ones

        fns
    }

    /// For now all functions take a Reference as a single arg - we can elaborate on this in the
    /// future.
    fn verify_one_arg(fn_name: &str, args: &[Node]) -> Result<String> {
        if args.len() != 1 {
            return Err(Error::CodeSyntaxError {
                bad_input: args.len().to_string(), // XXX figure out a way to format this
                message: format!("Expected a single argument to `{}`", fn_name),
                line_number: 0, // XXX
            })
        } 

        match &args[0] {
            Node::Reference(r) => Ok(r.to_owned()),
            n => Err(Error::CodeSyntaxError {
                bad_input: n.to_string(),
                line_number: 0, // XXX
                message: format!("Expected a cell reference as the only argumnent to `{}`", fn_name),
            }),
        }
    }
}

impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuiltinFunction")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

