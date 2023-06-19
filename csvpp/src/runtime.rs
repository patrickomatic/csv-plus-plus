//! # Runtime
//!
use clap::Parser;
use std::fmt;

use crate::{CliArgs, Init, Modifier, Options, OutputTarget, Result, SourceCode, Template};
use crate::compiler::token_library::TokenLibrary;

#[derive(Debug)]
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
        Self::new(CliArgs::parse())
    }

    pub fn new(cli_args: CliArgs) -> Result<Self> {
        let init = Init::from_cli_args(cli_args)?;
        // TODO this needs to merge in variables from both the CLI and the runtime variables

        Ok(Self {
            default_modifier: Modifier::default(),
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
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn new() {
        let mut cli_args = CliArgs::default();
        cli_args.input_filename = PathBuf::from("foo.csvpp");
        cli_args.google_sheet_id = Some("abc123".to_string());

        let runtime = Runtime::new(cli_args);
        assert!(runtime.is_ok())
    }

    #[test]
    fn display() {
        // let output = 
        // TODO
    }
}

