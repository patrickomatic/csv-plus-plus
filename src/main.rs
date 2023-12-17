//! # csv+++
//!
use colored::Colorize;
use csvpp::{Compiler, Result};
use std::process;

fn compile_from_cli() -> Result<()> {
    let compiler = Compiler::from_cli_args()?;
    let main_module = compiler.compile()?;

    let target = compiler.target()?;
    if compiler.options.backup {
        target.write_backup()?;
    }
    target.write(&main_module)?;

    Ok(())
}

fn main() {
    if let Err(e) = compile_from_cli() {
        eprintln!("{}", e.to_string().red());
        process::exit(1)
    }
}
