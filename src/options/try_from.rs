use crate::CliArgs;
use crate::parser::ast_parser::AstParser;
use super::Options;

impl TryFrom<&CliArgs> for Options {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> std::result::Result<Self, Self::Error> {
        let key_values_as_str = cli_args
            .key_values
            .iter()
            .map(|s| s.as_str())
            .collect();

        Ok(Options {
            backup: cli_args.backup,
            google_account_credentials: cli_args.google_account_credentials.clone(),
            key_values: AstParser::parse_key_value_str(key_values_as_str)?,
            offset: (cli_args.x_offset, cli_args.y_offset),
            overwrite_values: !cli_args.safe,
            sheet_name: Self::sheet_name(cli_args)?,
            verbose: cli_args.verbose,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path;
    use crate::CliArgs;
    use super::*;

    #[test]
    fn try_from_no_sheet_name() {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            ..Default::default()
        };
        let options = Options::try_from(&cli_args).unwrap();

        assert_eq!(options.sheet_name, "foo");
    }

    #[test]
    fn try_from_sheet_name() {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            sheet_name: Some("bar".to_string()),
            ..Default::default()
        };
        let options = Options::try_from(&cli_args).unwrap();

        assert_eq!(options.sheet_name, "bar");
    }
}
