use std::collections::HashMap;
use chrono;

type FunctionArguments = Vec<String>;
type FunctionCallArguments = Vec<Node>;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Boolean(bool),

    Date(chrono::NaiveDateTime),

    Function { 
        arguments: FunctionArguments,
        body: Box<Node>,
        name: String, 
    },

    FunctionCall {
        arguments: Box<FunctionCallArguments>,
        infix: bool,
        name: String,
    },

    Number(i32), // XXX need to handle floats too

    // XXX need to be able to init from a cell and row index
    Reference(String),

    // XXX map this to a static table with closures that do the logic?
    RuntimeValue(String),

    String(String),
}

impl Node {
    pub fn from_key_value_args(_key_value_args: String) -> HashMap<String, Node> {
        // TODO parse arg
        HashMap::new()
    }
}
