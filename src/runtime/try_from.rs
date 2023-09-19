use crate::ast::{BuiltinFunction, BuiltinVariable};
use crate::{CliArgs, Options, Output, Runtime, SourceCode};

impl TryFrom<&CliArgs> for Runtime {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            builtin_functions: BuiltinFunction::all(),
            builtin_variables: BuiltinVariable::all(),
            options: Options::try_from(cli_args)?,
            output: Output::try_from(cli_args)?,
            source_code: SourceCode::open(&cli_args.input_filename)?,
        })
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
