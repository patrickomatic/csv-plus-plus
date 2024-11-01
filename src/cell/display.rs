use crate::Cell;
use std::fmt;

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.ast
                .as_ref()
                .map_or_else(|| self.parsed_value.clone(), |a| format!("={a}"))
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn display_function_call() {
        let mut cell = Cell::new(build_field("foo", (0, 0)));
        cell.ast = Some(Node::fn_call("foo", &[Ast::new(1), Ast::new(2)]).into());

        assert_eq!(cell.to_string(), "=foo(1, 2)");
    }

    #[test]
    fn display_infix_function_call() {
        let mut cell = Cell::new(build_field("foo", (0, 0)));
        cell.ast = Some(Ast::new(Node::infix_fn_call(1, "*", 2)));

        assert_eq!(cell.to_string(), "=(1 * 2)");
    }

    #[test]
    fn display_number() {
        let mut cell = Cell::new(build_field("foo", (0, 0)));
        cell.ast = Some(Ast::new(1));

        assert_eq!(cell.to_string(), "=1");
    }
}
