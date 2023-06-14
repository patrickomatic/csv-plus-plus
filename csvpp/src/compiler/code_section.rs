//! # CodeSection
use std::collections::HashMap;
use crate::{Function, Node, Result, Runtime};

pub struct CodeSection {
    pub functions: HashMap<String, Function>,
    pub variables: HashMap<String, Box<dyn Node>>,
}

impl CodeSection {
    pub fn parse(runtime: &Runtime) -> Result<Self> {
        // TODO
        Ok(CodeSection {
            functions: HashMap::new(),
            variables: HashMap::new(),
        })
    }
}
