//!
//!
use clap::Parser;
use std::fmt;

use crate::{CliArgs, Init, Modifier, Options, OutputTarget, Result, SourceCode, Template};
use crate::compiler::token_library::TokenLibrary;

pub struct Runtime {
    pub default_modifier: Modifier,
    pub options: Options,
    pub output: OutputTarget,
    pub source_code: SourceCode,
    pub template: Template,
    pub token_library: TokenLibrary,
}

impl Runtime {
    pub fn from_cli_args() -> Result<Self> {
        let init = Init::from_cli_args(CliArgs::parse())?;

        Ok(Self {
            default_modifier: Modifier::new(false),
            options: init.options,
            output: init.output,
            source_code: init.source_code,
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

## Parsed template
{}
"#, 
            self.options,
            self.template,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_runtime() {
        /*
        Runtime::new(Options

        }
        */
    }

    #[test]
    fn display() {
        // let output = 
    }
}

