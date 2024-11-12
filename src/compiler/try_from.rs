use crate::logger;
use crate::parser::ast_parser::AstParser;
use crate::{CliArgs, Compiler, Config, Output};
use log::debug;

impl TryFrom<&CliArgs> for Compiler {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> std::result::Result<Self, Self::Error> {
        let config = Config {
            key_values: AstParser::parse_key_value_str(
                &cli_args.key_values,
                &cli_args.input_filename,
            )?,
            ..Config::try_from(cli_args)?
        };

        logger::init(config.verbosity);

        let compiler = Self {
            config,
            output: Output::try_from(cli_args)?,
            input_filename: cli_args.input_filename.clone(),
        };

        debug!("Initted compiler: {compiler}");

        Ok(compiler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from() {
        let test_file = TestSourceCode::new("csv", "foo,bar,baz");
        let cli_args = CliArgs {
            input_filename: test_file.input_file.clone(),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        let compiler = Compiler::try_from(&cli_args);

        assert!(compiler.is_ok());
    }
}
