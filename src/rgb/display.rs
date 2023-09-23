use super::Rgb;
use std::fmt;

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_6_chars() {
        let rgb = Rgb {
            r: 255,
            g: 0,
            b: 17,
        };

        assert_eq!(rgb.to_string(), "#FF0011");
    }
}
