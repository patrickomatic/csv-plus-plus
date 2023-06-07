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
mod rgb;
mod runtime;
mod source_code;

#[derive(Clone, Debug, PartialEq)]
pub struct Position(usize, usize);

impl Position {
    pub fn is_first_cell(&self) -> bool {
        self.0 == 0
    }
}

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
