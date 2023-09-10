//! # BuiltinFunction
//!
use crate::{InnerError, InnerResult};
use std::collections;
use std::fmt;
use std::str::FromStr;
use super::{Ast, FunctionEval, FunctionName, Node};

pub struct BuiltinFunction {
    pub eval: FunctionEval,
    pub name: FunctionName, 
}

impl BuiltinFunction {
    pub fn all() -> collections::HashMap<FunctionName, BuiltinFunction> {
        let mut fns = collections::HashMap::new();

        // A reference to a cell above this row
        fns = def_fn(fns, "cellabove", |current, args| {
            let column = verify_one_column("cellabove", args)?;
            Ok(current.shift_up(1).with_x(column.x).into())
        });

        // A reference to a cell below this row
        fns = def_fn(fns, "cellbelow", |current, args| {
            let column = verify_one_column("cellbelow", args)?;
            Ok(current.shift_down(1).with_x(column.x).into())
        });

        // A reference to a cell in the current row
        fns = def_fn(fns, "celladjacent", |current, args| {
            let column = verify_one_column("celladjacent", args)?;
            Ok(current.with_x(column.x).into())
        });

        fns
    }
}

/// NOTE: Debug is manually implemented because we can't derive it for `eval`.
impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuiltinFunction")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

fn def_fn<F>(
    mut fns: collections::HashMap<String, BuiltinFunction>, 
    name: &'static str, 
    eval_fn: F,
) -> collections::HashMap<FunctionName, BuiltinFunction>
where F: Fn(a1_notation::Address, &[Ast]) -> InnerResult<Node> + 'static {
    fns.insert(name.to_string(), BuiltinFunction {
        name: name.to_string(),
        eval: Box::new(move |current, args| eval_fn(current, args))
    });

    fns
}

fn verify_one_column(fn_name: &str, args: &[Ast]) -> InnerResult<a1_notation::Column> {
    if args.len() != 1 {
        Err(InnerError::bad_input(
            &args.len().to_string(), // TODO figure out a way to format this
            &format!("Expected a single argument to `{fn_name}`")))
    } else if let Node::Reference(r) = &*args[0] {
        Ok(a1_notation::Column::from_str(r)?)
    } else {
        Err(InnerError::bad_input(
                args[0].to_string().as_str(),
                &format!("Expected a cell reference as the only argumnent to `{fn_name}`")))
    }
}

#[cfg(test)]
mod tests {
    use a1_notation::Address;
    use super::*;

    #[test]
    fn all_cellabove() {
        let fns = BuiltinFunction::all();
        let current = Address::new(0, 1);
        let cellabove = fns.get("cellabove").unwrap();

        assert_eq!(
            (cellabove.eval)(current, &[Box::new(Node::reference("C"))]).unwrap(),
            Node::reference("C1"));
    }

    #[test]
    fn all_celladjacent() {
        let fns = BuiltinFunction::all();
        let current = Address::new(0, 1);
        let celladjacent = fns.get("celladjacent").unwrap();

        assert_eq!(
            (celladjacent.eval)(current, &[Box::new(Node::reference("C"))]).unwrap(),
            Node::reference("C2"));
    }

    #[test]
    fn all_cellbelow() {
        let fns = BuiltinFunction::all();
        let current = Address::new(0, 1);
        let cellbelow = fns.get("cellbelow").unwrap();

        assert_eq!(
            (cellbelow.eval)(current, &[Box::new(Node::reference("C"))]).unwrap(),
            Node::reference("C3"));
    }
}
