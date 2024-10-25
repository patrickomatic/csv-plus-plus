//! # `TokenMatch`
//!
use super::Token;
use crate::ast::{Ast, Node};
use crate::error::{BadInput, ParseError, ParseResult};
use crate::parser::TokenInput;
use crate::{compiler_error, ArcSourceCode, CharOffset, DateTime, LineNumber};
use std::fmt;

#[derive(Clone, Debug)]
pub(crate) struct TokenMatch {
    pub(crate) token: Token,
    pub(crate) str_match: String,
    pub(crate) position: a1::Address,
    pub(crate) cell_offset: CharOffset,
    pub(crate) source_code: ArcSourceCode,
}

impl TokenMatch {
    // TODO: make an actual Into impl
    pub(crate) fn into_number(self) -> ParseResult<i64> {
        self.str_match
            .parse::<i64>()
            .map_err(|e| self.into_parse_error(format!("Unable to parse integer: {e}")))
    }

    pub(crate) fn into_float(self) -> ParseResult<f64> {
        self.str_match
            .parse::<f64>()
            .map_err(|e| self.into_parse_error(format!("Unable to parse float: {e}")))
    }
}

impl fmt::Display for TokenMatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.str_match)
    }
}

impl BadInput for TokenMatch {
    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
        self.source_code.parse_error(&self, message)
    }

    fn line_number(&self) -> LineNumber {
        self.source_code.csv_line_number(self.position)
    }

    fn line_offset(&self) -> CharOffset {
        self.source_code.line_offset_for_cell(self.position, false) + self.cell_offset
    }
}

impl TokenInput for TokenMatch {
    fn input(&self) -> &str {
        self.str_match.as_str()
    }
}

/// A rough conversion from something we saw in a cell definition to an AST terminal value.  No
/// functions allowed (maybe that will change but we get into variable resolution and complex
/// parsing)
impl TryFrom<TokenMatch> for Ast {
    type Error = ParseError;

    fn try_from(tm: TokenMatch) -> ParseResult<Self> {
        Ok(Ast::new(match tm.token {
            Token::Date => Node::DateTime(DateTime::try_from(tm)?),
            Token::A1 | Token::Identifier => Node::Reference(tm.str_match),
            Token::String => Node::parse_text(tm)?,
            Token::Number | Token::PositiveNumber => {
                if tm.str_match.contains('.') {
                    tm.into_float()?.into()
                } else {
                    tm.into_number()?.into()
                }
            }
            _ => compiler_error(format!(
                "Unsupported token for converting to an AST: {}",
                tm.token
            )),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn build_token_match(token: Token, str_match: &str) -> TokenMatch {
        TokenMatch {
            token,
            str_match: str_match.to_string(),
            position: a1::Address::new(0, 0),
            cell_offset: 0,
            source_code: build_source_code(),
        }
    }

    #[test]
    fn into_float() {
        assert!(
            (build_token_match(Token::Number, "1.23")
                .into_float()
                .unwrap()
                - 1.23)
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn into_number() {
        assert_eq!(
            build_token_match(Token::Number, "123")
                .into_number()
                .unwrap(),
            123
        );
    }

    #[test]
    fn try_from_a1() {
        assert_eq!(
            Ast::try_from(build_token_match(Token::A1, "B3")).unwrap(),
            Ast::new(Node::Reference("B3".to_string()))
        );
    }

    #[test]
    fn try_from_date() {
        assert_eq!(
            Ast::try_from(build_token_match(Token::Date, "11/22/2023")).unwrap(),
            Ast::new(Node::DateTime(DateTime::Date(
                chrono::NaiveDate::from_ymd_opt(2023, 11, 22).unwrap()
            ))),
        );
    }

    #[test]
    fn try_from_identifier() {
        assert_eq!(
            Ast::try_from(build_token_match(Token::Identifier, "foo")).unwrap(),
            Ast::new(Node::Reference("foo".to_string()))
        );
    }

    #[test]
    fn try_from_string() {
        assert_eq!(
            Ast::try_from(build_token_match(Token::String, "foo")).unwrap(),
            Ast::new(Node::Text("foo".to_string()))
        );
    }

    #[test]
    fn try_from_number() {
        assert_eq!(
            Ast::try_from(build_token_match(Token::Number, "-123")).unwrap(),
            Ast::new((-123).into())
        );
    }

    #[test]
    fn try_from_positive_number() {
        assert_eq!(
            Ast::try_from(build_token_match(Token::Number, "123")).unwrap(),
            Ast::new(123.into())
        );
    }
}
