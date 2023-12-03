//! # ModuleLoader
//!
//! A multithreaded module loader that will resursively load the dependencies for a given
//! `CodeSection`.
//!
use super::ModuleName;
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{ArcSourceCode, CodeSection, Error, Result, SourceCode};
use std::collections;
use std::path;
use std::sync;
use std::thread;

type ArcRwLock<T> = sync::Arc<sync::RwLock<T>>;

type Attempted = ArcRwLock<collections::HashSet<ModuleName>>;
type Loaded = ArcRwLock<collections::HashMap<ModuleName, CodeSection>>;
type Failed = ArcRwLock<collections::HashMap<ModuleName, Error>>;

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
        // only try the ones which we haven't.  it's possible another module could have already
        // loaded the ones we want
        let mut to_attempt = vec![];
        let attempted_borrow = sync::Arc::clone(&self.attempted);
        let mut attempted = attempted_borrow.write()?;
        for module_name in &code_section.required_modules {
            if attempted.contains(module_name) {
                continue;
            } else {
                // insert into `attempted` pre-emptively so none of the other threads attempt it
                attempted.insert(module_name.clone());
                to_attempt.push(module_name.clone());
            }
        }
        drop(attempted);

        thread::scope(|s| {
            for module_name in to_attempt {
                s.spawn(|| self.load_module(module_name));
            }
        });

        Ok(())
    }

    fn load_module(&self, module_name: ModuleName) -> Result<()> {
        let p: path::PathBuf = module_name.clone().into();

        let source_code = match SourceCode::open(&p) {
            Ok(s) => ArcSourceCode::new(s),
            Err(e) => {
                let mut failed = self.failed.write()?;
                failed.insert(module_name, e);
                return Ok(());
            }
        };

        if let Some(code_section_source) = &source_code.code_section {
            match CodeSectionParser::parse(code_section_source, source_code.clone()) {
                Ok(cs) => {
                    // recursively load the newly loaded code section's dependencies
                    self.load(&cs)?;

                    let mut loaded = self.loaded.write()?;
                    loaded.insert(module_name, cs);
                }
                Err(e) => {
                    let mut failed = self.failed.write()?;
                    failed.insert(module_name, e);
                }
            }
        } else {
            let mut failed = self.failed.write()?;
            failed.insert(
                module_name.clone(),
                Error::ModuleLoadError("This module does not have a code section".to_string()),
            );
        }

        Ok(())
    }

    pub(super) fn into_modules_loaded(
        self,
    ) -> Result<collections::HashMap<ModuleName, CodeSection>> {
        // TODO: get rid of the unwraps
        let failed = sync::Arc::try_unwrap(self.failed).unwrap().into_inner()?;

        if failed.is_empty() {
            Ok(sync::Arc::try_unwrap(self.loaded).unwrap().into_inner()?)
        } else {
            Err(Error::ModuleLoadErrors(failed))
        }
    }
}
