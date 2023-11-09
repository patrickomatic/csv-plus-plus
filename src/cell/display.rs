use crate::Cell;
use std::fmt;

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.ast
                .clone()
                .map(|a| format!("={a}"))
                .unwrap_or_else(|| self.value.clone())
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
            ast: Some(Box::new(Node::fn_call("foo", &[1.into(), 2.into()]))),
            value: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(cell.to_string(), "=foo(1, 2)");
    }

    #[test]
    fn display_infix_function_call() {
        let cell = Cell {
            ast: Some(Box::new(Node::infix_fn_call(1.into(), "*", 2.into()))),
            value: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(cell.to_string(), "=(1 * 2)");
    }

    #[test]
    fn display_number() {
        let cell = Cell {
            ast: Some(Box::new(1.into())),
            value: "foo".to_string(),
            ..Default::default()
        };
        assert_eq!(cell.to_string(), "=1");
    }
}
