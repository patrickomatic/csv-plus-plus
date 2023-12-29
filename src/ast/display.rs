use super::{Ast, Node, VariableValue};
use a1_notation::A1;
use std::fmt;

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{}", if *b { "TRUE" } else { "FALSE" }),

            Self::DateTime(d) => write!(f, "{d}"),

            Self::Float(fl) => write!(f, "{fl}"),

            Self::Function { args, body, name } => {
                let joined_args = args.join(", ");
                write!(f, "{name}({joined_args}) {body}")
            }

            Self::FunctionCall { args, name } => {
                let args_to_string = args
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{name}({args_to_string})")
            }

            Self::InfixFunctionCall {
                left,
                operator,
                right,
            } => write!(f, "({left} {operator} {right})"),

            Self::Integer(i) => write!(f, "{i}"),

            Self::Reference(r) => write!(f, "{r}"),

            Self::Text(t) => write!(f, "\"{t}\""),

            Self::Variable { name, value } => write!(f, "{name} := {value}"),
        }
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Absolute(address) => write!(f, "{address}"),

            Self::Ast(ast) => write!(f, "{ast}"),

            Self::ColumnRelative { fill, column } => {
                let row_range: A1 = (*fill).into();
                write!(f, "{}", row_range.with_x(column.x))
            }

            Self::Row(row) => write!(f, "{row}"),

            Self::RowRelative { fill, .. } => {
                let row_range: A1 = (*fill).into();
                write!(f, "{row_range}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::build_date_time_ymd;

    use super::*;

    #[test]
    fn display_boolean() {
        let bt: Node = true.into();
        let bf: Node = false.into();

        assert_eq!("TRUE", bt.to_string());
        assert_eq!("FALSE", bf.to_string());
    }

    #[test]
    fn display_datetime() {
        let d: Node = build_date_time_ymd(2022, 10, 12).into();

        assert_eq!("2022-10-12", d.to_string());
    }

    #[test]
    fn display_float() {
        let f: Node = 123.45.into();

        assert_eq!("123.45", f.to_string());
    }

    #[test]
    fn display_function() {
        assert_eq!(
            "foo(a, b, c) 1",
            Node::fn_def("foo", &["a", "b", "c"], 1).to_string()
        );
    }

    #[test]
    fn display_function_call() {
        assert_eq!(
            "bar(1, \"foo\")",
            Node::fn_call("bar", &[1.into(), Node::text("foo")]).to_string()
        );
    }

    #[test]
    fn display_infix_function() {
        assert_eq!("(1 * 2)", Node::infix_fn_call(1, "*", 2).to_string());
    }

    #[test]
    fn display_integer() {
        let i: Node = 123.into();
        assert_eq!("123", i.to_string());
    }

    #[test]
    fn display_reference() {
        assert_eq!("foo", Node::reference("foo").to_string());
    }

    #[test]
    fn display_text() {
        assert_eq!("\"foo\"", Node::text("foo").to_string());
    }

    #[test]
    fn display_var() {
        assert_eq!(
            "foo := 1",
            Node::var("foo", VariableValue::Ast(1.into())).to_string()
        );
    }
}
