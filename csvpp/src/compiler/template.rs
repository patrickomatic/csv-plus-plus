use flexbuffers;
// use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use super::csv_section;
use super::code_section_parser::CodeSectionParser;
use crate::{Cell, Node, Function, Result, Runtime};

pub type Spreadsheet = Vec<Vec<Cell>>;

// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub struct Template {
    pub functions: HashMap<String, Function>,
    pub spreadsheet: RefCell<Spreadsheet>,
    pub variables: HashMap<String, Box<dyn Node>>,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "variables: {:?}", &self.variables)?;
        write!(f, "functions: {:?}", &self.functions)?;
        write!(f, "rows: {}", self.spreadsheet.borrow().len())
    }
}

impl Default for Template {
    fn default() -> Self {
        Self {
            functions: HashMap::new(),
            spreadsheet: RefCell::new(Vec::new()),
            variables: HashMap::new(),
        }
    }
}

impl Template {
    pub fn compile(runtime: &Runtime) -> Result<Self> {
        // TODO do these in parallel
        // XXX get variables and insert them into the template
        let spreadsheet = csv_section::parse(&runtime)?;

        let template = if let Some(code_section) = &runtime.source_code.code_section {
            let code_section_parser = CodeSectionParser::parse(&code_section)?;

            Template {
                functions: code_section_parser.functions,
                spreadsheet: RefCell::new(spreadsheet),
                variables: code_section_parser.variables,
            }
        } else {
            Self::new(spreadsheet)
        };

        template.resolve_cell_variables(runtime)
    }

    // XXX mix in the functions and variables from the runtime
    fn new(spreadsheet: Spreadsheet) -> Self {
        Template {
            spreadsheet: RefCell::new(spreadsheet),
            ..Self::default()
        }
    }

    fn resolve_cell_variables(self, _runtime: &Runtime) -> Result<Self> {
        // TODO
        Ok(self)
    }

    // TODO hmm should this just move onto impl Runtime rather than taking a runtime
    pub fn write_object_code(runtime: &Runtime) -> () {
        let _object_code_filename = runtime.source_code.object_code_filename();
        let mut _s = flexbuffers::FlexbufferSerializer::new();
        // runtime.template.serialize(&mut s).unwrap();
        // TODO: write `s` to a file
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
