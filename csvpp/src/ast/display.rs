use std::fmt;
use super::Node;

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(b) =>
                write!(f, "{}", if *b { "TRUE" } else { "FALSE" }),

            Self::DateTime(d) => write!(f, "{}", d),

            Self::Float(fl) => write!(f, "{}", fl),

            Self::Function { args, body, name } => 
                write!(f, "{}({}) {}", name, args.join(", "), body),

            Self::FunctionCall { args, name } => {
                let args_to_string = args
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}({})", name, args_to_string)
            },

            Self::InfixFunctionCall { left, operator, right } =>
                write!(f, "({} {} {})", left, operator, right),

            Self::Integer(i) => write!(f, "{}", i),

            Self::Reference(r) => write!(f, "{}", r),

            Self::Text(t) => write!(f, "\"{}\"", t),

            Self::Variable { body, name } => 
                write!(f, "{} := {}", name, body),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_boolean() {
        assert_eq!("TRUE", Node::Boolean(true).to_string());
        assert_eq!("FALSE", Node::Boolean(false).to_string());
    }

    #[test]
    fn display_datetime() {
        let date_time = chrono::DateTime::from_utc(
            chrono::NaiveDate::from_ymd_opt(2022, 10, 12).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            chrono::Utc,
        );
        let date = Node::DateTime(date_time);

        assert_eq!("2022-10-12 00:00:00 UTC", date.to_string());
    }


    #[test]
    fn display_float() {
        assert_eq!(
            "123.45",
            Node::Float(123.45).to_string());
    }

    #[test]
    fn display_function() {
        assert_eq!(
            "foo(a, b, c) 1", 
            Node::Function {
                name: "foo".to_owned(),
                args: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                body: Box::new(Node::Integer(1)),
            }.to_string());
    }

    #[test]
    fn display_function_call() {
        assert_eq!(
            "bar(1, \"foo\")", 
            Node::FunctionCall {
                name: "bar".to_owned(),
                args: vec![
                    Box::new(Node::Integer(1)), 
                    Box::new(Node::Text("foo".to_owned()))
                ],
            }.to_string());
    }

    #[test]
    fn display_infix_function() {
        assert_eq!(
            "(1 * 2)", 
            Node::InfixFunctionCall { 
                left: Box::new(Node::Integer(1)),
                operator: "*".to_owned(),
                right: Box::new(Node::Integer(2)),
            }.to_string());
    }

    #[test]
    fn display_integer() {
        assert_eq!(
            "123",
            Node::Integer(123).to_string());
    }

    #[test]
    fn display_reference() {
        assert_eq!(
            "foo",
            Node::Reference("foo".to_owned()).to_string());
    }

    #[test]
    fn display_text() {
        assert_eq!(
            "\"foo\"",
            Node::Text("foo".to_string()).to_string());
    }

    #[test]
    fn display() {
        assert_eq!(
            "foo := 1", 
            Node::Variable {
                name: "foo".to_owned(), 
                body: Box::new(Node::Integer(1))
            }.to_string());
    }
}
