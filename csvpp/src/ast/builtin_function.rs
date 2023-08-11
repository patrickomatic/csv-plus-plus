//! # BuiltinFunction
//!
use std::collections;
use std::fmt;
use crate::{InnerError, InnerResult};
use super::{Ast, FunctionEval, FunctionName, Node};

pub struct BuiltinFunction {
    pub eval: FunctionEval,
    pub name: FunctionName, 
}

impl BuiltinFunction {
    pub fn all() -> collections::HashMap<FunctionName, BuiltinFunction> {
        let mut fns = collections::HashMap::new();

        // A reference to a cell above this row
        fns = Self::def_fn(fns, "cellabove", |_, first_arg| {
            if let Some(x) = first_arg.x() {
                Ok(first_arg
                   .clone()
                   .shift_up(1)
                   .with_x(x)
                   .into())
            } else {
                Err(InnerError::bad_input(
                        &first_arg.to_string(),
                        "Expected arg to cellabove() to have an X component" ))
            }
        });

        // A reference to a cell below this row
        fns = Self::def_fn(fns, "cellbelow", |_, first_arg| {
            if let Some(x) = first_arg.x() {
                Ok(first_arg
                   .clone()
                   .shift_down(1)
                   .with_x(x)
                   .into())
            } else {
                Err(InnerError::bad_input(
                        &first_arg.to_string(),
                        "Expected arg to cellabove() to have an X component" ))
            }
        });

        fns = Self::def_fn(fns, "celladjacent", |_, first_arg| {
            if let Some(x) = first_arg.x() {
                Ok(first_arg
                   .clone()
                   .with_x(x)
                   .into())
            } else {
                Err(InnerError::bad_input(
                        &first_arg.to_string(),
                        "Expected arg to cellabove() to have an X component" ))
            }
        });

        fns
    }

    fn def_fn<F>(
        mut fns: collections::HashMap<String, BuiltinFunction>, 
        name: &'static str, 
        eval_fn: F,
    ) -> collections::HashMap<FunctionName, BuiltinFunction>
    where F: for<'a> Fn(&'a a1_notation::A1, &'a a1_notation::A1) -> InnerResult<Node> + 'static {
        fns.insert(name.to_string(), Self {
            name: name.to_string(),
            eval: Box::new(move |current, args| {
                let first_arg = Self::verify_one_a1_arg(name, args)?;
                eval_fn(current, &first_arg)
            })
        });

        fns
    }

    /// For now all functions take a cell reference as a single arg - we can elaborate on this 
    /// in the future.
    fn verify_one_a1_arg(fn_name: &str, args: &[Ast]) -> InnerResult<a1_notation::A1> {
        if args.len() != 1 {
            Err(InnerError::bad_input(
                &args.len().to_string(), // TODO figure out a way to format this
                &format!("Expected a single argument to `{fn_name}`")))
        } else if let Node::Reference(r) = &*args[0] {
            Ok(a1_notation::new(r)?)
        } else {
            Err(InnerError::bad_input(
                    args[0].to_string().as_str(),
                    &format!("Expected a cell reference as the only argumnent to `{fn_name}`")))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_cellabove() {
        let fns = BuiltinFunction::all();
        let a1 = a1_notation::cell(0, 0);
        let cellabove = fns.get("cellabove").unwrap();

        assert_eq!(
            (cellabove.eval)(&a1, &[Box::new(Node::reference("C3"))]).unwrap(),
            Node::reference("C2"));
    }

    #[test]
    fn all_cellbelow() {
        let fns = BuiltinFunction::all();
        let a1 = a1_notation::cell(0, 0);
        let cellbelow = fns.get("cellbelow").unwrap();

        assert_eq!(
            (cellbelow.eval)(&a1, &[Box::new(Node::reference("C3"))]).unwrap(),
            Node::reference("C4"));
    }

    #[test]
    fn debug() {
        // TODO
    }
}
