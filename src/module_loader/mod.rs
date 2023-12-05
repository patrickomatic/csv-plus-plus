//! # ModuleLoader
//!
//! A multithreaded module loader that will resursively load the dependencies for a given
//! `Scope`.
//!
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{ArcSourceCode, Error, ModulePath, Result, Scope, SourceCode};
use petgraph::graph;
use std::collections;
use std::path;
use std::sync;
use std::thread;

mod dependency;
use dependency::{Dependency, DependencyRelation};

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

/// Extract all direct dependencies on `scope`.  
fn direct_dependencies(module_path: &ModulePath, scope: &Scope, loaded: LoadedModules) -> Scope {
    let mut nodes = collections::HashMap::new();

    let mut dep_graph: graph::Graph<_, ()> = graph::Graph::new();
    let main_node = dep_graph.add_node(module_path);
    nodes.insert(module_path, main_node);

    // load all of the direct dependencies
    for p in &scope.required_modules {
        let n = dep_graph.add_node(p);
        nodes.insert(p, n);
        dep_graph.add_edge(main_node, n, ());
    }

    // and now all of the transitive dependencies (that have already been flattened out into a
    // HashMap).  this code is a little awkward because we need to consult with `added` to see
    // if we've already added the node
    // TODO: consume loaded so we don't have to clone below
    for (p, dep) in &loaded {
        let path_node = if let Some(pn) = nodes.get(p) {
            *pn
        } else {
            let pn = dep_graph.add_node(p);
            nodes.insert(p, pn);
            pn
        };

        for dep in &dep.scope.required_modules {
            let n = nodes.entry(dep).or_insert_with(|| dep_graph.add_node(dep));
            dep_graph.add_edge(path_node, *n, ());
        }
    }

    // now that we have a graph, use Tarjan's algo to give us a topological sort which will
    // represent the dependencies in the order they need to be resolved.
    let resolution_order = petgraph::algo::tarjan_scc(&dep_graph)
        .into_iter()
        .flatten()
        .filter_map(|n| loaded.get(dep_graph[n]));

    let mut dep_scope = Scope::default();
    let mut tmp_scope = Scope::default();

    for dep in resolution_order {
        match dep.relation {
            DependencyRelation::Direct => {
                // we're on the last one -
                for (var_name, ast) in dep.scope.variables.clone().into_iter() {
                    // XXX eval it
                    dep_scope.variables.insert(var_name, ast);
                }
                for (fn_name, ast) in dep.scope.functions.clone().into_iter() {
                    // XXX eval it
                    dep_scope.functions.insert(fn_name, ast);
                }
            }

            DependencyRelation::Transitive => {
                // TODO: we could optimize this by not evaling if required_modules.is_empty()
                for (var_name, ast) in dep.scope.variables.clone().into_iter() {
                    // XXX eval it
                    tmp_scope.variables.insert(var_name, ast);
                }
                for (fn_name, ast) in dep.scope.functions.clone().into_iter() {
                    // XXX eval it
                    tmp_scope.functions.insert(fn_name, ast);
                }
            }
        }
    }

    dep_scope
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

    fn load(&self, scope: &Scope, dependency_relation: DependencyRelation) -> Result<()> {
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
                    // recursively load the newly loaded code section's dependencies (which are
                    // transitive at this point)
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
    pub(super) fn into_direct_dependencies(self) -> Result<Scope> {
        // TODO: get rid of the unwraps
        let failed = sync::Arc::try_unwrap(self.failed).unwrap().into_inner()?;

        if failed.is_empty() {
            let loaded = sync::Arc::try_unwrap(self.loaded).unwrap().into_inner()?;
            Ok(direct_dependencies(
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

    #[test]
    fn direct_dependencies_empty() {
        let scope = Scope::default();
        let module_path = ModulePath(vec!["main".to_string()]);
        let loaded = collections::HashMap::new();
        let deps = direct_dependencies(&module_path, &scope, loaded);

        assert!(deps.functions.is_empty());
        assert!(deps.variables.is_empty());
    }

    #[test]
    fn direct_dependencies_dag() {
        // main -> a -> b -> c
        let scope = Scope {
            required_modules: vec![ModulePath(vec!["a".to_string()])],
            ..Default::default()
        };
        let module_path = ModulePath(vec!["main".to_string()]);
        let mut loaded = collections::HashMap::new();
        loaded.insert(
            ModulePath(vec!["a".to_string()]),
            Dependency::direct(Scope {
                required_modules: vec![ModulePath(vec!["b".to_string()])],
                ..Default::default()
            }),
        );
        loaded.insert(
            ModulePath(vec!["b".to_string()]),
            Dependency::transitive(Scope {
                required_modules: vec![ModulePath(vec!["c".to_string()])],
                ..Default::default()
            }),
        );
        loaded.insert(
            ModulePath(vec!["c".to_string()]),
            Dependency::transitive(Scope {
                required_modules: vec![],
                ..Default::default()
            }),
        );

        // assert!(
        // direct_dependencies(&module_path, &scope, loaded).is_ok()
        // );
    }

    #[test]
    fn direct_dependencies_cyclic() {
        // XXX
        let scope = Scope::default();
        let module_path = ModulePath(vec!["main".to_string()]);
        // let loaded = collections::HashMap::new();

        // assert!(
        // direct_dependencies(&module_path, &scope, loaded).is_ok()
        // );
    }
}
