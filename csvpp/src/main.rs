//! # csv+++
//!
//! # Examples
//!
//! ```
//! ```
//! 
// TODO:
// 
// * reduce the amount of things that are exported publicly 
//   * make the ASTs private
//
// * proper line numbers and indexes in error messages throughout
//
// * writing to:
//   * Excel
//   * GoogleSheets
//   * OpenDocument
//
// * writing object files via Serde?
//
// * make sure the --verbose output is actually useful
//
// * target the lowest versions of all dependencies
//
use csvpp::{Error, Runtime, Template};
use std::process;

fn compile_from_cli() -> Result<(), Box<Error>> {
    let runtime = Runtime::from_cli_args()?;
    let template = Template::compile(&runtime)?;
    let target = runtime.target()?;

    if runtime.options.backup {
        if runtime.options.verbose {
            println!("Backing up output file: {}", &runtime.output)
        }

        target.write_backup()?;
    }

    if runtime.options.verbose {
        println!("{}", runtime);
    }

    template.write_object_file(&runtime.source_code)?;

    target.write(&template)?;

    Ok(())
}

fn main() {
    if let Err(e) = compile_from_cli() {
        // TODO do more in verbose mode?
        eprintln!("{}", e);
        process::exit(1)
    }
}
