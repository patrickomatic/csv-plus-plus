use super::Options;
use crate::logger::u8_into_level_filter;
use crate::{CliArgs, Error, Result};
use std::collections;

impl From<chrono_tz::ParseError> for Error {
    fn from(tz_e: chrono_tz::ParseError) -> Self {
        Error::InitError(format!("Error parsing time zone: {tz_e}"))
    }
}

impl TryFrom<&CliArgs> for Options {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> Result<Self> {
        let time_zone = if let Some(tz) = &cli_args.time_zone {
            Some(chrono_tz::Tz::from_str_insensitive(tz)?)
        } else {
            None
        };

        Ok(Options {
            backup: cli_args.backup,
            google_account_credentials: cli_args.google_account_credentials.clone(),
            key_values: collections::HashMap::new(),
            offset: (cli_args.x_offset, cli_args.y_offset),
            overwrite_values: !cli_args.safe,
            sheet_name: Self::sheet_name(cli_args)?,
            time_zone,
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

    #[test]
    fn try_from_time_zone_invalid() {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            time_zone: Some("foo".to_string()),
            ..Default::default()
        };

        assert!(Options::try_from(&cli_args).is_err());
    }

    #[test]
    fn try_from_time_zone_valid() {
        let cli_args = CliArgs {
            input_filename: path::PathBuf::from("foo.csvpp"),
            time_zone: Some("EST".to_string()),
            ..Default::default()
        };

        let options = Options::try_from(&cli_args).unwrap();
        assert_eq!(options.time_zone, Some(chrono_tz::Tz::EST));
    }
}
