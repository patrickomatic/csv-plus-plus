use flexbuffers;
use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use crate::{Cell, Node, Runtime};

pub type Spreadsheet = Vec<Vec<Cell>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Template {
    // TODO can I enforce that this is only Node::Functions?
    functions: HashMap<String, Node>,
    spreadsheet: RefCell<Spreadsheet>,
    variables: HashMap<String, Node>,
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
    // TODO hmm should this just move onto impl Runtime rather than taking a runtime
    pub fn write_object_code(runtime: &Runtime) -> () {
        let _object_code_filename = runtime.options.input.object_code_filename();
        let mut s = flexbuffers::FlexbufferSerializer::new();
        runtime.template.serialize(&mut s).unwrap();
        // TODO: write s to a file
        todo!()
    }
}


