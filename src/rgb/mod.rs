//! # Rgb
//!
//! RGB-parsing and formatting functionality
use serde::{Deserialize, Serialize};

mod display;
mod from;
mod try_from;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    // TODO: allow for an alpha channel
}

impl Rgb {
    pub(crate) fn to_rgba(&self) -> String {
        format!("{:02X}{:02X}{:02X}FF", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_rgba() {
        let rgb = Rgb {
            r: 255,
            g: 0,
            b: 17,
        };

        assert_eq!(rgb.to_rgba(), "FF0011FF");
    }
}
