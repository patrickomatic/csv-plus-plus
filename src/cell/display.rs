use crate::Cell;
use std::fmt;

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.ast
                .clone()
                .map_or_else(|| self.value.clone(), |a| format!("={a}"))
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::*;

    #[test]
    fn display_function_call() {
        let cell = Cell {
            ast: Some(Node::fn_call("foo", &[Ast::new(1.into()), Ast::new(2.into())]).into()),
            value: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(cell.to_string(), "=foo(1, 2)");
    }

    #[test]
    fn display_infix_function_call() {
        let cell = Cell {
            ast: Some(Ast::new(Node::infix_fn_call(1, "*", 2))),
            value: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(cell.to_string(), "=(1 * 2)");
    }

    #[test]
    fn display_number() {
        let cell = Cell {
            ast: Some(Ast::new(1.into())),
            value: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(cell.to_string(), "=1");
    }
}
