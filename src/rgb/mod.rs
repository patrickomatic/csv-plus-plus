//! # Rgb
//!
//! RGB-parsing and formatting functionality
use serde::{Deserialize, Serialize};

mod display;
mod from;
mod from_str;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
