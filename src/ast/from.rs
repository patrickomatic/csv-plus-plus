use a1_notation::{A1, Address, RangeOrCell};
use super::Node;

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        Node::Boolean(value)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Node {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self::DateTime(value)
    }
}

impl From<f64> for Node {
    fn from(value: f64) -> Self {
        Node::Float(value)
    }
}

impl From<isize> for Node {
    fn from(value: isize) -> Self {
        Node::Integer(value as i64)
    }
}

impl From<i64> for Node {
    fn from(value: i64) -> Self {
        Node::Integer(value)
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Integer(value as i64)
    }
}

impl From<A1> for Node {
    fn from(value: A1) -> Self {
        Node::Reference(value.to_string())
    }
}

impl From<Address> for Node {
    fn from(value: Address) -> Self {
        Node::Reference(value.to_string())
    }
}

impl From<RangeOrCell> for Node {
    fn from(value: RangeOrCell) -> Self {
        Node::Reference(value.to_string())
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
