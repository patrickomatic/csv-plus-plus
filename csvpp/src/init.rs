//! # Init
//!
//! The first phase of processing which gets everything ready for parsing and compilation.  This
//! takes the `CliArgs` supplied by the user and:
//!
//! * Reads the source and does a rough parse (splitting it by `---`)
//! * Validates the output target
//! * Creates an `Options` given the various CLI options
//!
use crate::ast;
use crate::{CliArgs, Options, OutputTarget, Result, SourceCode};

pub struct Init {
    pub options: Options,
    pub output: OutputTarget,
    pub source_code: SourceCode,
}

impl Init {
    pub fn from_cli_args(cli_args: CliArgs) -> Result<Init> {
        let output = OutputTarget::from_cli_args(&cli_args)?;
        let source_code = SourceCode::open(cli_args.input_filename)?;

        Ok(Init {
            options: Options {
                backup: cli_args.backup,
                key_values: ast::from_key_value_args(cli_args.key_values),
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

    use super::*;
    
    #[test]
    fn from_cli_args() {
        let init = Init::from_cli_args(CliArgs {
            output_filename: Some(PathBuf::from("foo.xlsx")),
            input_filename: PathBuf::from("foo.csvpp"),
            ..CliArgs::default()
        }).unwrap();

        // it takes defaults for options - we don't need to check all
        assert_eq!(init.options.backup, false);
        assert_eq!(init.options.overwrite_values, true);

        assert_eq!(init.output, OutputTarget::Excel(PathBuf::from("foo.xlsx")));
        assert_eq!(init.source_code.filename, PathBuf::from("foo.csvpp"))
    }
}
