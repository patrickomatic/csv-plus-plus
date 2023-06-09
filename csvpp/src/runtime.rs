//!
//!
use std::fmt;

use crate::{Error, Modifier, Options, Template};
use crate::compiler::token_library::TokenLibrary;

pub struct Runtime {
    pub default_modifier: Modifier,
    pub options: Options,
    // TODO need to make the template mutable
    pub template: Template,
    pub token_library: TokenLibrary,
}

impl Runtime {
    pub fn new(options: Options) -> Result<Self, Error> {
        Ok(Self {
            default_modifier: Modifier::new(false),
            options,
            template: Template::default(),
            token_library: TokenLibrary::build()?,
        })
    }
}


impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"
# csv++ 

## Called with options
{}

## Parsed csvpp template
{}
"#, 
            self.options,
            self.template,
        )
    }
}


