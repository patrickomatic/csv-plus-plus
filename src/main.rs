//! # csv+++
//!
use csvpp::{Compiler, Result};
use log::error;
use std::process;

fn compile_from_cli() -> Result<()> {
    let compiler = Compiler::from_cli_args()?;
    let main_module = compiler.compile()?;

    let target = compiler.target()?;
    if compiler.config.backup {
        target.write_backup()?;
    }
    target.write(&main_module)?;

    Ok(())
}

fn main() {
    if let Err(e) = compile_from_cli() {
        error!("{e}");
        process::exit(1)
    }
}
