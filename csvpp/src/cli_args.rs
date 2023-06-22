use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author = "Patrick Carroll")]
#[command(version, about, long_about = None)] 
pub struct CliArgs {
    #[arg(short,
          long,
          default_value_t = false, 
          help = "Create a backup of the spreadsheet before applying changes.")]
    pub backup: bool,

    #[arg(
        group = "output",
        short,
        long,
        help = "The id of the sheet - you can find this from the URL: https://docs.google.com/spreadsheets/d/< ... SHEET_ID ... >/edit#gid=",
    )]
    pub google_sheet_id: Option<String>,

    #[arg(
        short,
        long,
        num_args = 0..,
        help = "`key=value` pairs where the right side will be parsed and made available as a variable",
    )]
    pub key_values: Vec<String>,

    #[arg(
        group = "output",
        short,
        long,
        help = "The file to write to (must be .csv, .ods, .xls)",
    )]
    pub output_filename: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Do not overwrite values in the spreadsheet being written to. The default is to overwrite",
    )]
    pub safe: bool,

    #[arg(
        short = 'n',
        long,
        help = "The name of the sheet to apply the template to.",
    )]
    pub sheet_name: Option<String>,

    #[arg(
        short,
        long,
        default_value_t = false,
    )]
    pub verbose: bool,

    #[arg(
        short, 
        long,
        default_value_t = 0,
        help = "Apply the template offset by this many cells",
    )]
    pub x_offset: u32,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "Apply the template offset by this many rows",
    )]
    pub y_offset: u32,

    #[arg(required = true)]
    pub input_filename: PathBuf,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            backup: false,
            google_sheet_id: None,
            input_filename: PathBuf::new(),
            key_values: vec![],
            output_filename: None,
            safe: false,
            sheet_name: None,
            verbose: false,
            x_offset: 0,
            y_offset: 0,
        }
    }
}
