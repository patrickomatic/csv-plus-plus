//! # Function
//!
//! A definition of a function.  Note that this is distinctly different than the calling of a
//! function (`FunctionCall`.)
//!
// use serde::{Serialize, Deserialize};
use std::fmt;

// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub struct Function { 
    args: super::FunctionArgs,
    body: Box<dyn super::Node>,
    name: super::FunctionName, 
}

impl super::Node for Function {}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}) {}", self.name, self.args.join(", "), self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn display() {
        let function = Function { 
            args: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            body: Box::new(Integer(1)),
            name: "foo".to_string(),
        };
        assert_eq!("foo(a, b, c) 1", function.to_string());
    }
}
