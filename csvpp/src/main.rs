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
    let target = runtime.output.compilation_target();
    let template = Template::compile(&runtime)?;

    if runtime.options.backup {
        if runtime.options.verbose {
            println!("Backing up output file: {}", &runtime.output)
        }

        target.write_backup()?;
    }

    if runtime.options.verbose {
        println!("{}", runtime.to_string());
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
        eprintln!("{}", e.to_string());
        process::exit(1)
    }
}
