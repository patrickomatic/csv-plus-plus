//!
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Expand {
    pub amount: Option<usize>,
}
