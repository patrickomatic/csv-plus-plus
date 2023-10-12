use super::{Ast, Node};
use crate::parser::token_library::TokenMatch;
use crate::{ParseResult, SourceCode};

// TODO: make the acceptable formats more flexible
const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S %Z";

impl Node {
    pub(crate) fn boolean_from_token_match(
        input: TokenMatch,
        source_code: &SourceCode,
    ) -> ParseResult<Ast> {
        let input_lower = input.str_match.to_lowercase();
        if input_lower == "true" {
            Ok(Box::new(true.into()))
        } else if input_lower == "false" {
            Ok(Box::new(false.into()))
        } else {
            Err(source_code.parse_error(
                input,
                "Error parsing boolean value: expected `true` or `false`",
            ))
        }
    }

    pub(crate) fn datetime_from_token_match(
        input: TokenMatch,
        source_code: &SourceCode,
    ) -> ParseResult<Ast> {
        let parsed_date = chrono::NaiveDateTime::parse_from_str(input.str_match, DATE_FORMAT)
            .map_err(|e| source_code.parse_error(input, &format!("Unable to parse date: {e}")))?;

        Ok(Box::new(Node::DateTime(chrono::DateTime::from_utc(
            parsed_date,
            chrono::Utc,
        ))))
    }

    pub(crate) fn float_from_token_match(
        input: TokenMatch,
        source_code: &SourceCode,
    ) -> ParseResult<Ast> {
        Ok(Box::new(
            input
                .str_match
                .parse::<f64>()
                .map_err(|e| {
                    source_code.parse_error(input, &format!("Error parsing float value: {e}"))
                })?
                .into(),
        ))
    }

    pub(crate) fn integer_from_token_match(
        input: TokenMatch,
        source_code: &SourceCode,
    ) -> ParseResult<Ast> {
        Ok(Box::new(
            input
                .str_match
                .parse::<i64>()
                .map_err(|e| {
                    source_code.parse_error(input, &format!("Error parsing integer value: {e}"))
                })?
                .into(),
        ))
    }

    pub(crate) fn reference_from_token_match(
        input: TokenMatch,
        _source_code: &SourceCode,
    ) -> ParseResult<Ast> {
        Ok(Box::new(Self::reference(input.str_match)))
    }

    pub(crate) fn text_from_token_match(
        input: TokenMatch,
        _source_code: &SourceCode,
    ) -> ParseResult<Ast> {
        Ok(Box::new(Self::text(input.str_match)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn boolean_from_token_match_false() {
        assert_eq!(
            Node::Boolean(false),
            *Node::boolean_from_token_match(build_ast_token_match("false"), &build_source_code())
                .unwrap()
        );
        assert_eq!(
            Node::Boolean(false),
            *Node::boolean_from_token_match(build_ast_token_match("FALSE"), &build_source_code())
                .unwrap()
        );
    }

    #[test]
    fn boolean_from_token_match_true() {
        assert_eq!(
            Node::Boolean(true),
            *Node::boolean_from_token_match(build_ast_token_match("true"), &build_source_code())
                .unwrap()
        );
        assert_eq!(
            Node::Boolean(true),
            *Node::boolean_from_token_match(build_ast_token_match("TRUE"), &build_source_code())
                .unwrap()
        );
    }

    #[test]
    fn boolean_from_token_match_invalid() {
        assert!(
            Node::boolean_from_token_match(build_ast_token_match("foo"), &build_source_code())
                .is_err()
        );
    }

    #[test]
    fn datetime_from_token_match() {
        let date_time = chrono::DateTime::from_utc(
            chrono::NaiveDate::from_ymd_opt(2022, 10, 12)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            chrono::Utc,
        );

        assert_eq!(
            Node::DateTime(date_time),
            *Node::datetime_from_token_match(
                build_ast_token_match("2022-10-12 00:00:00 UTC"),
                &build_source_code()
            )
            .unwrap()
        );
    }

    #[test]
    fn datetime_from_token_match_invalid() {
        assert!(Node::datetime_from_token_match(
            build_ast_token_match("foo"),
            &build_source_code()
        )
        .is_err());
    }

    #[test]
    fn float_from_token_match() {
        assert_eq!(
            Node::Float(123.45),
            *Node::float_from_token_match(build_ast_token_match("123.45"), &build_source_code())
                .unwrap()
        );
    }

    #[test]
    fn float_from_token_match_invalid() {
        assert!(
            Node::float_from_token_match(build_ast_token_match("foo"), &build_source_code())
                .is_err()
        );
    }

    #[test]
    fn integer_from_token_match() {
        assert_eq!(
            Node::Integer(123),
            *Node::integer_from_token_match(build_ast_token_match("123"), &build_source_code())
                .unwrap()
        );
    }

    #[test]
    fn reference_from_token_match() {
        assert_eq!(
            Node::reference("bar"),
            *Node::reference_from_token_match(build_ast_token_match("bar"), &build_source_code())
                .unwrap()
        );
    }

    #[test]
    fn text_from_token_match() {
        assert_eq!(
            Node::text("foo"),
            *Node::text_from_token_match(build_ast_token_match("foo"), &build_source_code())
                .unwrap()
        );
    }
}
