//! # csv+++
//!
use csvpp::{Error, Runtime, Template};
use std::process;

fn compile_from_cli() -> Result<(), Box<Error>> {
    let runtime = Runtime::from_cli_args()?;
    if runtime.options.verbose {
        println!("{runtime}");
    }

    let template = Template::compile(&runtime)?;
    if runtime.options.verbose {
        println!("{template}");
    }

    let target = runtime.target()?;

    if runtime.options.backup {
        if runtime.options.verbose {
            println!("Backing up output file: {}", &runtime.output)
        }

        target.write_backup()?;
    }

    template.write_object_file(&runtime.source_code)?;

    target.write(&template)?;

    Ok(())
}

fn main() {
    if let Err(e) = compile_from_cli() {
        eprintln!("{e}");
        process::exit(1)
    }
}
