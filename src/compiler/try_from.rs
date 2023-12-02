use crate::parser::ast_parser::AstParser;
use crate::{ArcSourceCode, CliArgs, Compiler, Options, Output, SourceCode};

impl TryFrom<&CliArgs> for Compiler {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> std::result::Result<Self, Self::Error> {
        let source_code = ArcSourceCode::new(SourceCode::open(&cli_args.input_filename)?);
        let compiler = Self {
            options: Options {
                key_values: AstParser::parse_key_value_str(
                    &cli_args.key_values,
                    source_code.clone(),
                )?,
                ..Options::try_from(cli_args)?
            },
            output: Output::try_from(cli_args)?,
            source_code,
        };

        compiler.info(&compiler);

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
