//! # ModuleLoader
//!
//! A multithreaded module loader that will resursively load the dependencies for a given
//! `Scope`.
//!
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{ArcSourceCode, Scope, Error, ModulePath, Result, SourceCode};
use std::collections;
use std::path;
use std::sync;
use std::thread;

mod dependency;
use dependency::{Dependency, DependencyRelation};

mod module_dependencies;
pub(super) use module_dependencies::ModuleDependencies;

type ArcRwLock<T> = sync::Arc<sync::RwLock<T>>;

pub(super) type LoadedModules = collections::HashMap<ModulePath, Dependency>;

type Attempted = ArcRwLock<collections::HashSet<ModulePath>>;
type Loaded = ArcRwLock<LoadedModules>;
type Failed = ArcRwLock<collections::HashMap<ModulePath, Error>>;

#[derive(Debug)]
pub(super) struct ModuleLoader<'a> {
    main_scope: &'a Scope,
    main_module_path: &'a ModulePath,
    attempted: Attempted,
    loaded: Loaded,
    failed: Failed,
}

impl<'a> ModuleLoader<'a> {
    /// Recursively load the dependencies from the given `scope`. This function does not
    /// return any `Result` and instead collects errors into `failed` and successes into `loaded`.
    /// The idea being that we want to show as many errors as possible to the user (otherwise it's
    /// annoying to have them fix and re-compile one-by-one), so we accumulate and keep going.  But
    /// in the end fail if there are any errors at all.
    pub(super) fn load_main(
        module_path: &'a ModulePath,
        scope: &'a Scope,
    ) -> Result<ModuleLoader<'a>> {
        let module_loader = Self {
            main_scope: scope,
            main_module_path: module_path,
            attempted: Default::default(),
            loaded: Default::default(),
            failed: Default::default(),
        };
        module_loader.load(scope, DependencyRelation::Direct)?;

        Ok(module_loader)
    }

    fn load(
        &self,
        scope: &Scope,
        dependency_relation: DependencyRelation,
    ) -> Result<()> {
        let mut to_attempt = vec![];
        // hold a lock while we reserve all of the dependencies we're going to resolve (by
        // preemptively marking them in `attempted`)
        {
            let mut attempted = self.attempted.write()?;
            for module_path in &scope.required_modules {
                if attempted.contains(module_path) {
                    // another modules has already loaded it
                    continue;
                } else {
                    attempted.insert(module_path.clone());
                    to_attempt.push(module_path.clone());
                }
            }
        }

        // now a thread for each module to load, and they'll recurse back to this function if they
        // in turn have modules to load
        thread::scope(|s| {
            for module_path in to_attempt {
                s.spawn(|| self.load_module(module_path, dependency_relation));
            }
        });

        Ok(())
    }

    fn load_module(
        &self,
        module_path: ModulePath,
        dependency_relation: DependencyRelation,
    ) -> Result<()> {
        // TODO: I think each thread needs to cd into the directory...
        let p: path::PathBuf = module_path.clone().into();

        // load the source code
        let source_code = match SourceCode::open(&p) {
            Ok(s) => ArcSourceCode::new(s),
            Err(e) => {
                self.failed.write()?.insert(module_path, e);
                return Ok(());
            }
        };

        // parse the code section out
        if let Some(scope_source) = &source_code.scope {
            // TODO: this should use the csvpo cache if there is one
            match CodeSectionParser::parse(scope_source, source_code.clone()) {
                Ok(cs) => {
                    // recursively load the newly loaded code section's dependencies
                    self.load(&cs, DependencyRelation::Transitive)?;

                    self.loaded.write()?.insert(
                        module_path,
                        Dependency {
                            relation: dependency_relation,
                            scope: cs,
                        },
                    );
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

    /// Returns only the direct dependencies for this module graph.  For example if our Module A
    /// requires Module B which in turn requires Module C, we will only get vars & functions from
    /// Module B, not from Module C (or any other indirect dependencies)
    pub(super) fn into_dependencies(self) -> Result<ModuleDependencies> {
        // TODO: get rid of the unwraps
        let failed = sync::Arc::try_unwrap(self.failed).unwrap().into_inner()?;

        if failed.is_empty() {
            let loaded = sync::Arc::try_unwrap(self.loaded).unwrap().into_inner()?;
            Ok(ModuleDependencies::direct_dependencies(
                self.main_module_path,
                self.main_scope,
                loaded,
            ))
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
    fn load_main_empty() {
        let module_path = ModulePath(vec!["foo".to_string()]);
        assert!(ModuleLoader::load_main(&module_path, &Scope::default()).is_ok());
    }

    #[test]
    fn load_main_multiple() {
        let module_path = ModulePath(vec!["foo".to_string()]);
        let scope = Scope {
            required_modules: vec![module_path.clone()],
            ..Default::default()
        };
        assert!(ModuleLoader::load_main(&module_path, &scope).is_ok());
    }

    #[test]
    fn load_main_circular() {
        // XXX
    }

    #[test]
    fn into_dependencies() {
        // XXX
    }

    #[test]
    fn into_dependencies_failed_not_empty() {
        // XXX
    }
}
