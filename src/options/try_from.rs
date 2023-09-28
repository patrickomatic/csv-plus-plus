use super::Options;
use crate::CliArgs;

impl TryFrom<&CliArgs> for Options {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> std::result::Result<Self, Self::Error> {
        Ok(Options {
            backup: cli_args.backup,
            google_account_credentials: cli_args.google_account_credentials.clone(),
            offset: (cli_args.x_offset, cli_args.y_offset),
            overwrite_values: !cli_args.safe,
            sheet_name: Self::sheet_name(cli_args)?,
            verbose: cli_args.verbose,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CliArgs;
    use std::path;

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
