//! # csv+++
//!
//! # Examples
//!
//! ```
//! ```
//! 
use csvpp::{Result, Runtime, Template};
use std::process;

fn compile_from_cli() -> Result<()> {
    let runtime = Runtime::from_cli_args()?;

    let template = Template::compile(&runtime)?;
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
    // writer.write(&runtime, &template)
}

// TODO wrap these two error handlings in a function that calls with ?
fn main() {
    if let Err(e) = compile_from_cli() {
        // TODO do more in verbose mode?
        eprintln!("{}", e.to_string());
        process::exit(1)
    }
}
