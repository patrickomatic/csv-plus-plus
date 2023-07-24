use crate::{InnerError, InnerResult};
use super::{Ast, Node};

// TODO: make the acceptable formats more flexible
const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S %Z";

impl Node {
    pub fn boolean_from_str(input: &str) -> InnerResult<Ast> {
        let input_lower = input.to_lowercase();
        if input_lower == "true" {
            Ok(Box::new(Self::Boolean(true)))
        } else if input_lower == "false" {
            Ok(Box::new(Self::Boolean(false)))
        } else {
            Err(InnerError::bad_input(input, "Error parsing boolean value"))
        }
    }

    pub fn datetime_from_str(input: &str) -> InnerResult<Ast> {
        match chrono::NaiveDateTime::parse_from_str(input, DATE_FORMAT) {
            Ok(d) => 
                Ok(Box::new(Node::DateTime(chrono::DateTime::from_utc(d, chrono::Utc)))),
            Err(e) => 
                Err(InnerError::bad_input(input, &format!("Unable to parse date: {}", e))),
        }
    }

    pub fn float_from_str(input: &str) -> InnerResult<Ast> {
        match input.parse::<f64>() {
            Ok(i) => 
                Ok(Box::new(Self::Float(i))),
            Err(e) => 
                Err(InnerError::bad_input(input, &format!("Error parsing float value: {}", e))),
        }
    }

    pub fn integer_from_str(input: &str) -> InnerResult<Ast> {
        match input.parse::<i64>() {
            Ok(i) => 
                Ok(Box::new(Self::Integer(i))),
            Err(e) => 
                Err(InnerError::bad_input(input, &format!("Error parsing integer value: {}", e))),
        }
    }

    pub fn reference_from_str(input: &str) -> InnerResult<Ast> {
        Ok(Box::new(Self::Reference(input.to_string())))
    }

    pub fn text_from_str(input: &str) -> InnerResult<Ast> {
        Ok(Box::new(Self::Text(input.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_from_str_false() {
        assert_eq!(Node::Boolean(false), *Node::boolean_from_str("false").unwrap());
        assert_eq!(Node::Boolean(false), *Node::boolean_from_str("FALSE").unwrap());
    }

    #[test]
    fn boolean_from_str_true() {
        assert_eq!(Node::Boolean(true), *Node::boolean_from_str("true").unwrap());
        assert_eq!(Node::Boolean(true), *Node::boolean_from_str("TRUE").unwrap());
    }

    #[test]
    fn boolean_from_str_invalid() {
        assert!(Node::boolean_from_str("foo").is_err());
    }

    #[test]
    fn datetime_from_str() {
        let date_time = chrono::DateTime::from_utc(
            chrono::NaiveDate::from_ymd_opt(2022, 10, 12).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            chrono::Utc,
        );

        assert_eq!(Node::DateTime(date_time), *Node::datetime_from_str("2022-10-12 00:00:00 UTC").unwrap());
    }

    #[test]
    fn datetime_from_str_invalid() {
        assert!(Node::datetime_from_str("foo").is_err());
    }

    #[test]
    fn float_from_str() {
        assert_eq!(Node::Float(123.45), *Node::float_from_str("123.45").unwrap());
    }

    #[test]
    fn float_from_str_invalid() {
        assert!(Node::float_from_str("foo").is_err());
    }

    #[test]
    fn integer_from_str() {
        assert_eq!(Node::Integer(123), *Node::integer_from_str("123").unwrap());
    }

    #[test]
    fn reference_from_str() {
        assert_eq!(Node::Reference("bar".to_owned()), *Node::reference_from_str("bar").unwrap());
    }

    #[test]
    fn text_from_str() {
        assert_eq!(Node::Text("foo".to_owned()), *Node::text_from_str("foo").unwrap());
    }
}
