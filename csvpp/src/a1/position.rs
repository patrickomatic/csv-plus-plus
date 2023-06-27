//! # Position
//!
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str;

use crate::{Error, Result};

static ALPHA: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Position {
    /// Absolute(x, y)
    ///
    /// * `x` - The column index
    /// * `y` - The row index
    Absolute(usize, usize),

    /// RowRelative(y)
    ///
    /// * `y` - The row index
    RowRelative(usize),

    /// ColumnRelative(y)
    ///
    /// * `x` - The column index
    ColumnRelative(usize),
}

impl Position {
    /// Only the first cell in a row can have a row modifier:
    ///
    /// ```skip
    /// # use csvpp::a1::position::Position;
    ///
    /// assert!(Position::Absolute(0, 0).can_have_row_modifier());
    /// assert!(Position::Absolute(0, 5).can_have_row_modifier());
    /// ```
    ///
    /// but not cells later in the row:
    ///
    /// ```skip
    /// # use csvpp::a1::position::Position;
    ///
    /// assert!(!Position::Absolute(5, 0).can_have_row_modifier());
    /// assert!(!Position::Absolute(1, 5).can_have_row_modifier());
    /// ```
    ///
    /// or relative cells:
    ///
    /// ```skip
    /// # use csvpp::a1::position::Position;
    ///
    /// assert!(!Position::RowRelative(0).can_have_row_modifier());
    /// ```
    // XXX this needs to move onto the cell
    pub fn can_have_row_modifier(&self) -> bool {
        match self {
            Self::Absolute(x, _) => *x == 0,
            _ => false,
        }
    }

    /// This function assumes that you've consumed the first part (the "A") of the A1 string and
    /// now we're just consuming the integer part
    fn parse_a1_x(a1: &str) -> Result<Option<usize>> {
        if !a1.ends_with(|c: char| c.is_ascii_digit()) {
            return Ok(None)
        };

        let n = match a1.parse::<usize>() {
            Ok(n) => n,
            Err(e) => return Err(Error::CodeSyntaxError {
                bad_input: a1.to_owned(), 
                line_number: 0, // XXX
                message: format!("Error parsing number part of A1 reference: {:?}", e),
            }),
        };

        if n < 1 {
            return Err(Error::CodeSyntaxError {
                bad_input: n.to_string(),
                line_number: 0, // XXX
                message: "A1 reference must be greater than 0".to_owned(),
            })
        }
        
        Ok(Some(n - 1))
    }

    fn parse_a1_y(a1: &str) -> Result<(Option<usize>, &str)> {
        if !a1.starts_with(|c: char| c.is_ascii_alphabetic()) {
            return Ok((None, a1))
        };

        let mut consumed = 0;
        let mut y = 0;
        for ch in a1.chars() {
            let uch = ch.to_ascii_uppercase();
            if let Some(ch_index) = ALPHA.iter().position(|&c| c == uch) {
                consumed += 1;
                y = y * 26 + ch_index + 1;
            } else if ch.is_ascii_digit() {
                break
            } else {
                return Err(Error::CodeSyntaxError { 
                    bad_input: ch.to_string(), 
                    line_number: 0,  // XXX
                    message: format!("Invalid character in A1 notation: {}", a1),
                })
            }
        }

        if consumed == 0 {
            Ok((None, a1))
        } else {
            Ok((Some(y - 1), &a1[consumed..]))
        }
    }

    /// Convert to the "A" part - 0 == 'A', 1 == 'B', etc.  we'll append to a string because
    /// if it's larger than 26, we'll have additional characters like AA1
    fn to_a1_left(&self) -> String {
        match self {
            Self::Absolute(_, y) | Self::RowRelative(y) => {
                let mut row_part = String::from("");
                let mut c = *y;
                
                loop {
                    row_part = format!("{}{}", ALPHA[c % 26], row_part);

                    let next_c = ((c as f64 / 26.0).floor() as isize) - 1;
                    if next_c < 0 {
                        break;
                    } 

                    c = next_c as usize;
                }

                row_part
            },
            Self::ColumnRelative(_) => self.to_a1_right(),
        }
    }

    /// This is the "1" part of "A1" which is easier because it's just our column index offset 
    /// by 1 instead of 0
    fn to_a1_right(&self) -> String {
        match self {
            Self::Absolute(x, _) | Self::ColumnRelative(x) => (x + 1).to_string(),
            Self::RowRelative(_) => self.to_a1_left(),
        }
    }
}

impl str::FromStr for Position {
    type Err = Error;

    fn from_str(a1: &str) -> Result<Self> {
        let (y, rest) = Self::parse_a1_y(a1)?;
        let x = Self::parse_a1_x(rest)?;

        if let Some(x) = x {
            if let Some(y) = y {
                Ok(Self::Absolute(x, y))
            } else {
                Ok(Self::ColumnRelative(x))
            }
        } else if let Some(y) = y {
            Ok(Self::RowRelative(y))
        } else {
            Err(Error::CodeSyntaxError {
                bad_input: a1.to_owned(),
                line_number: 0, // XXX
                message: "Error parsing A1 notation: could not determine a row or column".to_owned(),
            })
        }
    }

}

impl fmt::Display for Position {
    /// Converts a cell position to a String. The basic idea with A1 notation is that the row is
    /// represented by a letter A-Z and the column numerically, with the first position being `1`
    /// (not `0`).  So for example origin is `A1`:
    ///
    /// ```skip
    /// use csvpp::Position;
    ///
    /// assert_eq!(Position::Absolute(0, 0).to_a1(), "A1");
    /// ```
    ///
    /// And the position (1, 5) gives us `F2`. (F is the fifth letter, and 2 is the second cell
    /// when you start at 1):
    ///
    /// ```skip
    /// use csvpp::Position;
    ///
    /// assert_eq!(Position::Absolute(1, 5).to_a1(), "F2");
    /// ```
    ///
    /// For relative cells we just have the alpha *or* numeric component:
    ///
    /// ```skip
    /// use csvpp::Position;
    ///
    /// assert_eq!(Position::RowRelative(0).to_a1(), "A:A");
    /// assert_eq!(Position::ColumnRelative(0).to_a1(), "1:1");
    /// ```
    ///
    /// another complication is once we get past column 26, we'll have to start stacking the letters:
    /// ```skip
    /// use csvpp::Position;
    ///
    /// assert_eq!(Position::Absolute(0, 25).to_a1(), "Z1");
    /// assert_eq!(Position::Absolute(0, 26).to_a1(), "AA1");
    /// assert_eq!(Position::Absolute(0, 27).to_a1(), "AB1");
    /// ```
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let left = self.to_a1_left();
        let right = self.to_a1_right();
        let separator = match self {
            Self::RowRelative(_) | Self::ColumnRelative(_) => ":",
            _ => "",
        };

        write!(f, "{}{}{}", left, separator, right)
    }
}
