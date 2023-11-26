use crate::ast::{BuiltinFunction, BuiltinVariable};
use crate::parser::ast_lexer;
use crate::parser::ast_parser::AstParser;
use crate::parser::cell_lexer;
use crate::{CliArgs, Compiler, Options, Output, SourceCode};
use std::sync;

impl TryFrom<&CliArgs> for Compiler {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> std::result::Result<Self, Self::Error> {
        let source_code = SourceCode::open(&cli_args.input_filename)?;

        let mut compiler = Self {
            builtin_functions: BuiltinFunction::all(),
            builtin_variables: BuiltinVariable::all(),
            options: Options::try_from(cli_args)?,
            output: Output::try_from(cli_args)?,
            source_code: sync::Arc::new(source_code),
            ast_token_library: ast_lexer::TokenLibrary::build()?,
            cell_token_library: cell_lexer::TokenLibrary::build()?,
        };

        // we have to parse key/values afterwards, because we need an initialized `Compiler` to do so
        compiler.options.key_values =
            AstParser::parse_key_value_str(&cli_args.key_values, &compiler)?;

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
