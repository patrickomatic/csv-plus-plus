//! RGB-parsing and formatting functionality
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

use crate::{Error, Position, Result};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

fn string_to_hex(hex_code: &str, double_it: bool) -> Result<u8> {
    let hex_string = if double_it {
        hex_code.repeat(2)
    } else {
        hex_code.to_string()
    };

    match u8::from_str_radix(&hex_string, 16) {
        Ok(n) => Ok(n),
        Err(e) => Err(Error::ModifierSyntaxError {
            bad_input: hex_code.to_string(),
            message: format!("Invalid hex: {}", e),
            index: Position::Absolute(0, 0), // XXX
        }),
    }
}

impl FromStr for Rgb {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let start_at = if input.starts_with("#") { 1 } else { 0 };
        let input_len = input.len() - start_at;

        let rgb = if input_len == 6 {
            Rgb {
                r: string_to_hex(&input[start_at   .. start_at+2], false)?,
                g: string_to_hex(&input[start_at+2 .. start_at+4], false)?,
                b: string_to_hex(&input[start_at+4 .. start_at+6], false)?,
            }
        } else if input_len == 3 {
            Rgb {
                r: string_to_hex(&input[start_at   .. start_at+1], true)?,
                g: string_to_hex(&input[start_at+1 .. start_at+2], true)?,
                b: string_to_hex(&input[start_at+2 .. start_at+3], true)?,
            }
        } else {
            return Err(Error::ModifierSyntaxError {
                message: format!(
                    "\"{}\" must be a 3 or 6-character RGB string, optionally prefixed with '#'", 
                    input,
                ),
                bad_input: input.to_string(),
                index: Position::Absolute(0, 0), // XXX
            })
        };

        Ok(rgb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_7_chars() {
        let rgb = Rgb::from_str("#00FF11").unwrap();

        assert_eq!(0, rgb.r);
        assert_eq!(255, rgb.g);
        assert_eq!(17, rgb.b);
    }

    #[test]
    fn from_str_6_chars() {
        let rgb = Rgb::from_str("0B33F0").unwrap();

        assert_eq!(11, rgb.r);
        assert_eq!(51, rgb.g);
        assert_eq!(240, rgb.b);
    }

    #[test]
    fn from_str_3_chars() {
        let rgb = Rgb::from_str("FFF").unwrap();

        assert_eq!(255, rgb.r);
        assert_eq!(255, rgb.g);
        assert_eq!(255, rgb.b);
    }

    #[test]
    fn display_3_chars() {
        let rgb = Rgb { r: 255, g: 0, b: 17 };

        assert_eq!(rgb.to_string(), "#FF0011")
    }
}
