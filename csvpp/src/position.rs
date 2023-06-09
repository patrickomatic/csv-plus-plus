use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn is_first_cell(&self) -> bool {
        self.0 == 0
    }
}
