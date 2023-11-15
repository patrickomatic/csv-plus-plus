//! # csv+++
//!
use colored::Colorize;
use csvpp::{Module, Result, Runtime};
use std::process;

fn compile_from_cli() -> Result<()> {
    let runtime = Runtime::from_cli_args()?;
    let module = Module::compile(&runtime)?;

    let target = runtime.target()?;
    if runtime.options.backup {
        target.write_backup()?;
    }
    target.write(&module)?;

    Ok(())
}

fn main() {
    if let Err(e) = compile_from_cli() {
        eprintln!("{}", e.to_string().red());
        process::exit(1)
    }
}
