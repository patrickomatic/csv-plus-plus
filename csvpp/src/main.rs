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

pub type Position = (usize, usize);

fn main() {
    let options = options::parse_cli_args();
    if let Ok(template) = compiler::compile_template(&options) {
        if options.backup {
            println!("Backing up {}", options.backup)
            // XXX back it up
        }

        if options.verbose {
            println!(
r#"
# csv++ 

## Called with options
{}

## Parsed csvpp template
{}
"#, 
                options,
                template,
            );
        }
    } else {
        // TODO actually catch the error and handle
        println!("Error parsing template!!");
    }
}
