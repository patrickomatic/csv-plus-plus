use super::Rgb;
use crate::error::{BadInput, ParseError, ParseResult};
use crate::parser::cell_lexer::TokenMatch;

fn string_to_hex(
    hex_code: &str,
    double_it: bool,
) -> std::result::Result<u8, std::num::ParseIntError> {
    let hex_string = if double_it {
        hex_code.repeat(2)
    } else {
        hex_code.to_string()
    };

    u8::from_str_radix(&hex_string, 16)
}

fn parse_str(str_match: &str) -> std::result::Result<(u8, u8, u8), std::num::ParseIntError> {
    if str_match.len() == 6 {
        Ok((
            string_to_hex(&str_match[..2], false)?,
            string_to_hex(&str_match[2..4], false)?,
            string_to_hex(&str_match[4..6], false)?,
        ))
    } else {
        Ok((
            string_to_hex(&str_match[..1], true)?,
            string_to_hex(&str_match[1..2], true)?,
            string_to_hex(&str_match[2..3], true)?,
        ))
    }
}

impl TryFrom<TokenMatch> for Rgb {
    type Error = ParseError;

    fn try_from(input: TokenMatch) -> ParseResult<Self> {
        let str_match = input.str_match.clone();
        let start_at = usize::from(str_match.starts_with('#'));
        let input_len = str_match.len() - start_at;

        if input_len == 3 || input_len == 6 {
            let (r, g, b) = parse_str(&str_match[start_at..])
                .map_err(|e| input.into_parse_error(format!("Error parsing hex string: {e}")))?;
            Ok(Rgb::new(r, g, b))
        } else {
            Err(input.into_parse_error(
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
        let rgb = Rgb::try_from(build_cell_token_match("#00FF11")).unwrap();
        assert_eq!(0, rgb.r);
        assert_eq!(255, rgb.g);
        assert_eq!(17, rgb.b);
    }

    #[test]
    fn try_from_6_chars() {
        let rgb = Rgb::try_from(build_cell_token_match("0B33F0")).unwrap();
        assert_eq!(11, rgb.r);
        assert_eq!(51, rgb.g);
        assert_eq!(240, rgb.b);
    }

    #[test]
    fn try_from_3_chars() {
        let rgb = Rgb::try_from(build_cell_token_match("FFF")).unwrap();
        assert_eq!(255, rgb.r);
        assert_eq!(255, rgb.g);
        assert_eq!(255, rgb.b);
    }
}
