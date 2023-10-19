use super::{DataValidation, ModifierParser, Token};
use crate::error::{ModifierParseError, ParseResult};
use crate::DateTime;

// TODO: keep making this more general
macro_rules! data_validate_args {
    ($self:ident, $From:ident, $tok:path $(, $toks:path)*) => {{
        $self.lexer.take_token(Token::OpenParenthesis)?;

        let _parsed = $From::from_token_input($self.lexer.take_token($tok)?)?;

        $self.lexer.take_token(Token::CloseParenthesis)?;

        _parsed
    }};
}

impl ModifierParser<'_, '_> {
    pub(super) fn validate(&mut self) -> ParseResult<()> {
        let name = self.lexer.take_modifier_right_side()?;

        match name.str_match.to_lowercase().as_str() {
            "custom" => self.data_validate_custom(),
            "date_after" => self.data_validate_date_after(),
            "date_before" => self.data_validate_date_before(),
            "date_between" => self.data_validate_date_between(),
            "date_equal_to" => self.data_validate_date_equal_to(),
            "date_is_valid" => self.data_validate_date_is_valid(),
            "date_not_between" => self.data_validate_date_not_between(),
            "date_on_or_after" => self.data_validate_date_on_or_after(),
            "date_on_or_before" => self.data_validate_date_on_or_before(),
            "number_between" => self.data_validate_number_between(),
            "number_equal_to" => self.data_validate_number_equal_to(),
            "number_greater_than" => self.data_validate_number_greater_than(),
            "number_greater_than_or_equal_to" => {
                self.data_validate_number_greater_than_or_equal_to()
            }
            "number_less_than" => self.data_validate_number_less_than(),
            "number_less_than_or_equal_to" => self.data_validate_number_less_than_or_equal_to(),
            "number_not_between" => self.data_validate_number_not_between(),
            "number_not_equal_to" => self.data_validate_number_not_equal_to(),
            "text_contains" => self.data_validate_text_contains(),
            "text_does_not_contain" => self.data_validate_text_does_not_contain(),
            "text_equal_to" => self.data_validate_text_equal_to(),
            "text_is_valid_email" => self.data_validate_text_is_valid_email(),
            "text_is_valid_url" => self.data_validate_text_is_valid_url(),
            "value_in_list" => self.data_validate_value_in_list(),
            "value_in_range" => self.data_validate_value_in_range(),
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

    fn data_validate_custom(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::Custom(self.one_string_in_parens()?));
        Ok(())
    }

    fn data_validate_date_after(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::DateAfter(self.one_date_in_parens()?));
        Ok(())
    }

    fn data_validate_date_before(&mut self) -> ParseResult<()> {
        self.modifier.data_validation =
            Some(DataValidation::DateBefore(self.one_date_in_parens()?));
        Ok(())
    }

    fn data_validate_date_between(&mut self) -> ParseResult<()> {
        let (a, b) = self.two_dates_in_parens()?;
        self.modifier.data_validation = Some(DataValidation::DateBetween(a, b));
        Ok(())
    }

    fn data_validate_date_equal_to(&mut self) -> ParseResult<()> {
        self.modifier.data_validation =
            Some(DataValidation::DateEqualTo(self.one_date_in_parens()?));
        Ok(())
    }

    fn data_validate_date_is_valid(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::DateIsValid);
        Ok(())
    }

    fn data_validate_date_not_between(&mut self) -> ParseResult<()> {
        let (a, b) = self.two_dates_in_parens()?;
        self.modifier.data_validation = Some(DataValidation::DateNotBetween(a, b));
        Ok(())
    }

    fn data_validate_date_on_or_after(&mut self) -> ParseResult<()> {
        self.modifier.data_validation =
            Some(DataValidation::DateOnOrAfter(self.one_date_in_parens()?));
        Ok(())
    }

    fn data_validate_date_on_or_before(&mut self) -> ParseResult<()> {
        self.modifier.data_validation =
            Some(DataValidation::DateOnOrBefore(self.one_date_in_parens()?));
        Ok(())
    }

    fn data_validate_number_between(&mut self) -> ParseResult<()> {
        let (a, b) = self.two_numbers_in_parens()?;
        self.modifier.data_validation = Some(DataValidation::NumberBetween(a, b));
        Ok(())
    }

    fn data_validate_number_equal_to(&mut self) -> ParseResult<()> {
        self.modifier.data_validation =
            Some(DataValidation::NumberEqualTo(self.one_number_in_parens()?));
        Ok(())
    }

    fn data_validate_number_greater_than(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::NumberGreaterThan(
            self.one_number_in_parens()?,
        ));
        Ok(())
    }

    fn data_validate_number_greater_than_or_equal_to(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::NumberGreaterThanOrEqualTo(
            self.one_number_in_parens()?,
        ));
        Ok(())
    }

    fn data_validate_number_not_between(&mut self) -> ParseResult<()> {
        let (a, b) = self.two_numbers_in_parens()?;
        self.modifier.data_validation = Some(DataValidation::NumberNotBetween(a, b));
        Ok(())
    }

    fn data_validate_number_not_equal_to(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::NumberNotEqualTo(
            self.one_number_in_parens()?,
        ));
        Ok(())
    }

    fn data_validate_number_less_than(&mut self) -> ParseResult<()> {
        self.modifier.data_validation =
            Some(DataValidation::NumberLessThan(self.one_number_in_parens()?));
        Ok(())
    }

    fn data_validate_number_less_than_or_equal_to(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::NumberLessThanOrEqualTo(
            self.one_number_in_parens()?,
        ));
        Ok(())
    }

    fn data_validate_text_contains(&mut self) -> ParseResult<()> {
        self.modifier.data_validation =
            Some(DataValidation::TextContains(self.one_string_in_parens()?));
        Ok(())
    }

    fn data_validate_text_does_not_contain(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::TextDoesNotContain(
            self.one_string_in_parens()?,
        ));
        Ok(())
    }

    fn data_validate_text_equal_to(&mut self) -> ParseResult<()> {
        self.modifier.data_validation =
            Some(DataValidation::TextEqualTo(self.one_string_in_parens()?));
        Ok(())
    }

    fn data_validate_text_is_valid_email(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::TextIsValidEmail);
        Ok(())
    }

    fn data_validate_text_is_valid_url(&mut self) -> ParseResult<()> {
        self.modifier.data_validation = Some(DataValidation::TextIsValidUrl);
        Ok(())
    }

    fn data_validate_value_in_list(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_value_in_range(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn one_date_in_parens(&mut self) -> ParseResult<DateTime> {
        /*
        self.lexer.take_token(Token::OpenParenthesis)?;

        let date_match = self.lexer.take_token(Token::Date)?;
        let date_time = DateTime::from_token_input(date_match, &self.runtime.source_code)?;

        self.lexer.take_token(Token::CloseParenthesis)?;

        Ok(date_time)

        */
        Ok(data_validate_args!(self, DateTime, Token::Date))
    }

    fn two_dates_in_parens(&mut self) -> ParseResult<(DateTime, DateTime)> {
        self.lexer.take_token(Token::OpenParenthesis)?;

        let match_one = self.lexer.take_token(Token::Date)?;
        let date_one = DateTime::from_token_input(match_one)?;

        self.lexer.take_token(Token::Comma)?;

        let match_two = self.lexer.take_token(Token::Date)?;
        let date_two = DateTime::from_token_input(match_two)?;

        self.lexer.take_token(Token::CloseParenthesis)?;

        Ok((date_one, date_two))
    }

    fn one_number_in_parens(&mut self) -> ParseResult<isize> {
        self.lexer.take_token(Token::OpenParenthesis)?;
        let number = self.lexer.take_token(Token::Number)?.into_number()?;
        self.lexer.take_token(Token::CloseParenthesis)?;

        Ok(number)
    }

    fn two_numbers_in_parens(&mut self) -> ParseResult<(isize, isize)> {
        self.lexer.take_token(Token::OpenParenthesis)?;

        let a = self.lexer.take_token(Token::Number)?.into_number()?;

        self.lexer.take_token(Token::Comma)?;

        let b = self.lexer.take_token(Token::Number)?.into_number()?;

        self.lexer.take_token(Token::CloseParenthesis)?;

        Ok((a, b))
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
