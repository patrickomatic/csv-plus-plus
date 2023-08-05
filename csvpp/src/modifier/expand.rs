//! # Expand
//!
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Expand {
    pub amount: Option<usize>,
}

impl Expand {
    pub fn expand_amount(&self) -> usize {
        self.amount.unwrap_or(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_amount_finite() {
        let expand = Expand { amount: Some(5) };
        assert_eq!(expand.expand_amount(), 5);
    }

    #[test]
    fn expand_amount_infinite() {
        let expand = Expand { amount: None };
        assert_eq!(expand.expand_amount(), 1000);
    }
}
