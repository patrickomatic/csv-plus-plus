use super::Rgb;
use crate::error::{RgbParseError, RgbParseResult};
use crate::parser::modifier_lexer::TokenMatch;

fn string_to_hex(token_match: &TokenMatch, hex_code: &str, double_it: bool) -> RgbParseResult<u8> {
    let hex_string = if double_it {
        hex_code.repeat(2)
    } else {
        hex_code.to_string()
    };

    u8::from_str_radix(&hex_string, 16)
        .map_err(|e| RgbParseError::new(token_match.clone(), &format!("Invalid hex: {e})")))
}

impl TryFrom<TokenMatch> for Rgb {
    type Error = RgbParseError;

    fn try_from(input: TokenMatch) -> RgbParseResult<Self> {
        let str_match = input.str_match.clone();
        let start_at = if str_match.starts_with('#') { 1 } else { 0 };
        let input_len = str_match.len() - start_at;

        if input_len == 6 {
            Ok(Rgb::new(
                string_to_hex(&input, &str_match[start_at..start_at + 2], false)?,
                string_to_hex(&input, &str_match[start_at + 2..start_at + 4], false)?,
                string_to_hex(&input, &str_match[start_at + 4..start_at + 6], false)?,
            ))
        } else if input_len == 3 {
            Ok(Rgb::new(
                string_to_hex(&input, &str_match[start_at..start_at + 1], true)?,
                string_to_hex(&input, &str_match[start_at + 1..start_at + 2], true)?,
                string_to_hex(&input, &str_match[start_at + 2..start_at + 3], true)?,
            ))
        } else {
            // return Err(ParseError::rgb_syntax_error(input, &format!("\"{input}\" must be a 3 or 6-character RGB string, optionally prefixed with '#'")));
            Err(RgbParseError::new(
                input,
                "must be a 3 or 6-character RGB string, optionally prefixed with '#'",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from_7_chars() {
        let rgb = Rgb::try_from(build_modifier_token_match("#00FF11")).unwrap();

        assert_eq!(0, rgb.r);
        assert_eq!(255, rgb.g);
        assert_eq!(17, rgb.b);
    }

    #[test]
    fn try_from_6_chars() {
        let rgb = Rgb::try_from(build_modifier_token_match("0B33F0")).unwrap();

        assert_eq!(11, rgb.r);
        assert_eq!(51, rgb.g);
        assert_eq!(240, rgb.b);
    }

    #[test]
    fn try_from_3_chars() {
        let rgb = Rgb::try_from(build_modifier_token_match("FFF")).unwrap();

        assert_eq!(255, rgb.r);
        assert_eq!(255, rgb.g);
        assert_eq!(255, rgb.b);
    }
}
