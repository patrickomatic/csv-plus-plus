//! # Runtime
//!
use clap::Parser;
use std::fmt;

use crate::{CliArgs, CompilationTarget, Init, Options, OutputTarget, Result, SourceCode, Template};
use crate::compiler::token_library::TokenLibrary;

#[derive(Debug)]
pub struct Runtime {
    pub options: Options,
    pub target: OutputTarget,
    pub source_code: SourceCode,
    pub template: Template,
    pub token_library: TokenLibrary,
}

impl Runtime {
    pub fn from_cli_args() -> Result<Self> {
        Self::new(CliArgs::parse())
    }

    pub fn new(cli_args: CliArgs) -> Result<Self> {
        let token_library = TokenLibrary::build()?;
        let init = Init::from_cli_args(cli_args, &token_library)?;

        Ok(Self {
            options: init.options,
            target: init.output,
            source_code: init.source_code,
            template: Template::default(),
            token_library: TokenLibrary::build()?,
        })
    }

    pub fn target<'a>(&'a self) -> Box<dyn CompilationTarget + 'a> {
        self.target.compilation_target(self)
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
        let cli_args = CliArgs {
            input_filename: PathBuf::from("foo.csvpp"),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        let runtime = Runtime::new(cli_args);

        assert!(runtime.is_ok())
    }

    #[test]
    fn display() {
        // let output = 
        // TODO
    }
}

