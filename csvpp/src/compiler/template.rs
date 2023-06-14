use flexbuffers;
// use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use super::csv_section;
use super::code_section::CodeSection;
use crate::{Cell, Node, Function, Result, Runtime};

pub type Spreadsheet = Vec<Vec<Cell>>;

// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug)]
pub struct Template {
    functions: HashMap<String, Function>,
    spreadsheet: RefCell<Spreadsheet>,
    variables: HashMap<String, Box<dyn Node>>,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: include variables and functions
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
        let spreadsheet = csv_section::parse(&runtime)?;
        let code_section = CodeSection::parse(&runtime)?;

        let template = Template {
            functions: code_section.functions,
            spreadsheet: RefCell::new(spreadsheet),
            variables: code_section.variables,
        };

        template.resolve_cell_variables(runtime)
    }

    fn resolve_cell_variables(self, runtime: &Runtime) -> Result<Self> {
        // TODO
        Ok(self)
    }

    // TODO hmm should this just move onto impl Runtime rather than taking a runtime
    pub fn write_object_code(runtime: &Runtime) -> () {
        let _object_code_filename = runtime.source_code.object_code_filename();
        let mut s = flexbuffers::FlexbufferSerializer::new();
        // runtime.template.serialize(&mut s).unwrap();
        // TODO: write s to a file
        todo!()
    }
}


