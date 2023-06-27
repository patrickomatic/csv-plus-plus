use std::str::FromStr;

use crate::{Error, Result};
use super::A1;
use super::position::Position;
use super::range_or_cell::RangeOrCell;

#[derive(Debug, Default)]
pub struct RangeBuilder {
    parent_builder: A1Builder,
    from: Option<A1>,
    to: Option<A1>,
}

#[derive(Debug, Default)]
pub struct A1Builder {
    sheet_name: Option<String>,
    cell: Option<RangeOrCell>,
}

impl A1Builder {
    pub fn build(self) -> Result<A1> {
        if let Some(c) = self.cell {
            Ok(A1 {
                reference: c,
                sheet_name: self.sheet_name,
            })
        } else {
            Err(Error::A1BuilderError("`build()` called without any reference set".to_owned()))
        }
    }

    pub fn a1str(mut self, sheet_name: &str) -> Result<Self> {
        let a1 = A1::from_str(sheet_name)?;

        self.cell = Some(a1.reference);
        self.sheet_name = a1.sheet_name;

        Ok(self)
    }

    pub fn range(self) -> RangeBuilder {
        RangeBuilder { 
            parent_builder: self, 
            ..Default::default()
        }
    }

    pub fn sheet_name(mut self, sheet_name: &str) -> Self {
        self.sheet_name = Some(sheet_name.to_string());
        self
    }


    pub fn x(mut self, v: usize) -> Self {
        self.cell = Some(RangeOrCell::Cell(Position::ColumnRelative(v)));
        self
    }

    pub fn xy(mut self, xv: usize, yv: usize) -> Self {
        self.cell = Some(RangeOrCell::Cell(Position::Absolute(xv, yv)));
        self
    }

    pub fn y(mut self, v: usize) -> Self {
        self.cell = Some(RangeOrCell::Cell(Position::RowRelative(v)));
        self
    }
}

impl RangeBuilder {
    pub fn from(mut self, b: A1) -> Self {
        self.from = Some(b);
        self
    }

    pub fn to(mut self, b: A1) -> Self {
        self.to = Some(b);
        self
    }

    /// for `build()` to pass, it requires that both `from()` and `to()` have been called with
    /// cell-based `RangeOrCell`s
    pub fn build(self) -> Result<A1> {
        let parent_builder = self.parent_builder;

        // TODO ugh I hate this
        if let Some(from) = self.from {
            if let Some(to) = self.to {
                return match from.reference {
                    RangeOrCell::Cell(from_p) => {
                        match to.reference {
                            RangeOrCell::Cell(to_p) => 
                                Ok(A1 {
                                    sheet_name: parent_builder.sheet_name,
                                    reference: RangeOrCell::Range { 
                                        from: from_p,
                                        to: to_p,
                                    },
                                }),
                            _ => Err(Error::A1BuilderError("`to` must be a cell and not a range".to_owned())),
                        }
                    },
                    _ => Err(Error::A1BuilderError("`from` must be a cell and not a range".to_owned())),
                }
            }
        } 

        Err(Error::A1BuilderError("You must specify both `to` and `from` to build a range".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::position::*;
    use super::super::range_or_cell::*;

    #[test]
    fn build_a1str() {
        let a1 = A1Builder::default().a1str("A1").unwrap().build().unwrap();

        assert_eq!(a1, A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell(Position::Absolute(0, 0)),
        });
    }

    #[test]
    fn build_x() {
        let a1 = A1Builder::default().x(1).build().unwrap();

        assert_eq!(a1, A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell(Position::ColumnRelative(1)),
        });
    }

    #[test]
    fn build_xy() {
        let a1 = A1Builder::default().xy(1, 2).build().unwrap();

        assert_eq!(a1, A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell(Position::Absolute(1, 2)),
        });
    }

    #[test]
    fn build_y() {
        let a1 = A1Builder::default().y(1).build().unwrap();

        assert_eq!(a1, A1 {
            sheet_name: None,
            reference: RangeOrCell::Cell(Position::RowRelative(1)),
        });
    }

    #[test]
    fn build_range() {
        // TODO
    }
}
