use super::{Ast, Node};
use crate::DateTime;

impl From<Node> for Ast {
    fn from(value: Node) -> Self {
        Ast::new(value)
    }
}

impl From<bool> for Ast {
    fn from(value: bool) -> Self {
        Node::Boolean(value).into()
    }
}

impl From<DateTime> for Ast {
    fn from(value: DateTime) -> Self {
        Node::DateTime(value).into()
    }
}

impl From<f64> for Ast {
    fn from(value: f64) -> Self {
        Node::Float {
            value,
            percentage: false,
            sign: None,
        }
        .into()
    }
}

impl From<isize> for Ast {
    fn from(value: isize) -> Self {
        Node::Integer {
            value: value as i64,
            percentage: false,
            sign: None,
        }
        .into()
    }
}

impl From<i64> for Ast {
    fn from(value: i64) -> Self {
        Node::Integer {
            value,
            percentage: false,
            sign: None,
        }
        .into()
    }
}

impl From<i32> for Ast {
    fn from(value: i32) -> Self {
        Node::Integer {
            value: i64::from(value),
            percentage: false,
            sign: None,
        }
        .into()
    }
}

impl From<a1_notation::A1> for Ast {
    fn from(value: a1_notation::A1) -> Self {
        Node::Reference(value.to_string()).into()
    }
}

impl From<a1_notation::Address> for Ast {
    fn from(value: a1_notation::Address) -> Self {
        Node::Reference(value.to_string()).into()
    }
}

impl From<a1_notation::RangeOrCell> for Ast {
    fn from(value: a1_notation::RangeOrCell) -> Self {
        Node::Reference(value.to_string()).into()
    }
}

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<DateTime> for Node {
    fn from(value: DateTime) -> Self {
        Self::DateTime(value)
    }
}

impl From<f64> for Node {
    fn from(value: f64) -> Self {
        Self::Float {
            value,
            percentage: false,
            sign: None,
        }
    }
}

impl From<isize> for Node {
    fn from(value: isize) -> Self {
        Self::Integer {
            value: value as i64,
            percentage: false,
            sign: None,
        }
    }
}

impl From<i64> for Node {
    fn from(value: i64) -> Self {
        Self::Integer {
            value,
            percentage: false,
            sign: None,
        }
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Self::Integer {
            value: i64::from(value),
            percentage: false,
            sign: None,
        }
    }
}

impl From<a1_notation::A1> for Node {
    fn from(value: a1_notation::A1) -> Self {
        Self::Reference(value.to_string())
    }
}

impl From<a1_notation::Address> for Node {
    fn from(value: a1_notation::Address) -> Self {
        Self::Reference(value.to_string())
    }
}

impl From<a1_notation::RangeOrCell> for Node {
    fn from(value: a1_notation::RangeOrCell) -> Self {
        Self::Reference(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_from_a1_notation() {
        let a1 = a1_notation::new("A1").unwrap();
        assert_eq!(Node::reference("A1"), a1.into());
    }
}
