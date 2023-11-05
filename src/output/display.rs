use super::Output;
use std::fmt;

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GoogleSheets(id) => write!(f, "https://docs.google.com/spreadsheets/d/{id}"),
            Self::Csv(path) | Self::Excel(path) | Self::OpenDocument(path) => {
                write!(f, "{}", path.display())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn display_link() {
        assert_eq!(
            Output::GoogleSheets("sheet-id".to_string()).to_string(),
            "https://docs.google.com/spreadsheets/d/sheet-id"
        );
    }

    #[test]
    fn display_path() {
        assert_eq!(
            Output::Csv(path::Path::new("foo.csv").to_path_buf()).to_string(),
            "foo.csv"
        );
        assert_eq!(
            Output::Excel(path::Path::new("foo.xlsx").to_path_buf()).to_string(),
            "foo.xlsx"
        );
        assert_eq!(
            Output::OpenDocument(path::Path::new("foo.ods").to_path_buf()).to_string(),
            "foo.ods"
        );
    }
}
