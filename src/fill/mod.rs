//! # Fill
//!
use a1::Row;
use serde::{Deserialize, Serialize};
use std::cmp;

mod into;

pub(crate) const ROW_MAX: usize = 1000;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Fill {
    pub amount: Option<usize>,
    pub start_row: Row,
}

impl Fill {
    #[must_use]
    pub(crate) fn clone_to_row<R: Into<Row>>(&self, row: R) -> Self {
        Self {
            amount: self.amount,
            start_row: row.into(),
        }
    }

    #[must_use]
    pub(crate) fn end_row(&self) -> Row {
        if let Some(a) = self.amount {
            cmp::min(self.start_row.y + a, ROW_MAX).into()
        } else {
            ROW_MAX.into()
        }
    }

    pub fn new<R: Into<Row>>(start_row: R, amount: Option<usize>) -> Self {
        Self {
            amount,
            start_row: start_row.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_to_row() {
        let fill = Fill {
            amount: Some(10),
            start_row: 0.into(),
        };

        assert_eq!(
            fill.clone_to_row(55),
            Fill {
                amount: Some(10),
                start_row: 55.into()
            }
        );
    }

    #[test]
    fn end_row_with_amount() {
        let fill = Fill {
            amount: Some(10),
            start_row: 0.into(),
        };

        assert_eq!(fill.end_row(), 10.into());
    }

    #[test]
    fn end_row_without_amount() {
        let fill = Fill {
            amount: None,
            start_row: 0.into(),
        };

        assert_eq!(fill.end_row(), 1000.into());
    }
}
