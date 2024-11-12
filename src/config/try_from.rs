use super::Config;
use crate::logger::u8_into_level_filter;
use crate::{CliArgs, Result};
use std::collections;

impl TryFrom<&CliArgs> for Config {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> Result<Self> {
        Ok(Config {
            backup: cli_args.backup,
            google_account_credentials: cli_args.google_account_credentials.clone(),
            key_values: collections::HashMap::new(),
            offset: (cli_args.x_offset, cli_args.y_offset),
            overwrite_values: !cli_args.safe,
            sheet_name: Self::sheet_name(cli_args)?,
            use_cache: !cli_args.no_cache,
            verbosity: u8_into_level_filter(cli_args.verbose)?,
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
        let config = Config::try_from(&cli_args).unwrap();

        assert_eq!(config.sheet_name, "foo");
    }

    #[test]
    fn try_from_sheet_name() {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            sheet_name: Some("bar".to_string()),
            ..Default::default()
        };
        let config = Config::try_from(&cli_args).unwrap();

        assert_eq!(config.sheet_name, "bar");
    }
}
