//! # csv+++
//!
//! # Examples
//!
//! ```
//! ```
//! 
use csvpp::{compile_template, parse_cli_args};
use std::process;

fn main() {
    // TODO handle errors from parse_cli_args without a panic
    let options = parse_cli_args();

    match compile_template(options) {
        Ok(runtime) => {
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

            // template.write_compiled_template(&options, &template);
            // writer.write(&options, &template)
        },
        Err(error) => {
            // TODO more extensive stuff if in verbose mode?
            eprintln!("{}", error.to_string());
            process::exit(1)
        }
    }
}
