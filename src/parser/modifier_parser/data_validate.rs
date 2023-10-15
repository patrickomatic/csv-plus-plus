use super::{DataValidation, ModifierParser, Token};
use crate::error::{ModifierParseError, ParseResult};
use crate::DateAndTime;

impl ModifierParser<'_, '_> {
    pub(super) fn data_validate(&mut self) -> ParseResult<()> {
        self.lexer.take_token(Token::Equals)?;

        let dv_name = self.lexer.take_modifier_right_side()?;
        match dv_name.str_match.to_lowercase().as_str() {
            "custom" => self.data_validate_custom()?,
            "date_after" => self.data_validate_date_after()?,
            "date_before" => self.data_validate_date_before()?,
            "date_between" => self.data_validate_date_between()?,
            "date_equal_to" => self.data_validate_date_equal_to()?,
            "date_is_valid" => self.data_validate_date_is_valid()?,
            "date_not_between" => self.data_validate_date_not_between()?,
            "date_on_or_after" => self.data_validate_date_on_or_after()?,
            "date_on_or_before" => self.data_validate_date_on_or_before()?,
            "number_between" => self.data_validate_number_between()?,
            "number_equal_to" => self.data_validate_number_equal_to()?,
            "number_greater_than_or_equal_to" => {
                self.data_validate_number_greater_than_or_equal_to()?
            }
            "number_greater_than" => self.data_validate_number_greater_than()?,
            "number_less_than_or_equal_to" => self.data_validate_number_less_than_or_equal_to()?,
            "number_less_than" => self.data_validate_number_less_than()?,
            "number_not_between" => self.data_validate_number_not_between()?,
            "number_not_equal_to" => self.data_validate_number_not_equal_to()?,
            "text_contains" => self.data_validate_text_contains()?,
            "text_does_not_contain" => self.data_validate_text_does_not_contain()?,
            "text_equal_to" => self.data_validate_text_equal_to()?,
            "text_is_valid_email" => self.data_validate_text_is_valid_email()?,
            "text_is_valid_url" => self.data_validate_text_is_valid_url()?,
            "value_in_list" => self.data_validate_value_in_list()?,
            "value_in_range" => self.data_validate_value_in_range()?,
            _ => {
                return Err(ModifierParseError::new(
                    "validate",
                    dv_name,
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
                .into_parse_error(&self.runtime.source_code));
            }
        }

        Ok(())
    }

    fn data_validate_custom(&mut self) -> ParseResult<()> {
        todo!();
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
            Some(DataValidation::DateOnOrAfter(self.one_date_in_parens()?));
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
        todo!();
    }

    fn data_validate_text_does_not_contain(&mut self) -> ParseResult<()> {
        todo!();
    }

    fn data_validate_text_equal_to(&mut self) -> ParseResult<()> {
        todo!();
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

    fn one_date_in_parens(&mut self) -> ParseResult<DateAndTime> {
        self.lexer.take_token(Token::OpenParenthesis)?;
        let date_time = self
            .lexer
            .take_token(Token::Date)?
            .into_date_and_time("%M/%d/%Y", &self.runtime.source_code)?;
        self.lexer.take_token(Token::CloseParenthesis)?;

        Ok(date_time)
    }

    fn two_numbers_in_parens(&mut self) -> ParseResult<(isize, isize)> {
        self.lexer.take_token(Token::OpenParenthesis)?;

        let a = self
            .lexer
            .take_token(Token::Number)?
            .into_number(&self.runtime.source_code)?;

        self.lexer.take_token(Token::Comma)?;

        let b = self
            .lexer
            .take_token(Token::Number)?
            .into_number(&self.runtime.source_code)?;

        self.lexer.take_token(Token::CloseParenthesis)?;

        Ok((a, b))
    }

    fn one_number_in_parens(&mut self) -> ParseResult<isize> {
        self.lexer.take_token(Token::OpenParenthesis)?;
        let number = self
            .lexer
            .take_token(Token::Number)?
            .into_number(&self.runtime.source_code)?;
        self.lexer.take_token(Token::CloseParenthesis)?;

        Ok(number)
    }

    fn two_dates_in_parens(&mut self) -> ParseResult<(DateAndTime, DateAndTime)> {
        self.lexer.take_token(Token::OpenParenthesis)?;

        let date_one = self
            .lexer
            .take_token(Token::Date)?
            .into_date_and_time("%M/%d/%Y", &self.runtime.source_code)?;

        self.lexer.take_token(Token::Comma)?;

        let date_two = self
            .lexer
            .take_token(Token::Date)?
            .into_date_and_time("%M/%d/%Y", &self.runtime.source_code)?;

        self.lexer.take_token(Token::CloseParenthesis)?;

        Ok((date_one, date_two))
    }
}
