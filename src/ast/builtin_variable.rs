//! # BuiltinVariable
//!
use super::{Node, VariableEval, VariableName};
use crate::error::EvalResult;
use std::collections;
use std::fmt;

pub(crate) struct BuiltinVariable {
    pub(crate) eval: VariableEval,
    pub(crate) name: VariableName,
}

impl BuiltinVariable {
    #[allow(dead_code)]
    fn new<F, S>(name: S, eval: F) -> Self
    where
        F: Fn(a1_notation::Address) -> EvalResult<Node> + 'static,
        S: Into<String>,
    {
        Self {
            name: name.into(),
            eval: Box::new(eval),
        }
    }

    pub fn all() -> collections::HashMap<String, BuiltinVariable> {
        /* NOTE
         * this is an example of a built-in variable, but there are no currently active ones.
         * I went through a lot of trouble to build out the concept of builtin functions but the
         * concept itself never really caught traction - all of the functions I could think of
         * could already be done with native spreadsheet functions.  And not only that, by adding
         * support for importing functions, you could just implement these all as a "standard lib"
         *
        let mut vars = collections::HashMap::new();
        vars = def_var(vars, "colleft", |a1| {
            let col: RangeOrCell = a1.shift_left(1).column.into();
            Ok(col.into())
        });
        vars
        */
        collections::HashMap::new()
    }
}

#[allow(dead_code)]
fn def_var<F, S>(
    mut vars: collections::HashMap<String, BuiltinVariable>,
    name: S,
    eval: F,
) -> collections::HashMap<String, BuiltinVariable>
where
    F: Fn(a1_notation::Address) -> EvalResult<Node> + 'static,
    S: Clone + Into<String>,
{
    vars.insert(name.clone().into(), BuiltinVariable::new(name, eval));
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
    // use super::*;
}
