use super::{DataValidation, ModifierParser, Token};
use crate::error::{BadInput, ModifierParseError, ParseError, ParseResult};
use crate::DateTime;

macro_rules! take_parens {
    ($self:ident, $tt:tt) => {{
        $self.lexer.take_token(Token::OpenParenthesis)?;
        let _res = $tt;
        $self.lexer.take_token(Token::CloseParenthesis)?;
        _res
    }};
}

// TODO: keep making this more general, handle commas/multiple
macro_rules! validate_args {
    ($self:ident, $From:ident, $tok:path $(, $toks:path)*) => {{
        $From::try_from($self.lexer.take_token($tok)?)?
    }};
}

macro_rules! validate {
    ($self:ident, $variant:ident, $tt:tt) => {
        fn $variant(&mut $self) -> ParseResult<()> {
            $self.modifier.data_validation = Some($tt);
            Ok(())
        }
    };
}

impl ModifierParser<'_, '_> {
    pub(super) fn validate(&mut self) -> ParseResult<()> {
        let name = self.lexer.take_modifier_right_side()?;

        match name.str_match.to_lowercase().as_str() {
            "c" | "custom" => self.validate_custom(),
            "date_gt" | "date_after" => self.validate_date_after(),
            "date_lt" | "date_before" => self.validate_date_before(),
            "date_btwn" | "date_between" => self.validate_date_between(),
            "date_eq" | "date_equal_to" => self.validate_date_equal_to(),
            "is_date" | "date_is_valid" => self.validate_date_is_valid(),
            "date_nbtwn" | "date_not_between" => self.validate_date_not_between(),
            "date_gte" | "date_on_or_after" => self.validate_date_on_or_after(),
            "date_lte" | "date_on_or_before" => self.validate_date_on_or_before(),
            "num_btwn" | "number_between" => self.validate_number_between(),
            "num_eq" | "number_equal_to" => self.validate_number_equal_to(),
            "num_gt" | "number_greater_than" => self.validate_number_greater_than(),
            "num_gte" | "number_greater_than_or_equal_to" => {
                self.validate_number_greater_than_or_equal_to()
            }
            "num_lt" | "number_less_than" => self.validate_number_less_than(),
            "num_lte" | "number_less_than_or_equal_to" => {
                self.validate_number_less_than_or_equal_to()
            }
            "num_nbtwn" | "number_not_between" => self.validate_number_not_between(),
            "num_neq" | "number_not_equal_to" => self.validate_number_not_equal_to(),
            "text_contains" => self.validate_text_contains(),
            "text_does_not_contain" => self.validate_text_does_not_contain(),
            "text_eq" | "text_equal_to" => self.validate_text_equal_to(),
            "is_email" | "text_is_valid_email" => self.validate_text_is_valid_email(),
            "is_url" | "text_is_valid_url" => self.validate_text_is_valid_url(),
            // TODO:
            // "in_list" | "value_in_list" => self.validate_value_in_list(),
            "in_list" | "value_in_list" => todo!(),
            "in_range" | "value_in_range" => self.validate_value_in_range(),
            _ => Err(ModifierParseError::new(
                "validate",
                name,
                &[
                    "custom(FORMULA)",
                    "date_after(DATE)",
                    "date_before(DATE)",
                    "date_between(DATE, DATE)",
                    "date_equal_to(DATE)",
                    "date_is_valid",
                    "date_not_between(DATE, DATE)",
                    "date_on_or_after(DATE)",
                    "date_on_or_before(DATE)",
                    "number_between(NUMBER, NUMBER)",
                    "number_equal_to(NUMBER)",
                    "number_greater_than_or_equal_to(NUMBER)",
                    "number_greater_than(NUMBER)",
                    "number_less_than_or_equal_to(NUMBER)",
                    "number_less_than(NUMBER)",
                    "number_not_between(NUMBER, NUMBER)",
                    "number_not_equal_to(NUMBER)",
                    "text_contains(TEXT)",
                    "text_does_not_contain(TEXT)",
                    "text_equal_to(TEXT)",
                    "text_is_valid_email",
                    "text_is_valid_url",
                    "value_in_list(ANY, ...)",
                    "value_in_range(A1)",
                ],
            )
            .into()),
        }
    }

    validate! {self, validate_custom, {
        DataValidation::Custom(self.one_string_in_parens()?)
    }}

    validate! {self, validate_date_after, {
        DataValidation::DateAfter(self.one_date_in_parens()?)
    }}

    validate! {self, validate_date_before, {
        DataValidation::DateBefore(self.one_date_in_parens()?)
    }}

    validate! {self, validate_date_between, {
        let (a, b) = self.two_dates_in_parens()?;
        DataValidation::DateBetween(a, b)
    }}

    validate! {self, validate_date_equal_to, {
        DataValidation::DateEqualTo(self.one_date_in_parens()?)
    }}

    validate! {self, validate_date_is_valid, {
        DataValidation::DateIsValid
    }}

    validate! {self, validate_date_not_between, {
        let (a, b) = self.two_dates_in_parens()?;
        DataValidation::DateNotBetween(a, b)
    }}

    validate! {self, validate_date_on_or_after, {
        DataValidation::DateOnOrAfter(self.one_date_in_parens()?)
    }}

    validate! {self, validate_date_on_or_before, {
        DataValidation::DateOnOrBefore(self.one_date_in_parens()?)
    }}

    validate! {self, validate_number_between, {
        let (a, b) = self.two_numbers_in_parens()?;
        DataValidation::NumberBetween(a, b)
    }}

    validate! {self, validate_number_equal_to, {
        DataValidation::NumberEqualTo(self.one_number_in_parens()?)
    }}

    validate! {self, validate_number_greater_than, {
        DataValidation::NumberGreaterThan(self.one_number_in_parens()?)
    }}

    validate! {self, validate_number_greater_than_or_equal_to, {
        DataValidation::NumberGreaterThanOrEqualTo(self.one_number_in_parens()?)
    }}

    validate! {self, validate_number_not_between, {
        let (a, b) = self.two_numbers_in_parens()?;
        DataValidation::NumberNotBetween(a, b)
    }}

    validate! {self, validate_number_not_equal_to, {
        DataValidation::NumberNotEqualTo(self.one_number_in_parens()?)
    }}

    validate! {self, validate_number_less_than, {
        DataValidation::NumberLessThan(self.one_number_in_parens()?)
    }}

    validate! {self, validate_number_less_than_or_equal_to, {
        DataValidation::NumberLessThanOrEqualTo(self.one_number_in_parens()?)
    }}

    validate! {self, validate_text_contains, {
        DataValidation::TextContains(self.one_string_in_parens()?)
    }}

    validate! {self, validate_text_does_not_contain, {
        DataValidation::TextDoesNotContain(self.one_string_in_parens()?)
    }}

    validate! {self, validate_text_equal_to, {
        DataValidation::TextEqualTo(self.one_string_in_parens()?)
    }}

    validate! {self, validate_text_is_valid_email, {
        DataValidation::TextIsValidEmail
    }}

    validate! {self, validate_text_is_valid_url, {
        DataValidation::TextIsValidUrl
    }}

    /*
    validate! {self, validate_value_in_list, {
        take_parens!(self, {
            todo!()
        })
    }}
    */

    validate!(self, validate_value_in_range, {
        take_parens!(self, {
            let a1_match = self.lexer.take_token(Token::A1)?;
            DataValidation::ValueInRange(a1_notation::new(&a1_match.str_match).map_err(|e| {
                a1_match.into_parse_error(&format!("Expected an A1 reference: {e}"))
            })?)
        })
    });

    fn one_date_in_parens(&mut self) -> ParseResult<DateTime> {
        take_parens!(self, { Ok(validate_args!(self, DateTime, Token::Date)) })
    }

    fn two_dates_in_parens(&mut self) -> ParseResult<(DateTime, DateTime)> {
        take_parens!(self, {
            let match_one = self.lexer.take_token(Token::Date)?;
            let date_one = DateTime::try_from(match_one)?;

            self.lexer.take_token(Token::Comma)?;

            let match_two = self.lexer.take_token(Token::Date)?;
            let date_two = DateTime::try_from(match_two)?;

            Ok::<(DateTime, DateTime), ParseError>((date_one, date_two))
        })
    }

    fn one_number_in_parens(&mut self) -> ParseResult<isize> {
        take_parens!(self, {
            self.lexer.take_token(Token::Number)?.into_number()
        })
    }

    fn two_numbers_in_parens(&mut self) -> ParseResult<(isize, isize)> {
        take_parens!(self, {
            let a = self.lexer.take_token(Token::Number)?.into_number()?;
            self.lexer.take_token(Token::Comma)?;
            let b = self.lexer.take_token(Token::Number)?.into_number()?;
            Ok::<(isize, isize), ParseError>((a, b))
        })
    }

    fn one_string_in_parens(&mut self) -> ParseResult<String> {
        self.lexer.take_token(Token::OpenParenthesis)?;
        let string = self.lexer.take_token(Token::String)?.str_match.to_string();
        self.lexer.take_token(Token::CloseParenthesis)?;
        Ok(string)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::test_utils::*;

    fn test_parse(input: &str) -> ParsedCell {
        ModifierParser::parse(
            input,
            Address::new(0, 0),
            &RowModifier::default(),
            std::sync::Arc::new(build_source_code()),
        )
        .unwrap()
    }

    #[test]
    fn parse_data_validate_custom() {
        let parsed_modifiers = test_parse("[[validate=custom('foo')]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::Custom("foo".to_string())
        );
    }

    #[test]
    fn parse_data_validate_date_after() {
        let parsed_modifiers = test_parse("[[validate=date_after(11/22/2024)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::DateAfter(build_date_time_ymd(2024, 11, 22))
        );
    }

    #[test]
    fn parse_data_validate_date_before() {
        let parsed_modifiers = test_parse("[[validate=date_before(12/1/23)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::DateBefore(build_date_time_ymd(2023, 12, 1))
        );
    }

    #[test]
    fn parse_data_validate_date_between() {
        let parsed_modifiers = test_parse("[[validate=date_between(1/2/23, 4/5/26)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::DateBetween(
                build_date_time_ymd(2023, 1, 2),
                build_date_time_ymd(2026, 4, 5)
            )
        );
    }

    #[test]
    fn parse_data_validate_date_equal_to() {
        let parsed_modifiers = test_parse("[[validate=date_equal_to(12/1/23)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::DateEqualTo(build_date_time_ymd(2023, 12, 1))
        );
    }

    #[test]
    fn parse_data_validate_date_is_valid() {
        let parsed_modifiers = test_parse("[[validate=date_is_valid]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::DateIsValid
        );
    }

    #[test]
    fn parse_data_validate_date_not_between() {
        let parsed_modifiers = test_parse("[[validate=date_not_between(1/2/23, 4/5/26)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::DateNotBetween(
                build_date_time_ymd(2023, 1, 2),
                build_date_time_ymd(2026, 4, 5)
            )
        );
    }

    #[test]
    fn parse_data_validate_date_on_or_after() {
        let parsed_modifiers = test_parse("[[validate=date_on_or_after(11/22/2024)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::DateOnOrAfter(build_date_time_ymd(2024, 11, 22))
        );
    }

    #[test]
    fn parse_data_validate_date_on_or_before() {
        let parsed_modifiers = test_parse("[[validate=date_on_or_before(12/1/23)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::DateOnOrBefore(build_date_time_ymd(2023, 12, 1))
        );
    }

    #[test]
    fn parse_data_validate_invalid() {
        let res = ModifierParser::parse(
            "[[validate=foo_bar(12/1/23)]]abc123",
            Address::new(0, 0),
            &RowModifier::default(),
            std::sync::Arc::new(build_source_code()),
        );
        assert!(res.is_err());
    }

    #[test]
    fn parse_data_validate_number_between() {
        let parsed_modifiers = test_parse("[[validate=number_between(123, 456)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::NumberBetween(123, 456)
        );
    }

    #[test]
    fn parse_data_validate_number_equal_to() {
        let parsed_modifiers = test_parse("[[validate=number_equal_to(123)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::NumberEqualTo(123)
        );
    }

    #[test]
    fn parse_data_validate_number_greater_than() {
        let parsed_modifiers = test_parse("[[validate=number_greater_than(123)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::NumberGreaterThan(123)
        );
    }

    #[test]
    fn parse_data_validate_number_greater_than_or_equal_to() {
        let parsed_modifiers =
            test_parse("[[validate=number_greater_than_or_equal_to(123)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::NumberGreaterThanOrEqualTo(123)
        );
    }

    #[test]
    fn parse_data_validate_number_less_than() {
        let parsed_modifiers = test_parse("[[validate=number_less_than(123)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::NumberLessThan(123)
        );
    }

    #[test]
    fn parse_data_validate_number_less_than_or_equal_to() {
        let parsed_modifiers = test_parse("[[validate=number_less_than_or_equal_to(123)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::NumberLessThanOrEqualTo(123)
        );
    }

    #[test]
    fn parse_data_validate_number_not_between() {
        let parsed_modifiers = test_parse("[[validate=number_not_between(123, 456)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::NumberNotBetween(123, 456)
        );
    }

    #[test]
    fn parse_data_validate_number_not_equal_to() {
        let parsed_modifiers = test_parse("[[validate=number_not_equal_to(123)]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::NumberNotEqualTo(123)
        );
    }

    #[test]
    fn parse_data_validate_text_contains() {
        let parsed_modifiers = test_parse("[[validate=text_contains('foo')]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::TextContains("foo".to_string())
        );
    }

    #[test]
    fn parse_data_validate_text_does_not_contain() {
        let parsed_modifiers = test_parse("[[validate=text_does_not_contain('foo')]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::TextDoesNotContain("foo".to_string())
        );
    }

    #[test]
    fn parse_data_validate_text_equal_to() {
        let parsed_modifiers = test_parse("[[validate=text_equal_to('foo')]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::TextEqualTo("foo".to_string())
        );
    }

    #[test]
    fn parse_data_validate_text_is_valid_email() {
        let parsed_modifiers = test_parse("[[validate=text_is_valid_email]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::TextIsValidEmail
        );
    }

    #[test]
    fn parse_data_validate_text_is_valid_url() {
        let parsed_modifiers = test_parse("[[validate=text_is_valid_url]]abc123");
        assert_eq!(
            parsed_modifiers.modifier.unwrap().data_validation.unwrap(),
            DataValidation::TextIsValidUrl,
        );
    }
}
