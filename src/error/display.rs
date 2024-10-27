use super::Error;
use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CellSyntaxError {
                filename,
                parse_error,
                address,
            } => {
                writeln!(f, "Syntax error in cell {address} of {filename:?}",)?;
                writeln!(f, "{parse_error}")
            }

            Self::CodeSyntaxError {
                parse_error,
                filename,
            } => {
                writeln!(f, "Syntax error in code section of {filename:?}")?;
                writeln!(f, "{parse_error}")
            }

            Self::CsvParseError {
                parse_error,
                filename,
            } => {
                writeln!(f, "Syntax error in CSV section of {filename:?}")?;
                writeln!(f, "{parse_error}")
            }

            Self::EvalError {
                eval_error,
                address,
                filename,
            } => {
                if let Some(address) = address {
                    writeln!(
                        f,
                        "Error evaluating formula in cell {address} ({}, {}) of {filename:?}",
                        address.column.x, address.row.y,
                    )?;
                } else {
                    writeln!(f, "Error evaluating formula in {filename:?}")?;
                }
                writeln!(f, "{eval_error}")
            }

            Self::GoogleSetupError(message) => {
                // TODO: move this message into a template and use `include!`
                // TODO: make this a little smarter, if gcloud isn't in path complain about that?
                // TODO: can we run gcloud auth login automatically?
                // TODO: gcloud ... --update-adc is deprecated.  this gets us close:
                // $ gcloud auth application-default login --scopes openid,https://www.googleapis.com/auth/userinfo.email,https://www.googleapis.com/auth/cloud-platform,https://www.googleapis.com/auth/appengine.admin,https://www.googleapis.com/auth/sqlservice.login,https://www.googleapis.com/auth/compute,https://www.googleapis.com/auth/accounts.reauth,https://www.googleapis.com/auth/drive
                writeln!(
                    f,
                    "Unable to access the specified spreadsheet on Google Sheets.

Are you sure that you have the `gcloud` CLI tools installed and properly configured? To 
authenticate using your Google user account try running:

$ gcloud init
$ gcloud auth login --enable-gdrive-access --update-adc

If you would like to specify service credentials or the path to your own user credentials, call 
csv++ with `GOOGLE_APPLICATION_CREDENTIALS` or the `--google-account-credentials` flag.

{message}"
                )
            }

            Self::InitError(message) | Self::ModuleLoadError(message) => {
                writeln!(f, "{message}")
            }

            Self::ModuleLoadErrors(errors) => {
                for (m, e) in errors {
                    writeln!(f, "Error loading module {m}")?;
                    writeln!(f, "{e}")?;
                }
                Ok(())
            }

            Self::SourceCodeError { filename, message } => {
                writeln!(f, "Error reading source {filename:?}")?;
                writeln!(f, "{message}")
            }

            Self::TargetWriteError { output, message } => {
                writeln!(f, "Error writing to \"{output}\"")?;
                writeln!(f, "{message}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{EvalError, Output, ParseError};
    use super::*;
    use std::path;

    fn build_parse_error() -> ParseError {
        ParseError {
            bad_input: "bar".into(),
            message: "it should be foo".into(),
            position: (5, 3).into(),
            possible_values: None,
            highlighted_lines: vec!["foo".into(), "bar".into(), "baz".into()],
        }
    }

    #[test]
    fn display_cell_syntax_error() {
        let message = Error::CellSyntaxError {
            filename: path::PathBuf::from("a_file.csvpp"),
            address: (1, 5).into(),
            parse_error: Box::new(build_parse_error()),
        };

        assert_eq!(
            message.to_string(),
            "Syntax error in cell B6 of \"a_file.csvpp\"
On line 4 it should be foo but saw bar

foo
bar
baz

",
        );
    }

    #[test]
    fn display_code_syntax_error() {
        let message = Error::CodeSyntaxError {
            filename: path::PathBuf::from("a_file.csvpp"),
            parse_error: Box::new(build_parse_error()),
        };

        assert_eq!(
            message.to_string(),
            "Syntax error in code section of \"a_file.csvpp\"
On line 4 it should be foo but saw bar

foo
bar
baz

"
        );
    }

    #[test]
    fn display_eval_error() {
        let message = Error::EvalError {
            filename: path::PathBuf::from("a_file.csvpp"),
            eval_error: Box::new(EvalError {
                message: "Error".to_string(),
                bad_input: "foo".to_string(),
            }),
            address: Some((2, 2).into()),
        };

        assert_eq!(
            message.to_string(),
            "Error evaluating formula in cell C3 (2, 2) of \"a_file.csvpp\"
Error: foo
"
        );
    }

    #[test]
    fn display_init_error() {
        let message = Error::InitError("foo".to_string());

        assert_eq!("foo\n", message.to_string());
    }

    #[test]
    fn display_source_code_error() {
        let message = Error::SourceCodeError {
            filename: path::PathBuf::from("a_file.csvpp"),
            message: "foo".to_string(),
        };

        assert_eq!(
            message.to_string(),
            "Error reading source \"a_file.csvpp\"\nfoo\n",
        );
    }

    #[test]
    fn display_target_write_error() {
        let message = Error::TargetWriteError {
            output: Output::Excel(path::PathBuf::from("foo.xlsx")),
            message: "foo".to_string(),
        };

        assert_eq!(message.to_string(), "Error writing to \"foo.xlsx\"\nfoo\n",);
    }
}
