use super::Config;
use std::collections;

impl Default for Config {
    fn default() -> Self {
        Self {
            backup: false,
            google_account_credentials: None,
            key_values: collections::HashMap::new(),
            offset: (0, 0),
            overwrite_values: true,
            sheet_name: "empty".to_string(),
            use_cache: true,
            verbosity: log::LevelFilter::Info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let options = Config::default();

        assert!(!options.backup);
        assert!(options.overwrite_values);
        assert_eq!(options.google_account_credentials, None);
        assert_eq!(options.key_values, collections::HashMap::new());
        assert_eq!(options.offset, (0, 0));
        assert_eq!(options.verbosity, log::LevelFilter::Info);
    }
}
