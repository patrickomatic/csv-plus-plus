use super::ModulePath;
use std::fmt;

impl fmt::Display for ModulePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_one() {
        assert_eq!(ModulePath(vec!["foo".to_string()]).to_string(), "foo")
    }

    #[test]
    fn display_multiple() {
        assert_eq!(
            ModulePath(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string()
            ])
            .to_string(),
            "foo/bar/baz"
        )
    }
}
