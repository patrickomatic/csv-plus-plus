//! # Init
//!
//! The first phase of processing which gets everything ready for parsing and compilation.  This
//! takes the `CliArgs` supplied by the user and:
//!
//! * Reads the source and does a rough parse (splitting it by `---`)
//! * Validates the output target
//! * Creates an `Options` given the various CLI options
//!
use crate::compiler::ast_parser::AstParser;
use crate::compiler::token_library::TokenLibrary;
use crate::{CliArgs, Options, OutputTarget, Result, SourceCode};

pub struct Init {
    pub options: Options,
    pub output: OutputTarget,
    pub source_code: SourceCode,
}

impl Init {
    pub fn from_cli_args(cli_args: CliArgs, tl: &TokenLibrary) -> Result<Init> {
        let output = OutputTarget::from_cli_args(&cli_args)?;
        let source_code = SourceCode::open(cli_args.input_filename)?;

        let key_values_as_str = cli_args.key_values.iter().map(|s| s.as_str()).collect();

        Ok(Init {
            options: Options {
                backup: cli_args.backup,
                key_values: AstParser::parse_key_value_str(key_values_as_str, tl)?,
                offset: (cli_args.x_offset, cli_args.y_offset),
                overwrite_values: !cli_args.safe,
                verbose: cli_args.verbose,
            },
            output,
            source_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::compiler::token_library::TokenLibrary;
    use super::*;
    
    fn build_token_library() -> TokenLibrary {
        TokenLibrary::build().unwrap()
    }

    #[test]
    fn from_cli_args() {
        let init = Init::from_cli_args(CliArgs {
            output_filename: Some(PathBuf::from("foo.xlsx")),
            input_filename: PathBuf::from("foo.csvpp"),
            ..CliArgs::default()
        }, &build_token_library()).unwrap();

        // it takes defaults for options - we don't need to check all
        assert!(!init.options.backup);
        assert!(init.options.overwrite_values);

        assert_eq!(init.output, OutputTarget::Excel(PathBuf::from("foo.xlsx")));
        assert_eq!(init.source_code.filename, PathBuf::from("foo.csvpp"))
    }
}
