use super::Token;
use crate::ast::{Ast, Node};
use crate::error::{BadInput, Error, ParseError, ParseResult};
use crate::parser::TokenInput;
use crate::{CharOffset, DateTime, LineNumber, SourceCode};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub(crate) struct TokenMatch<'a> {
    pub(crate) token: Token,
    pub(crate) str_match: &'a str,
    pub(crate) line_number: LineNumber,
    pub(crate) line_offset: CharOffset,
    pub(crate) source_code: &'a SourceCode,
}

impl TokenMatch<'_> {
    pub(crate) fn into_error<S: Into<String>>(self, message: S) -> Error {
        self.source_code
            .code_syntax_error(self.into_parse_error(message))
    }
}

impl BadInput for TokenMatch<'_> {
    fn line_number(&self) -> LineNumber {
        self.line_number
    }

    fn line_offset(&self) -> CharOffset {
        self.line_offset
    }

    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
        self.source_code.parse_error(self, message)
    }
}

impl TokenInput for TokenMatch<'_> {
    fn input(&self) -> &str {
        self.str_match
    }
}

impl fmt::Display for TokenMatch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.str_match)
    }
}

impl TryFrom<TokenMatch<'_>> for Ast {
    type Error = ParseError;

    fn try_from(tm: TokenMatch) -> ParseResult<Self> {
        match tm.token {
            Token::Boolean => {
                let input_lower = tm.str_match.to_lowercase();
                if input_lower == "true" {
                    Ok(Box::new(true.into()))
                } else if input_lower == "false" {
                    Ok(Box::new(false.into()))
                } else {
                    Err(tm.into_parse_error(
                        "Error parsing boolean value: expected `true` or `false`",
                    ))
                }
            }

            Token::DateTime => Ok(Box::new(Node::DateTime(DateTime::try_from(tm)?))),

            Token::Float => Ok(Box::new(
                tm.str_match
                    .parse::<f64>()
                    .map_err(|e| tm.into_parse_error(format!("Error parsing float value: {e}")))?
                    .into(),
            )),

            Token::Integer => Ok(Box::new(
                tm.str_match
                    .parse::<i64>()
                    .map_err(|e| tm.into_parse_error(format!("Error parsing integer value: {e}")))?
                    .into(),
            )),

            Token::Reference => Ok(Box::new(Node::reference(tm.str_match))),

            Token::DoubleQuotedString => Ok(Box::new(Node::text(tm.str_match))),

            // TODO: create a new error type for these kinds of things... Error::InternalError
            _ => {
                Err(tm.into_parse_error(format!("Unable to convert non-terminal token: {:?}", tm)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn build_token_match<'a>(
        token: Token,
        str_match: &'a str,
        source_code: &'a SourceCode,
    ) -> TokenMatch<'a> {
        let mut tm = build_ast_token_match(str_match, source_code);
        tm.token = token;
        tm
    }

    #[test]
    fn try_from_boolean() {
        let source_code = build_source_code();

        assert_eq!(
            Node::Boolean(false),
            *(Ast::try_from(build_token_match(Token::Boolean, "false", &source_code)).unwrap())
        );
        assert_eq!(
            Node::Boolean(false),
            *(Ast::try_from(build_token_match(Token::Boolean, "FALSE", &source_code)).unwrap())
        );

        assert_eq!(
            Node::Boolean(true),
            *(Ast::try_from(build_token_match(Token::Boolean, "true", &source_code)).unwrap())
        );
        assert_eq!(
            Node::Boolean(true),
            *(Ast::try_from(build_token_match(Token::Boolean, "TRUE", &source_code)).unwrap())
        );
    }

    #[test]
    fn try_from_invalid() {
        let source_code = build_source_code();
        assert!(Ast::try_from(build_token_match(Token::Comma, "bar", &source_code)).is_err());
    }

    #[test]
    fn try_from_datetime() {
        let source_code = build_source_code();
        let date = build_date_time_ymd(2022, 10, 12);

        assert_eq!(
            Node::DateTime(date),
            *(Ast::try_from(build_token_match(
                Token::DateTime,
                "2022-10-12",
                &source_code
            ))
            .unwrap())
        );
    }

    #[test]
    fn try_from_float() {
        let source_code = build_source_code();
        assert_eq!(
            Node::Float(123.45),
            *(Ast::try_from(build_token_match(Token::Float, "123.45", &source_code)).unwrap())
        );
    }

    #[test]
    fn try_from_integer() {
        let source_code = build_source_code();
        assert_eq!(
            Node::Integer(123),
            *(Ast::try_from(build_token_match(Token::Integer, "123", &source_code)).unwrap())
        );
    }

    #[test]
    fn try_from_reference() {
        let source_code = build_source_code();
        assert_eq!(
            Node::reference("bar"),
            *(Ast::try_from(build_token_match(Token::Reference, "bar", &source_code)).unwrap())
        );
    }

    #[test]
    fn try_from_text() {
        let source_code = build_source_code();
        assert_eq!(
            Node::text("foo"),
            *(Ast::try_from(build_token_match(
                Token::DoubleQuotedString,
                "foo",
                &source_code
            ))
            .unwrap())
        );
    }
}
