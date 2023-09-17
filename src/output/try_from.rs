use crate::{CliArgs, Output};

impl TryFrom<&CliArgs> for Output {
    type Error = crate::Error;

    fn try_from(cli_args: &CliArgs) -> std::result::Result<Self, Self::Error> {
        if let Some(sheet_id) = &cli_args.google_sheet_id {
            Ok(Self::from_google_sheet_id(sheet_id.to_string())?)
        } else if let Some(filename) = &cli_args.output_filename {
            Ok(Self::from_filename(filename.to_path_buf())?)
        } else {
            Err(crate::Error::InitError(
                    "Must specify either -g/--google-sheet-id or -o/--output-filename".to_string()))
        }
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn try_from_csv() {
        let cli_args = CliArgs {
            output_filename: Some(PathBuf::from("foo.csv")),
            ..Default::default()
        };
        let output_target = Output::try_from(&cli_args).unwrap();

        assert_eq!(output_target, Output::Csv(PathBuf::from("foo.csv")))
    }

    #[test]
    fn try_from_excel() {
        let cli_args = CliArgs {
            output_filename: Some(PathBuf::from("foo.xlsx")),
            ..Default::default()
        };
        let output_target = Output::try_from(&cli_args).unwrap();

        assert_eq!(output_target, Output::Excel(PathBuf::from("foo.xlsx")))
    }

    #[test]
    fn try_from_google_sheets() {
        let cli_args = CliArgs {
            google_sheet_id: Some("abc".to_string()),
            ..Default::default()
        };
        let output_target = Output::try_from(&cli_args).unwrap();

        assert_eq!(output_target, Output::GoogleSheets("abc".to_string()));
    }

    #[test]
    fn try_from_open_document() {
        let cli_args = CliArgs {
            output_filename: Some(PathBuf::from("foo.ods")),
            ..Default::default()
        };
        let output_target = Output::try_from(&cli_args).unwrap();

        assert_eq!(output_target, Output::OpenDocument(PathBuf::from("foo.ods")))
    }

    #[test]
    fn try_from_invalid() {
        let cli_args = CliArgs::default();
        let output_target = Output::try_from(&cli_args);

        assert!(output_target.is_err());
    }
}
