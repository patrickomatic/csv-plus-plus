use super::{CellParser, Token};
use crate::ast::Ast;
use crate::error::{BadInput, CellParseError, ParseResult};
use crate::{DataValidation, DateTime};

macro_rules! take_parens {
    ($self:ident, $tt:tt) => {{
        $self.lexer.take_token(Token::OpenParenthesis)?;
        let _res = $tt;
        $self.lexer.take_token(Token::CloseParenthesis)?;
        _res
    }};
}

macro_rules! validate {
    ($self:ident, $variant:ident, $tt:tt) => {
        fn $variant(&mut $self) -> ParseResult<()> {
            if $self.is_row_options {
                $self.row.data_validation = Some($tt);
            } else {
                $self.cell.data_validation = Some($tt);
            }

            Ok(())
        }
    };
}

impl CellParser<'_, '_> {
    pub(super) fn validate(&mut self) -> ParseResult<()> {
        let name = self.lexer.take_option_right_side()?;

        match name.str_match.to_lowercase().as_str() {
            "c" | "custom" => self.validate_custom(),
            "date_gt" | "date_after" => self.validate_date_after(),
            "date_lt" | "date_before" => self.validate_date_before(),
            "date_btwn" | "date_between" => self.validate_date_between(),
            "date_eq" | "date_equal_to" => self.validate_date_equal_to(),
            "in_list" => self.validate_value_in_list(),
            "in_range" => self.validate_value_in_range(),
            "is_date" | "date_is_valid" => self.validate_date_is_valid(),
            "is_email" | "is_valid_email" => self.validate_text_is_valid_email(),
            "is_url" | "is_valid_url" => self.validate_text_is_valid_url(),
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
            _ => Err(CellParseError::new(
                "validate",
                name,
                &[
                    "custom(FORMULA)",
                    "date_after(DATE)",
                    "date_before(DATE)",
                    "date_between(DATE DATE)",
                    "date_equal_to(DATE)",
                    "date_is_valid",
                    "date_not_between(DATE DATE)",
                    "date_on_or_after(DATE)",
                    "date_on_or_before(DATE)",
                    "number_between(NUMBER NUMBER)",
                    "number_equal_to(NUMBER)",
                    "number_greater_than_or_equal_to(NUMBER)",
                    "number_greater_than(NUMBER)",
                    "number_less_than_or_equal_to(NUMBER)",
                    "number_less_than(NUMBER)",
                    "number_not_between(NUMBER NUMBER)",
                    "number_not_equal_to(NUMBER)",
                    "text_contains(TEXT)",
                    "text_does_not_contain(TEXT)",
                    "text_equal_to(TEXT)",
                    "text_is_valid_email",
                    "text_is_valid_url",
                    "value_in_list(ANY ...)",
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

    validate! {self, validate_value_in_list, {
        take_parens!(self, {
            let mut values: Vec<Ast> = vec![];
            loop {
                // There are only a handful of tokens we accept here and the order we parse them matters:
                //
                // * Date
                // * Number (whole & float)
                // * Single-quoted string
                // * Identifier (as a catch-all if single-quoted string doesn't match)
                //
                // If none of these are seen but it's not a closing-paren either then it's a syntax error
                //
                let m = self.lexer.maybe_take_date()
                    .or_else(|| self.lexer.maybe_take_number())
                    .or_else(|| self.lexer.maybe_take_single_quoted_string().unwrap_or(None))
                    .or_else(|| self.lexer.maybe_take_identifier());

                if let Some(tm) = m {
                    values.push(Ast::try_from(tm)?);
                } else {
                    return Err(self.lexer.unknown_string("Expected a date, number or string"));
                }

                if self.lexer.peek_close_parenthesis() {
                    break
                }
            }

            DataValidation::ValueInList(values)
        })
    }}

    validate!(self, validate_value_in_range, {
        take_parens!(self, {
            let a1_match = self.lexer.take_token(Token::A1)?;
            DataValidation::ValueInRange(
                a1_notation::new(&a1_match.str_match).map_err(|e| {
                    a1_match.into_parse_error(format!("Expected an A1 reference: {e}"))
                })?,
            )
        })
    });

    fn one_date_in_parens(&mut self) -> ParseResult<DateTime> {
        take_parens!(self, {
            DateTime::try_from(self.lexer.take_token(Token::Date)?)
        })
    }

    fn two_dates_in_parens(&mut self) -> ParseResult<(DateTime, DateTime)> {
        take_parens!(self, {
            Ok((
                DateTime::try_from(self.lexer.take_token(Token::Date)?)?,
                DateTime::try_from(self.lexer.take_token(Token::Date)?)?,
            ))
        })
    }

    fn one_number_in_parens(&mut self) -> ParseResult<i64> {
        take_parens!(self, {
            self.lexer.take_token(Token::Number)?.into_number()
        })
    }

    fn two_numbers_in_parens(&mut self) -> ParseResult<(i64, i64)> {
        take_parens!(self, {
            Ok((
                self.lexer.take_token(Token::Number)?.into_number()?,
                self.lexer.take_token(Token::Number)?.into_number()?,
            ))
        })
    }

    fn one_string_in_parens(&mut self) -> ParseResult<String> {
        take_parens!(self, {
            Ok(self.lexer.take_token(Token::String)?.str_match.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::test_utils::*;
    use crate::DateTime;
    use crate::*;

    fn test_parse(input: &str) -> Cell {
        let mut row = Row::default();
        CellParser::parse(
            input,
            a1_notation::Address::new(0, 0),
            &mut row,
            build_source_code(),
        )
        .unwrap()
    }

    #[test]
    fn parse_validate_custom() {
        let cell = test_parse("[[validate=custom('foo')]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::Custom("foo".to_string())
        );
    }

    #[test]
    fn parse_validate_date_after() {
        let cell = test_parse("[[validate=date_after(11/22/2024)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::DateAfter(build_date_time_ymd(2024, 11, 22))
        );
    }

    #[test]
    fn parse_validate_date_before() {
        let cell = test_parse("[[validate=date_before(12/1/23)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::DateBefore(build_date_time_ymd(2023, 12, 1))
        );
    }

    #[test]
    fn parse_validate_date_between() {
        let cell = test_parse("[[validate=date_between(2/4/2025 10/20/2026)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::DateBetween(
                build_date_time_ymd(2025, 2, 4),
                build_date_time_ymd(2026, 10, 20)
            )
        );
    }

    #[test]
    fn parse_validate_date_equal_to() {
        let cell = test_parse("[[validate=date_equal_to(2023-12-01)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::DateEqualTo(build_date_time_ymd(2023, 12, 1))
        );
    }

    #[test]
    fn parse_validate_date_is_valid() {
        let cell = test_parse("[[validate=date_is_valid]]abc123");
        assert_eq!(cell.data_validation.unwrap(), DataValidation::DateIsValid);
    }

    #[test]
    fn parse_validate_date_not_between() {
        let cell = test_parse("[[validate=date_not_between(1/2/23 4/5/26)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::DateNotBetween(
                build_date_time_ymd(2023, 1, 2),
                build_date_time_ymd(2026, 4, 5)
            )
        );
    }

    #[test]
    fn parse_validate_date_on_or_after() {
        let cell = test_parse("[[validate=date_on_or_after(11/22/2024)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::DateOnOrAfter(build_date_time_ymd(2024, 11, 22))
        );
    }

    #[test]
    fn parse_validate_date_on_or_before() {
        let cell = test_parse("[[validate=date_on_or_before(12/1/23)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::DateOnOrBefore(build_date_time_ymd(2023, 12, 1))
        );
    }

    #[test]
    fn parse_validate_invalid() {
        let res = CellParser::parse(
            "[[validate=foo_bar(12/1/23)]]abc123",
            a1_notation::Address::new(0, 0),
            &mut Row::default(),
            build_source_code(),
        );
        assert!(res.is_err());
    }

    #[test]
    fn parse_validate_in_range() {
        let cell = test_parse("[[validate=in_range(A:A)]]abc123");

        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::ValueInRange(a1_notation::column(0))
        );
    }

    #[test]
    fn parse_validate_in_list() {
        let cell = test_parse("[[validate=in_list('foo' bar 123 11/22/2024)]]abc123");

        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::ValueInList(vec![
                Ast::new(Node::Text("foo".to_string())),
                Ast::new(Node::Reference("bar".to_string())),
                Ast::new(Node::Integer {
                    percentage: false,
                    sign: None,
                    value: 123,
                }),
                Ast::new(Node::DateTime(DateTime::Date(
                    chrono::NaiveDate::from_ymd_opt(2024, 11, 22).unwrap()
                ))),
            ])
        );
    }

    #[test]
    fn parse_validate_is_valid_email() {
        let cell = test_parse("[[validate=is_valid_email]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::TextIsValidEmail
        );
    }

    #[test]
    fn parse_validate_is_valid_url() {
        let cell = test_parse("[[validate=is_valid_url]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::TextIsValidUrl,
        );
    }

    #[test]
    fn parse_validate_number_between() {
        let cell = test_parse("[[validate=number_between(123 456)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::NumberBetween(123, 456)
        );
    }

    #[test]
    fn parse_validate_number_equal_to() {
        let cell = test_parse("[[validate=number_equal_to(123)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::NumberEqualTo(123)
        );
    }

    #[test]
    fn parse_validate_number_greater_than() {
        let cell = test_parse("[[validate=number_greater_than(123)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::NumberGreaterThan(123)
        );
    }

    #[test]
    fn parse_validate_number_greater_than_or_equal_to() {
        let cell = test_parse("[[validate=number_greater_than_or_equal_to(123)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::NumberGreaterThanOrEqualTo(123)
        );
    }

    #[test]
    fn parse_validate_number_less_than() {
        let cell = test_parse("[[validate=number_less_than(123)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::NumberLessThan(123)
        );
    }

    #[test]
    fn parse_validate_number_less_than_or_equal_to() {
        let cell = test_parse("[[validate=number_less_than_or_equal_to(123)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::NumberLessThanOrEqualTo(123)
        );
    }

    #[test]
    fn parse_validate_number_not_between() {
        let cell = test_parse("[[validate=number_not_between(123 456)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::NumberNotBetween(123, 456)
        );
    }

    #[test]
    fn parse_validate_number_not_equal_to() {
        let cell = test_parse("[[validate=number_not_equal_to(123)]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::NumberNotEqualTo(123)
        );
    }

    #[test]
    fn parse_validate_text_contains() {
        let cell = test_parse("[[validate=text_contains('foo')]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::TextContains("foo".to_string())
        );
    }

    #[test]
    fn parse_validate_text_does_not_contain() {
        let cell = test_parse("[[validate=text_does_not_contain('foo')]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::TextDoesNotContain("foo".to_string())
        );
    }

    #[test]
    fn parse_validate_text_equal_to() {
        let cell = test_parse("[[validate=text_equal_to('foo')]]abc123");
        assert_eq!(
            cell.data_validation.unwrap(),
            DataValidation::TextEqualTo("foo".to_string())
        );
    }
}
