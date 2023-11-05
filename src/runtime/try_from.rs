use crate::ast::{BuiltinFunction, BuiltinVariable};
use crate::parser::ast_lexer;
use crate::parser::ast_parser::AstParser;
use crate::parser::modifier_lexer;
use crate::{CliArgs, Options, Output, Runtime, SourceCode};
use std::sync;

impl TryFrom<&CliArgs> for Runtime {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> std::result::Result<Self, Self::Error> {
        let source_code = SourceCode::open(&cli_args.input_filename)?;

        let mut runtime = Self {
            builtin_functions: BuiltinFunction::all(),
            builtin_variables: BuiltinVariable::all(),
            options: Options::try_from(cli_args)?,
            output: Output::try_from(cli_args)?,
            source_code: sync::Arc::new(source_code),
            ast_token_library: ast_lexer::TokenLibrary::build()?,
            cell_token_library: modifier_lexer::TokenLibrary::build()?,
        };

        // we have to parse key/values afterwards, because we need an initialized `Runtime` to do so
        runtime.options.key_values =
            AstParser::parse_key_value_str(&cli_args.key_values, &runtime)?;

        runtime.info(&runtime);

        Ok(runtime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestFile;

    #[test]
    fn try_from() {
        let test_file = TestFile::new("csv", "foo,bar,baz");
        let cli_args = CliArgs {
            input_filename: test_file.input_file.clone(),
            google_sheet_id: Some("abc123".to_string()),
            ..Default::default()
        };
        let runtime = Runtime::try_from(&cli_args);

        assert!(runtime.is_ok());
    }
}
