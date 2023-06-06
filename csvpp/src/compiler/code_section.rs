use crate::ast;
use crate::options;

use std::error::Error;
use std::collections::HashMap;

pub struct CodeSection {
    variables: HashMap<String, ast::Node>,
    // TODO can I enforce that this is only Node::Functions?
    functions: HashMap<String, ast::Node>,
}

pub fn parse(_options: &options::Options) -> Result<CodeSection, Box<dyn Error>> {
    // XXX do the parsing
    Ok(CodeSection {
        variables: HashMap::new(),
        functions: HashMap::new(),
    })
}


