use super::Token;
use crate::ast::{Ast, Node};
use crate::error::{BadInput, Error, ParseError, ParseResult};
use crate::parser::TokenInput;
use crate::{ArcSourceCode, CharOffset, LineNumber};
use colored::Colorize;
use std::fmt;

#[derive(Clone, Debug)]
pub(crate) struct TokenMatch<'a> {
    pub(crate) token: Token,
    pub(crate) str_match: &'a str,
    pub(crate) line_number: LineNumber,
    pub(crate) line_offset: CharOffset,
    pub(crate) source_code: ArcSourceCode,
    pub(crate) position: Option<a1_notation::Address>,
}

impl TokenMatch<'_> {
    pub(crate) fn into_error<S: Into<String>>(self, message: S) -> Error {
        self.source_code
            .clone()
            .code_syntax_error(self.into_parse_error(message))
    }
}

impl BadInput for TokenMatch<'_> {
    fn line_number(&self) -> LineNumber {
        if let Some(position) = self.position {
            self.line_number + self.source_code.csv_line_number(position)
        } else {
            self.line_number
        }
    }

    fn line_offset(&self) -> CharOffset {
        if let Some(position) = self.position {
            self.line_offset + self.source_code.line_offset_for_cell(position, true)
        } else {
            self.line_offset
        }
    }

    fn into_parse_error<S: Into<String>>(self, message: S) -> ParseError {
        self.source_code.parse_error(&self, message)
    }
}

impl TokenInput for TokenMatch<'_> {
    fn input(&self) -> &str {
        self.str_match
    }
}

impl fmt::Display for TokenMatch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.token == Token::Eof {
            write!(f, "EOF")?;
            if self.position.is_some() {
                write!(
                    f,
                    "{}",
                    "\nIf your formula has a comma in it, you might need to escape it with quotes (i.e. `foo,\"=my_function(1, 2)\",bar`)".cyan())?;
            }
        } else {
            write!(f, "`{}`", self.str_match)?;
        }
        Ok(())
    }
}

impl TryFrom<TokenMatch<'_>> for Ast {
    type Error = ParseError;

    fn try_from(tm: TokenMatch) -> ParseResult<Self> {
        match tm.token {
            Token::Boolean => {
                let input_lower = tm.str_match.to_lowercase();
                if input_lower == "true" {
                    Ok(true.into())
                } else if input_lower == "false" {
                    Ok(false.into())
                } else {
                    Err(tm.into_parse_error(
                        "Error parsing boolean value: expected `true` or `false`",
                    ))
                }
            }

            Token::Float => Ok(tm
                .str_match
                .parse::<f64>()
                .map_err(|e| tm.into_parse_error(format!("Error parsing float value: {e}")))?
                .into()),

            Token::Integer => Ok(tm
                .str_match
                .parse::<i64>()
                .map_err(|e| tm.into_parse_error(format!("Error parsing integer value: {e}")))?
                .into()),

            Token::Reference => Ok(Node::reference(tm.str_match).into()),

            Token::DoubleQuotedString => Ok(Node::text(tm.str_match).into()),

            // TODO: create a new error type for these kinds of things... Error::InternalError
            _ => Err(tm
                .clone()
                .into_parse_error(format!("Unable to convert non-terminal token: {tm:?}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    fn build_token_match(token: Token, str_match: &str, source_code: ArcSourceCode) -> TokenMatch {
        let mut tm = build_ast_token_match(str_match, source_code);
        tm.token = token;
        tm
    }

    #[test]
    fn try_from_boolean() {
        let source_code = build_source_code();

        assert_eq!(
            Node::Boolean(false),
            *(Ast::try_from(build_token_match(
                Token::Boolean,
                "false",
                source_code.clone()
            ))
            .unwrap())
        );
        assert_eq!(
            Node::Boolean(false),
            *(Ast::try_from(build_token_match(
                Token::Boolean,
                "FALSE",
                source_code.clone()
            ))
            .unwrap())
        );

        assert_eq!(
            Node::Boolean(true),
            *(Ast::try_from(build_token_match(
                Token::Boolean,
                "true",
                source_code.clone()
            ))
            .unwrap())
        );
        assert_eq!(
            Node::Boolean(true),
            *(Ast::try_from(build_token_match(
                Token::Boolean,
                "TRUE",
                source_code.clone()
            ))
            .unwrap())
        );
    }

    #[test]
    fn try_from_invalid() {
        assert!(
            Ast::try_from(build_token_match(Token::Comma, "bar", build_source_code())).is_err()
        );
    }

    #[test]
    fn try_from_float() {
        assert_eq!(
            Node::Float {
                percentage: false,
                sign: None,
                value: 123.45,
            },
            *(Ast::try_from(build_token_match(
                Token::Float,
                "123.45",
                build_source_code()
            ))
            .unwrap())
        );
    }

    #[test]
    fn try_from_integer() {
        assert_eq!(
            Node::Integer {
                percentage: false,
                sign: None,
                value: 123,
            },
            *(Ast::try_from(build_token_match(
                Token::Integer,
                "123",
                build_source_code()
            ))
            .unwrap())
        );
    }

    #[test]
    fn try_from_reference() {
        assert_eq!(
            Node::reference("bar"),
            *(Ast::try_from(build_token_match(
                Token::Reference,
                "bar",
                build_source_code()
            ))
            .unwrap())
        );
    }

    #[test]
    fn try_from_text() {
        assert_eq!(
            Node::text("foo"),
            *(Ast::try_from(build_token_match(
                Token::DoubleQuotedString,
                "foo",
                build_source_code()
            ))
            .unwrap())
        );
    }
}
