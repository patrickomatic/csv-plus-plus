//! # CodeSectionParser
//!
//! The `CodeSectionParser` relies on the `AstParser` to parse individual expressions, but it
//! handles the parsing of functions and variables.
//!
//! It will be looking for something like:
//!
//! ```bnf
//! <var_name> := <expr>
//! ```
//!
//! or 
//!
//! ```bnf
//! fn <function_name>(<fn-arg-1>, <fn-arg-2>, ...) <expr>
//! ```
//!
use std::collections::HashMap;

// use super::ast_parser::AstParser;
use crate::{Function, Node, Result};

pub struct CodeSectionParser {
    pub functions: HashMap<String, Function>,
    pub variables: HashMap<String, Box<dyn Node>>,
}

impl CodeSectionParser {
    pub fn parse(input: &str) -> Result<Self> {
        let mut functions = HashMap::new();
        let mut variables = HashMap::new();


        Ok(Self {
            functions,
            variables,
        })
    }
}
