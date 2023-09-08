//! # Expand
//!
use serde::{Serialize, Deserialize};
use std::cmp;

const ROW_MAX: usize = 1000;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Expand {
    pub amount: Option<usize>,
    pub start_row: a1_notation::Row,
}

impl Expand {
    pub fn clone_to_row<R: Into<a1_notation::Row>>(&self, row: R) -> Self {
        Self { amount: self.amount, start_row: row.into() }
    }

    pub fn end_row(&self) -> a1_notation::Row {
        if let Some(a) = self.amount {
            cmp::min(self.start_row.y + a, ROW_MAX).into()
        } else {
            ROW_MAX.into()
        }
    }

    pub fn expand_amount<R: Into<a1_notation::Row>>(&self, row: R) -> usize {
        self.amount.unwrap_or(ROW_MAX - row.into().y)
    }

    pub fn new<R: Into<a1_notation::Row>>(start_row: R, amount: Option<usize>) -> Self {
        Self { amount, start_row: start_row.into() }
    }
}

#[allow(clippy::from_over_into)]
impl Into<a1_notation::A1> for Expand {
    fn into(self) -> a1_notation::A1 {
        a1_notation::row_range(self.start_row, self.end_row())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_to_row() {
        let expand = Expand { amount: Some(10), start_row: 0.into() };

        assert_eq!(
            expand.clone_to_row(55),
            Expand { amount: Some(10), start_row: 55.into() });
    }

    #[test]
    fn end_row_with_amount() {
        let expand = Expand { amount: Some(10), start_row: 0.into() };

        assert_eq!(expand.end_row(), 10.into());
    }

    #[test]
    fn end_row_without_amount() {
        let expand = Expand { amount: None, start_row: 0.into() };

        assert_eq!(expand.end_row(), 1000.into());
    }

    #[test]
    fn expand_amount_finite() {
        let expand = Expand { amount: Some(5), start_row: 0.into() };

        assert_eq!(expand.expand_amount(0), 5);
        assert_eq!(expand.expand_amount(10), 5);
    }

    #[test]
    fn expand_amount_infinite() {
        let expand = Expand { amount: None, start_row: 0.into() };

        assert_eq!(expand.expand_amount(0), 1000);
        assert_eq!(expand.expand_amount(10), 990);
    }
}
