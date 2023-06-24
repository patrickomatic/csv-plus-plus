use serde::{Serialize, Deserialize};
use std::fmt;

/// # Position
///
/// A position (location of a cell or range of cells) in a spreadsheet.  (0, 0) is the top left of
/// the spreadsheet.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Position {
    /// Absolute(x, y)
    ///
    /// * `x` - The row index
    /// * `y` - The column index
    Absolute(usize, usize),

    /// Relative(y)
    ///
    /// * `y` - The column index
    Relative(usize),
}

static ALPHA: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Position {
    /// Only the first cell in a row can have a row modifier:
    ///
    /// ```
    /// use csvpp::Position;
    ///
    /// assert!(Position::Absolute(0, 0).can_have_row_modifier());
    /// assert!(Position::Absolute(0, 5).can_have_row_modifier());
    /// ```
    ///
    /// but not cells later in the row:
    ///
    /// ```
    /// use csvpp::Position;
    ///
    /// assert!(!Position::Absolute(5, 0).can_have_row_modifier());
    /// assert!(!Position::Absolute(1, 5).can_have_row_modifier());
    /// ```
    ///
    /// or relative cells:
    ///
    /// ```
    /// use csvpp::Position;
    ///
    /// assert!(!Position::Relative(0).can_have_row_modifier());
    /// ```
    pub fn can_have_row_modifier(&self) -> bool {
        match self {
            Self::Absolute(x, _) => *x == 0,
            _ => false,
        }
    }

    pub fn from_a1(&self) -> String {
        // TODO
        // (cell_ref.upcase.chars.reduce(0) do |cell_index, letter|
        //  (cell_index * 26) + ::T.must(ALPHA.find_index(letter)) + 1
        // end) - 1

        todo!()
    }

    /// Converts a cell position to A1 notation. The basic idea with A1 notation is that the row is
    /// represented by a letter A-Z and the column numerically, with the first position being `1`
    /// (not `0`).  So for example origin is `A1`:
    ///
    /// ```
    /// use csvpp::Position;
    ///
    /// assert_eq!(Position::Absolute(0, 0).to_a1(), "A1");
    /// ```
    ///
    /// And the position (1, 5) gives us `F2`. (F is the fifth letter, and 2 is the second cell
    /// when you start at 1):
    ///
    /// ```
    /// use csvpp::Position;
    ///
    /// assert_eq!(Position::Absolute(1, 5).to_a1(), "F2");
    /// ```
    ///
    /// For relative cells we just have the alpha component:
    ///
    /// ```
    /// use csvpp::Position;
    ///
    /// assert_eq!(Position::Relative(0).to_a1(), "A");
    /// ```
    ///
    /// and the final complication is once we get past column 26, we'll have to start stacking the
    /// letters:
    ///
    /// ```
    /// use csvpp::Position;
    ///
    /// assert_eq!(Position::Relative(25).to_a1(), "Z");
    /// assert_eq!(Position::Relative(26).to_a1(), "AA");
    /// assert_eq!(Position::Relative(27).to_a1(), "AB");
    /// ```
    ///
    /// ### Links
    ///
    /// * [Google Sheets API Overview](https://developers.google.com/sheets/api/guides/concepts)
    /// * [Refer to Cells and Ranges by Using A1 Notation](https://learn.microsoft.com/en-us/office/vba/excel/concepts/cells-and-ranges/refer-to-cells-and-ranges-by-using-a1-notation)
    pub fn to_a1(&self) -> String {
        // convert to the "A" part - 0 == 'A', 1 == 'B', etc.  we'll append to a string because
        // if it's larger than 26, we'll have additional characters like AA1
        let mut row_part = String::from("");
        let mut c = match self {
            Self::Absolute(_, y) => *y,
            Self::Relative(y) => *y,
        };

        loop {
            row_part = format!("{}{}", ALPHA[c % 26], row_part);

            let next_c = ((c as f64 / 26.0).floor() as isize) - 1;
            if next_c < 0 {
                break;
            } 

            c = next_c as usize;
        }

        match self {
            Self::Absolute(x, _) => {
                // this is the "1" part of "A1" which is easier because it's just our column 
                // index offset by 1 instead of 0
                let column_part = (x + 1).to_string();
                format!("{}{}", row_part, column_part)
            },
            Self::Relative(_) => row_part,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Absolute(x, y) => write!(f, "[{}, {}]", x, y),
            Self::Relative(y) => write!(f, "[{}]", y),
        }
    }
}
