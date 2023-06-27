//! # A1
//!
//! A position (location of a cell or range of cells) in a spreadsheet.  (0, 0) is the top left of
//! the spreadsheet. A lot of the logic here involves converting to and from A1-notation to a X/Y
//! based canonical representation.
//!
//! ### Links
//!
//! * [Google Sheets API Overview](https://developers.google.com/sheets/api/guides/concepts)
//! * [Refer to Cells and Ranges by Using A1 Notation](https://learn.microsoft.com/en-us/office/vba/excel/concepts/cells-and-ranges/refer-to-cells-and-ranges-by-using-a1-notation)
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str;

use crate::{Error, Result};
use super::a1_builder::A1Builder;
use super::range_or_cell::RangeOrCell;
use super::position::Position;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct A1 {
    pub sheet_name: Option<String>,
    pub reference: RangeOrCell,
}

impl A1 {
    pub fn builder() -> A1Builder {
        A1Builder::default()
    }

    pub fn x(&self) -> Option<usize> {
        match self.cell_reference()? {
            Position::Absolute(x, _) | Position::ColumnRelative(x) => Some(x),
            _ => None,
        }
    }

    pub fn y(&self) -> Option<usize> {
        match self.cell_reference()? {
            Position::Absolute(_, y) | Position::RowRelative(y) => Some(y),
            _ => None,
        }
    }

    pub fn xy(&self) -> Option<(usize, usize)> {
        match self.cell_reference()? {
            Position::Absolute(x, y) => Some((x, y)),
            _ => None,
        }
    }

    fn cell_reference(&self) -> Option<Position> {
        match self.reference {
            RangeOrCell::Cell(p) => Some(p),
            _ => None,
        }
    }

    fn parse_sheet_name(a1: &str) -> Result<(Option<String>, &str)> {
        if let Some((sheet_name, rest)) = a1.split_once('!') {
            Ok((Some(sheet_name.to_string()), rest))
        } else {
            Ok((None, a1))
        }
    }
}

impl str::FromStr for A1 {
    type Err = Error;

    /// Parse A1-format.
    ///
    /// This code can parse a variety of A1 formats.  The most simple is just a direct cell
    /// reference:
    ///
    /// ```
    /// ```
    ///
    // TODO: 
    //
    // * handle Sheet1!A1 style
    //
    // * handle commas? it might make it annoying to use this lib if the common cases is
    // 	 assuming a vector of ranges
    fn from_str(a1: &str) -> Result<Self> {
        let (sheet_name, rest) = Self::parse_sheet_name(a1)?;
        let reference = RangeOrCell::from_str(rest)?;

        Ok(A1 { sheet_name, reference })
    }
}

impl fmt::Display for A1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(sheet_name) = &self.sheet_name {
            write!(f, "{}!{}", sheet_name, self.reference)
        } else {
            write!(f, "{}", self.reference)
        }
    }
}
