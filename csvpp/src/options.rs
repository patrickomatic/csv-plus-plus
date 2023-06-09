use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use clap::Parser;

use crate::ast;
use crate::source_code;

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
    pub input: source_code::SourceCode,
    pub key_values: HashMap<String, ast::Node>,
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

// TODO: maybe come up with a better name for this class that isn't similar to the Option primitive
impl Options {
    fn build(cli_args: CliArgs) -> Options {
        let output = if let Some(google_sheet_id) = cli_args.google_sheet_id {
            OutputTarget::GoogleSheets(google_sheet_id.to_string())
        } else if let Some(output_filename) = cli_args.output_filename {
            OutputTarget::File(output_filename)
        } else {
            // XXX return a Result<> or move this check up into the CLI parser
            panic!("Must specify either -g/--google-sheet-id or -o/--output-filename");
        };
        
        let source_code = match source_code::SourceCode::open(cli_args.input_filename) {
            Ok(source_code) => source_code,
            Err(error) => panic!("{}", error),
        };

        Options {
            backup: cli_args.backup,
            input: source_code,
            key_values: ast::Node::from_key_value_args(cli_args.key_values),
            offset: (cli_args.x_offset, cli_args.y_offset),
            output,
            overwrite_values: !cli_args.safe,
            verbose: cli_args.verbose,
        }
    }
}

// TODO actually use this
fn validate_output_filename(output_filename: &Path) -> Result<(), String> {
    match output_filename.extension() {
        None => Err(String::from("Output filename must end with .csv, .xlsx or .ods")),
        Some(ext) => {
            if ext.eq_ignore_ascii_case("csv") || ext.eq_ignore_ascii_case("xlsx") || ext.eq_ignore_ascii_case("ods") {
                Ok(())
            } else {
                Err(
                    format!(
                        "{} is an unsupported extension: only .csv, .xlsx or .ods are supported.",
                        ext.to_str().unwrap()
                    )
                )
            }
        }
    }
}

// TODO actually use this
fn validate_google_sheet_id(google_sheet_id: &str) -> Result<(), &str> {
    if google_sheet_id.chars().all(char::is_alphanumeric) {
        Ok(())
    } else {
        Err("The GOOGLE_SHEET_ID must be all letters and digits.")
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

pub fn parse_cli_args() -> Options {
    Options::build(CliArgs::parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_google_sheet_id_valid() {
        assert_eq!(Ok(()), validate_google_sheet_id("abc123"))
    }
    
    #[test]
    fn validate_google_sheet_id_invalid() {
        assert_eq!(
            Err("The GOOGLE_SHEET_ID must be all letters and digits."), 
            validate_google_sheet_id("abc #!*)! 123")
        )
    }

    #[test]
    fn validate_output_filename_csv() {
        assert_eq!(Ok(()), validate_output_filename(&Path::new("/foo/bar/file.csv")))
    }

    #[test]
    fn validate_output_filename_xlsx() {
        assert_eq!(Ok(()), validate_output_filename(&Path::new("FileName.xlsx")))
    }

    #[test]
    fn validate_output_filename_ods() {
        assert_eq!(Ok(()), validate_output_filename(&Path::new("TEST.ODS")))
    }

    #[test]
    fn validate_output_filename_invalid() {
        assert_eq!(
            Err(String::from("Output filename must end with .csv, .xlsx or .ods")),
            validate_output_filename(&Path::new("/home/Patrick/not_a_valid_file"))
        )
    }
}
