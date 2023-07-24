//! # BuiltinFunction
//!
use std::collections;
use std::fmt;
use std::str::FromStr;
use crate::{InnerError, InnerResult};
use super::{Ast, FunctionEval, FunctionName, Node};

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
                let p = a1_notation::A1::from_str(&r)?;

                Ok(Node::Reference(p.to_string()))
            })
        };

        fns.insert("cellabove".to_owned(), cellabove_ast);

        // XXX other ones

        fns
    }

    /// For now all functions take a Reference as a single arg - we can elaborate on this in the
    /// future.
    fn verify_one_arg(fn_name: &str, args: &[Ast]) -> InnerResult<String> {
        if args.len() != 1 {
            return Err(InnerError::bad_input(
                &args.len().to_string(), // TODO figure out a way to format this
                &format!("Expected a single argument to `{}`", fn_name)))
        } 

        match &*args[0] {
            Node::Reference(r) => Ok(r.to_owned()),
            n => Err(InnerError::bad_input(
                n.to_string().as_str(),
                &format!("Expected a cell reference as the only argumnent to `{}`", fn_name))),
        }
    }
}

/// Debug is manually implemented because we can't derive it for `eval`.
impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuiltinFunction")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

