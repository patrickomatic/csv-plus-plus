//! # BuiltinFunction
//!
use std::any;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::{A1, Error, Result};
use super::{Ast, FunctionName, Functions, Node, NodeId, NodeWithId, Reference};

pub struct BuiltinFunction { 
    pub eval: Box<dyn Fn(&A1, &[Ast]) -> Result<Ast>>,
    pub name: FunctionName, 
}

impl BuiltinFunction {
    pub fn all() -> Functions {
        let mut fns = HashMap::new();

        let cellabove_ast: Ast = Box::new(BuiltinFunction {
            name: "cellabove".to_owned(),
            eval: Box::new(|_a1, args| {
                let r = Self::verify_one_arg("cellabove", args)?;
                let p = A1::from_str(&r)?;

                let ast: Ast = Box::new(Reference(p.to_string()));
                Ok(ast)
            })
        });

        fns.insert("cellabove".to_owned(), cellabove_ast);

        // XXX other ones

        fns
    }

    /// For now all functions take a Reference as a single arg - we can elaborate on this in the
    /// future.
    fn verify_one_arg(fn_name: &str, args: &[Ast]) -> Result<String> {
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
            Err(Error::CodeSyntaxError {
                bad_input: args[0].to_string(),
                line_number: 0, // XXX
                message: format!("Expected a cell reference as the only argumnent to `{}`", fn_name),
            })
        }
    }
}

impl NodeWithId for BuiltinFunction {
    fn id(&self) -> NodeId {
        self.name.clone()
    }
}

impl Node for BuiltinFunction {
    fn as_any(&self) -> &dyn any::Any { self }

    fn eval_fn(&self, a1: &A1, args: &[Ast]) -> Result<Option<Ast>> {
        Ok(Some((*self.eval)(a1, args)?))
    }

    fn node_eq(&self, other: &dyn any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, { |f| 
            self.name == f.name
        })
    }
}

impl Clone for BuiltinFunction {
    fn clone(&self) -> BuiltinFunction {
        // TODO I dunno what's reasonable here, I'm not sure that this will happen in real
        // code........
        panic!("Cannot clone a built-in function")
    }
}

impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BuiltinFunction")
            .field("name", &self.name)
            .field("eval", &"...".to_string())
            .finish()
    }
}

impl fmt::Display for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(...)", self.name)
    }
}

