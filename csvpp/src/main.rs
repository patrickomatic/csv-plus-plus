//! # csv+++
//!
//! # Examples
//!
//! ```
//! ```
//! 
use csvpp::{Error, compile_template, parse_cli_args};
use std::process;

fn compile_from_cli() -> Result<(), Error> {
    let options = parse_cli_args()?;

    let runtime = compile_template(options)?;
    if runtime.options.backup {
        if runtime.options.verbose {
            // TODO: better message
            println!("Backing up {}", runtime.options.backup)
        }
        todo!();
    }

    if runtime.options.verbose {
        println!("{}", runtime.to_string());
    }

    Ok(())

    // template.write_compiled_template(&options, &template);
    // writer.write(&options, &template)
}

// TODO wrap these two error handlings in a function that calls with ?
fn main() {
    if let Err(e) = compile_from_cli() {
        // TODO do more in verbose mode?
        eprintln!("{}", e.to_string());
        process::exit(1)
    }
}
