//! # RangeOrCell
//!
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str;

use crate::{Error, Result};
use super::position::Position;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RangeOrCell {
    Range { 
        from: Position, 
        to: Position,
    },
    Cell(Position),
}

impl str::FromStr for RangeOrCell {
    type Err = Error;

    fn from_str(a1: &str) -> Result<Self> {
        if let Some((l, r)) = a1.split_once(':') {
            Ok(RangeOrCell::Range {
                from: Position::from_str(l)?,
                to: Position::from_str(r)?,
            })
        } else {
            Ok(RangeOrCell::Cell(Position::from_str(a1)?))
        }
    }
}

impl fmt::Display for RangeOrCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Range { from, to } =>
                write!(f, "{}:{}", from, to),
            Self::Cell(p) =>
                write!(f, "{}", p),
        }
    }
}
