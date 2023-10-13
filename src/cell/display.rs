use crate::Cell;
use std::fmt;

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_val = self
            .ast
            .clone()
            .map(|a| format!("={}", a))
            .unwrap_or_else(|| self.value.clone());

        write!(f, "{string_val}")
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::*;
    use a1_notation::Address;

    #[test]
    fn display_function_call() {
        let cell = Cell {
            ast: Some(Box::new(Node::fn_call("foo", &[1.into(), 2.into()]))),
            value: "foo".to_string(),
            position: Address::new(0, 4),
            modifier: Modifier::default(),
        };

        assert_eq!(cell.to_string(), "=foo(1, 2)");
    }

    #[test]
    fn display_infix_function_call() {
        let cell = Cell {
            ast: Some(Box::new(Node::infix_fn_call(1.into(), "*", 2.into()))),
            value: "foo".to_string(),
            position: Address::new(0, 4),
            modifier: Modifier::default(),
        };

        assert_eq!(cell.to_string(), "=(1 * 2)");
    }

    #[test]
    fn display_number() {
        let cell = Cell {
            ast: Some(Box::new(1.into())),
            value: "foo".to_string(),
            position: Address::new(0, 4),
            modifier: Modifier::default(),
        };

        assert_eq!(cell.to_string(), "=1");
    }
}
