//! # ModuleLoader
//!
//! A multithreaded module loader that will resursively load the dependencies for a given
//! `CodeSection`.
//!
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{ArcSourceCode, CodeSection, Error, ModulePath, Result, SourceCode};
use std::collections;
use std::path;
use std::sync;
use std::thread;

type ArcRwLock<T> = sync::Arc<sync::RwLock<T>>;

type Attempted = ArcRwLock<collections::HashSet<ModulePath>>;
type Loaded = ArcRwLock<collections::HashMap<ModulePath, CodeSection>>;
type Failed = ArcRwLock<collections::HashMap<ModulePath, Error>>;

#[derive(Debug, Default)]
pub(super) struct ModuleLoader {
    attempted: Attempted,
    loaded: Loaded,
    failed: Failed,
}

impl ModuleLoader {
    /// Recursively load the dependencies from the given `code_section`. This function does not
    /// return any `Result` and instead collects errors into `failed` and successes into `loaded`.
    /// The idea being that we want to show as many errors as possible to the user (otherwise it's
    /// annoying to have them fix and re-compile one-by-one), so we accumulate and keep going.  But
    /// in the end fail if there are any errors at all.
    pub(super) fn load(&self, code_section: &CodeSection) -> Result<()> {
        let mut to_attempt = vec![];
        // hold a lock while we reserve all of the dependencies we're going to resolve (by marking
        // them in `attempted`)
        {
            let mut attempted = self.attempted.write()?;
            for module_path in &code_section.required_modules {
                if attempted.contains(module_path) {
                    // another modules has already loaded it
                    continue;
                } else {
                    attempted.insert(module_path.clone());
                    to_attempt.push(module_path.clone());
                }
            }
        }

        thread::scope(|s| {
            for module_path in to_attempt {
                s.spawn(|| self.load_module(module_path));
            }
        });

        Ok(())
    }

    fn load_module(&self, module_path: ModulePath) -> Result<()> {
        let p: path::PathBuf = module_path.clone().into();

        let source_code = match SourceCode::open(&p) {
            Ok(s) => ArcSourceCode::new(s),
            Err(e) => {
                self.failed.write()?.insert(module_path, e);
                return Ok(());
            }
        };

        if let Some(code_section_source) = &source_code.code_section {
            match CodeSectionParser::parse(code_section_source, source_code.clone()) {
                Ok(cs) => {
                    // recursively load the newly loaded code section's dependencies
                    self.load(&cs)?;
                    self.loaded.write()?.insert(module_path, cs);
                }
                Err(e) => {
                    self.failed.write()?.insert(module_path, e);
                }
            }
        } else {
            self.failed.write()?.insert(
                module_path.clone(),
                Error::ModuleLoadError("This module does not have a code section".to_string()),
            );
        }

        Ok(())
    }

    pub(super) fn into_modules_loaded(
        self,
    ) -> Result<collections::HashMap<ModulePath, CodeSection>> {
        // TODO: get rid of the unwraps
        let failed = sync::Arc::try_unwrap(self.failed).unwrap().into_inner()?;

        if failed.is_empty() {
            Ok(sync::Arc::try_unwrap(self.loaded).unwrap().into_inner()?)
        } else {
            Err(Error::ModuleLoadErrors(failed))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn load_empty() {
        assert!(ModuleLoader::default()
            .load(&CodeSection::default())
            .is_ok());
    }

    #[test]
    fn load_multiple() {
        let code_section = CodeSection {
            required_modules: vec![ModulePath(vec!["foo".to_string()])],
            ..Default::default()
        };
        assert!(ModuleLoader::default().load(&code_section).is_ok());
    }

    #[test]
    fn load_circular() {
        // XXX
    }

    #[test]
    fn into_modules_loaded_failed_empty() {
        // XXX
    }

    #[test]
    fn into_modules_loaded_failed_not_empty() {
        // XXX
    }
}
