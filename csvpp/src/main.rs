//! # csv+++
//!
//! # Examples
//!
//! ```
//! ```
//! 
mod ast;
mod compiler;
mod error;
mod modifier;
mod options;
mod runtime;
mod source_code;

fn main() {
    let options = options::parse_cli_args();
    let template = compiler::compile_template(&options);

    if options.backup {
        println!("Backing up {}", options.backup)
        // XXX back it up
    }

    if options.verbose {
        println!(
r#"
# Parsed csvpp template
{}
"#, 
            options.input
        );
    }
}
