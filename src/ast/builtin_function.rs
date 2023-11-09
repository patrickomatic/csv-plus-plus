//! # BuiltinFunction
//!
use super::{Ast, FunctionEval, FunctionName, Node};
use crate::error::{EvalError, EvalResult};
use std::collections;
use std::fmt;
use std::str::FromStr;

pub(crate) struct BuiltinFunction {
    pub(crate) eval: FunctionEval,
    pub(crate) name: FunctionName,
}

impl BuiltinFunction {
    pub fn all() -> collections::HashMap<FunctionName, BuiltinFunction> {
        /* NOTE:
         * this is an example of a built-in function, but there are no currently active ones.
         * I went through a lot of trouble to build out the concept of builtin functions but the
         * concept itself never really caught traction - all of the functions I could think of
         * could already be done with native spreadsheet functions.  And not only that, by adding
         * support for importing functions, you could just implement these all as a "standard lib"
         *
        let fns = collections::HashMap::new();
        fns = def_fn(fns, "cellabove", |current, args| {
            let column = verify_one_column("cellabove", args, current)?;
            Ok(current.shift_up(1).with_x(column.x).into())
        });
        fns
        */
        collections::HashMap::new()
    }
}

// NOTE: Debug is manually implemented because we can't derive it for `eval`.
impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuiltinFunction")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

#[allow(dead_code)]
fn def_fn<F, S>(
    mut fns: collections::HashMap<String, BuiltinFunction>,
    name: S,
    eval_fn: F,
) -> collections::HashMap<FunctionName, BuiltinFunction>
where
    F: Fn(a1_notation::Address, &[Ast]) -> EvalResult<Node> + 'static,
    S: Into<String> + Clone,
{
    fns.insert(
        name.clone().into(),
        BuiltinFunction {
            name: name.into(),
            eval: Box::new(move |current, args| eval_fn(current, args)),
        },
    );

    fns
}

#[allow(dead_code)]
fn verify_one_column(
    fn_name: &str,
    args: &[Ast],
    position: a1_notation::Address,
) -> EvalResult<a1_notation::Column> {
    if args.len() != 1 {
        Err(EvalError::new(
            position,
            format!("Expected a single argument to `{fn_name}`"),
        ))
    } else if let Node::Reference(r) = &*args[0] {
        Ok(a1_notation::Column::from_str(r).map_err(|e| {
            EvalError::new(
                position,
                format!("Expected an A1 reference as the first argument: {e}"),
            )
        })?)
    } else {
        Err(EvalError::new(
            position,
            format!("Expected a cell reference as the only argumnent to `{fn_name}`"),
        ))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
