use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use clap::Parser;

use crate::ast;
use crate::{Error, Node, SourceCode};

type GoogleSheetID = String;

#[derive(Clone, Debug, PartialEq)]
pub enum OutputTarget {
    GoogleSheets(GoogleSheetID),
    File(PathBuf),
}

impl fmt::Display for OutputTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GoogleSheets(id) => write!(f, "Google Sheets[{}]", id),
            Self::File(path) => write!(f, "{}", path.to_str().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct Options {
    pub backup: bool,
    pub input: SourceCode,
    pub key_values: HashMap<String, Box<dyn Node>>,
    pub offset: (u32, u32),
    pub output: OutputTarget,
    pub overwrite_values: bool,
    pub verbose: bool,
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            r#"
    backup: {}
    input: {}
    key_values: {:?}
    offset: ({}, {})
    output: {}
    overwrite_values: {}
    verbose: {}
            "#,
            self.backup,
            self.input,
            self.key_values,
            self.offset.0,
            self.offset.1,
            self.output,
            self.overwrite_values,
            self.verbose,
        )
    }
}

impl Options {
    fn build(cli_args: CliArgs) -> Result<Options, Error> {
        let output = if let Some(google_sheet_id) = cli_args.google_sheet_id {
            OutputTarget::GoogleSheets(google_sheet_id.to_string())
        } else if let Some(output_filename) = cli_args.output_filename {
            OutputTarget::File(output_filename)
        } else {
            return Err(Error::InitError(
                    "Must specify either -g/--google-sheet-id or -o/--output-filename".to_string()
                )
            )
        };
        
        let source_code = match SourceCode::open(cli_args.input_filename.clone()) {
            Ok(source_code) => source_code,
            Err(error) => 
                return Err(Error::InitError(
                    format!(
                        "Error opening source code {}: {}", 
                        // XXX make maybe a SourceCodeError error that takes the filename
                        cli_args.input_filename.into_os_string().into_string().unwrap(), 
                        error)))
        };

        Ok(Options {
            backup: cli_args.backup,
            input: source_code,
            key_values: ast::from_key_value_args(cli_args.key_values),
            offset: (cli_args.x_offset, cli_args.y_offset),
            output,
            overwrite_values: !cli_args.safe,
            verbose: cli_args.verbose,
        })
    }
}

// TODO actually use this
fn validate_output_filename(output_filename: &Path) -> Result<(), Error> {
    match output_filename.extension() {
        None => Err(Error::InitError("Output filename must end with .csv, .xlsx or .ods".to_string())),
        Some(ext) => {
            if ext.eq_ignore_ascii_case("csv") || ext.eq_ignore_ascii_case("xlsx") || ext.eq_ignore_ascii_case("ods") {
                Ok(())
            } else {
                Err(Error::InitError(
                    format!(
                        "{} is an unsupported extension: only .csv, .xlsx or .ods are supported.",
                        ext.to_str().unwrap()
                    )
                ))
            }
        }
    }
}

// TODO actually use this
fn validate_google_sheet_id(google_sheet_id: &str) -> Result<(), Error> {
    if google_sheet_id.chars().all(char::is_alphanumeric) {
        Ok(())
    } else {
        Err(Error::InitError("The GOOGLE_SHEET_ID must be all letters and digits.".to_string()))
    }
}

#[derive(Debug, Parser)]
#[command(author = "Patrick Carroll")]
#[command(version, about, long_about = None)] 
struct CliArgs {
    #[arg(short,
          long,
          default_value_t = false, 
          help = "Create a backup of the spreadsheet before applying changes.")]
    backup: bool,

    #[arg(
        group = "output",
        short,
        long,
        help = "The id of the sheet - you can find this from the URL: https://docs.google.com/spreadsheets/d/< ... SHEET_ID ... >/edit#gid=",
        // validator = validate_google_sheet_id)]
    )]
    google_sheet_id: Option<String>,

    #[arg(
        short,
        long,
        default_value_t = String::from(""),
        help = "A comma-separated list of key=values which will be made available to the template",
    )]
    key_values: String,

    #[arg(
        group = "output",
        short,
        long,
        help = "The file to write to (must be .csv, .ods, .xls)",
        // validator = validate_output_filename)]
    )]
    output_filename: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Do not overwrite values in the spreadsheet being written to. The default is to overwrite",
    )]
    safe: bool,

    #[arg(
        short = 'n',
        long,
        help = "The name of the sheet to apply the template to.",
    )]
    sheet_name: Option<String>,

    #[arg(
        short,
        long,
        default_value_t = false,
    )]
    verbose: bool,

    #[arg(
        short, 
        long,
        default_value_t = 0,
        help = "Apply the template offset by this many cells",
    )]
    x_offset: u32,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "Apply the template offset by this many rows",
    )]
    y_offset: u32,

    #[arg(required = true)]
    input_filename: PathBuf,
}

pub fn parse_cli_args() -> Result<Options, Error> {
    Options::build(CliArgs::parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_google_sheet_id_valid() {
        assert!(validate_google_sheet_id("abc123").is_ok())
    }
    
    #[test]
    fn validate_google_sheet_id_invalid() {
        assert!(validate_google_sheet_id("abc #!*)! 123").is_err())
    }

    #[test]
    fn validate_output_filename_csv() {
        assert!(validate_output_filename(&Path::new("/foo/bar/file.csv")).is_ok())
    }

    #[test]
    fn validate_output_filename_xlsx() {
        assert!(validate_output_filename(&Path::new("FileName.xlsx")).is_ok())
    }

    #[test]
    fn validate_output_filename_ods() {
        assert!(validate_output_filename(&Path::new("TEST.ODS")).is_ok())
    }

    #[test]
    fn validate_output_filename_invalid() {
        assert!(validate_output_filename(&Path::new("/home/Patrick/not_a_valid_file")).is_err())
    }
}
