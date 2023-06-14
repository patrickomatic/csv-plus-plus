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
    use super::*;
    
    #[test]
    fn from_cli_args() {
        // TODO
    }
}
