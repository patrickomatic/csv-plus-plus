//! # csv+++
//!
use csvpp::{Result, Runtime, Template};
use std::process;

fn compile_from_cli() -> Result<()> {
    let runtime = Runtime::from_cli_args()?;
    let template = Template::compile(&runtime)?;

    template.write_object_file(&runtime.source_code)?;

    let target = runtime.target()?;
    if runtime.options.backup {
        target.write_backup()?;
    }
    target.write(&template)?;

    Ok(())
}

fn main() {
    if let Err(e) = compile_from_cli() {
        eprintln!("{e}");
        process::exit(1)
    }
}
