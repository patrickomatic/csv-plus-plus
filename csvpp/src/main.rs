//! # csv+++
//!
//! # Examples
//!
//! ```
//! ```
//! 
// TODO:
// * reduce the amount of things that are exported publicly 
//   * make the ASTs private
//
// * proper line numbers and indexes in error messages throughout
//
// * writing to:
//   * CSV
//   * Excel
//   * GoogleSheets
//   * OpenDocument
//
// * writing object files via Serde?
//
// * make sure the --verbose output is actually useful
//
// * extract the A1-notation stuff into a separate crate
//
// * target the lowest versions of all dependencies
//
use csvpp::{Result, Runtime, Template};
use std::process;

fn compile_from_cli() -> Result<()> {
    let runtime = Runtime::from_cli_args()?;
    let template = Template::compile(&runtime)?;
    let target = runtime.output.compilation_target();

    if runtime.options.backup {
        if runtime.options.verbose {
            println!("Backing up output file: {}", &runtime.output)
        }

        target.write_backup()?;
    }

    if runtime.options.verbose {
        println!("{}", runtime);
    }

    // TODO write (and read) object files
    // template.write_compiled_template(&options, &template);

    let target = runtime.output.compilation_target();
    target.write(&runtime.options, &template)?;

    Ok(())
}

fn main() {
    if let Err(e) = compile_from_cli() {
        // TODO do more in verbose mode?
        eprintln!("{}", e);
        process::exit(1)
    }
}
