//! # ModuleLoader
//!
//! A multithreaded module loader that will resursively load the dependencies for a given
//! `Scope`.
//!
// TODO:
// * make it so that `---` is not required
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

type LoadedModules = collections::HashMap<ModulePath, Dependency>;

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

macro_rules! merge {
    ($scope:expr, $dep:expr, $functions_or_variables:ident) => {
        for (var_name, ast) in $dep.scope.$functions_or_variables.clone().into_iter() {
            $scope.$functions_or_variables.insert(
                var_name,
                ast.eval(&$dep.scope, None)
                    .map_err(|e| $dep.source_code.eval_error(e, None))?,
            );
        }
    };
}

macro_rules! merge_scopes {
    ($scope:expr, $dep:expr) => {
        merge!($scope, $dep, variables);
        merge!($scope, $dep, functions);
    };
}

// TODO:
// * get rid of unwrap()s
// * see if I can reduce the clone()s
impl<'a> ModuleLoader<'a> {
    /// Recursively load the dependencies from the given `scope` while collecting any errors into
    /// `failed` and sucesses into `loaded`. The idea being that we want to show as many errors as
    /// possible to the user (otherwise it's annoying to have them fix and re-compile one-by-one),
    /// so we accumulate and keep going.  But in the end fail if there are any errors at all.
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

    /// Returns only the direct dependencies for this module graph.  For example if our Module A
    /// requires Module B which in turn requires Module C, we will only get vars & functions from
    /// Module B, not from Module C (or any other indirect dependencies)
    pub(super) fn into_direct_dependencies(self) -> Result<Scope> {
        if self.has_failures() {
            let failed = sync::Arc::try_unwrap(self.failed).unwrap().into_inner()?;
            Err(Error::ModuleLoadErrors(failed))
        } else {
            self.direct_dependencies()
        }
    }

    /// Extract all direct dependencies on `scope`.  
    fn direct_dependencies(self) -> Result<Scope> {
        let loaded = sync::Arc::try_unwrap(self.loaded).unwrap().into_inner()?;
        // TODO: this whole thing could probably be cleaned up if we built the adjacency list in an
        // array and managed it ourselves and got rid of the `nodes` HashMap
        let mut nodes = collections::HashMap::new();

        let mut dep_graph: graph::Graph<_, ()> = graph::Graph::new();
        let main_node = dep_graph.add_node(self.main_module_path);
        nodes.insert(self.main_module_path, main_node);

        // load all of the direct dependencies
        for p in &self.main_scope.required_modules {
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

        for dep in resolution_order.into_iter() {
            match dep.relation {
                DependencyRelation::Direct => {
                    // for direct dependencies, we want the names to be exposed to the module
                    // requiring them.  so merge into `dep_scope` instead of `tmp_scope` (which we'll
                    // be abandoning after this function runs)
                    merge_scopes!(dep_scope, dep);
                }

                DependencyRelation::Transitive => {
                    // build up transitive dependencies in a global "tmp" namespace, but we'll *not* be
                    // exposing this namespace to the main module since it should only get the direct
                    // dependencies
                    if dep.scope.required_modules.is_empty() {
                        tmp_scope
                            .functions
                            .extend(dep.scope.functions.clone().into_iter());
                        tmp_scope
                            .variables
                            .extend(dep.scope.variables.clone().into_iter());
                    } else {
                        merge_scopes!(tmp_scope, dep);
                    }
                }
            }
        }

        Ok(dep_scope)
    }

    fn has_failures(&self) -> bool {
        !self.failed.try_read().unwrap().is_empty()
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
        let p: path::PathBuf = module_path.clone().into();

        // load the source code
        let source_code = match SourceCode::open(&p) {
            Ok(s) => ArcSourceCode::new(s),
            Err(e) => {
                self.failed.write()?.insert(module_path, e);
                return Ok(());
            }
        };

        // parse the code section
        if let Some(scope_source) = &source_code.code_section {
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
                            source_code: source_code.clone(),
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
                Error::ModuleLoadError(
                    "This module does not have a code section (but you imported it)".to_string(),
                ),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn load_main_empty() {
        let module_path = ModulePath::new("foo");

        assert!(ModuleLoader::load_main(&module_path, &Scope::default()).is_ok());
    }

    #[test]
    fn load_main_require_does_not_exist() {
        let module_path = ModulePath::new("foo");
        let scope = Scope {
            required_modules: vec![module_path.clone()],
            ..Default::default()
        };
        let module_loader = ModuleLoader::load_main(&module_path, &scope).unwrap();

        assert_eq!(module_loader.failed.read().unwrap().len(), 1);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 1);
        assert_eq!(module_loader.loaded.read().unwrap().len(), 0);
    }

    #[test]
    fn load_main_valid_files() {
        let mod1 = TestFile::new(
            "csvpp",
            "
a := 42
---
        ",
        );
        let mod2 = TestFile::new(
            "csvpp",
            "
b := 24
---
        ",
        );
        let mod1_path: ModulePath = (&mod1).into();
        let mod2_path: ModulePath = (&mod2).into();
        let scope = Scope {
            required_modules: vec![mod1_path.clone(), mod2_path.clone()],
            ..Default::default()
        };
        let module_path = ModulePath::new("main");
        let module_loader = ModuleLoader::load_main(&module_path, &scope).unwrap();
        let loaded = module_loader.loaded.read().unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 2);
        assert_eq!(module_loader.failed.read().unwrap().len(), 0);
        assert_eq!(
            loaded
                .get(&mod1_path)
                .unwrap()
                .scope
                .variables
                .get("a")
                .unwrap(),
            &Ast::new(Node::var("a", VariableValue::Ast(42.into()))),
        );
        assert_eq!(
            loaded
                .get(&mod2_path)
                .unwrap()
                .scope
                .variables
                .get("b")
                .unwrap(),
            &Ast::new(Node::var("b", VariableValue::Ast(24.into()))),
        );
    }

    #[test]
    fn load_in_directory() {
        let module = TestFile::new_in_dir(
            "csvpp",
            "
a := 42
---
        ",
        );
        let module_path: ModulePath = (&module).into();
        let scope = Scope {
            required_modules: vec![module_path.clone()],
            ..Default::default()
        };
        let module_path = ModulePath::new("main");
        let module_loader = ModuleLoader::load_main(&module_path, &scope).unwrap();

        assert_eq!(module_loader.loaded.read().unwrap().len(), 1);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 1);
        assert_eq!(module_loader.failed.read().unwrap().len(), 0);
    }

    #[test]
    fn load_main_double_load() {
        let mod1 = TestFile::new(
            "csvpp",
            "
a := 42
---
        ",
        );
        let mod1_path: ModulePath = (&mod1).into();
        let mod2 = TestFile::new(
            "csvpp",
            &format!(
                "
use {mod1_path}
b := 24
---
        "
            ),
        );
        let mod2_path: ModulePath = (&mod2).into();
        let scope = Scope {
            required_modules: vec![mod1_path.clone(), mod2_path.clone()],
            ..Default::default()
        };
        let module_path = ModulePath::new("main");
        let module_loader = ModuleLoader::load_main(&module_path, &scope).unwrap();
        let loaded = module_loader.loaded.read().unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 2);
        assert_eq!(module_loader.failed.read().unwrap().len(), 0);
    }

    /*
    #[test]
    fn into_direct_dependencies_empty() {
        let scope = Scope::default();
        let module_path = ModulePath(vec!["main".to_string()]);
        let loaded = collections::HashMap::new();
        let deps = direct_dependencies(&module_path, &scope, loaded);

        assert!(deps.functions.is_empty());
        assert!(deps.variables.is_empty());
    }

    #[test]
    fn into_direct_dependencies_dag() {
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

        assert!(direct_dependencies(&module_path, &scope, loaded).is_ok());
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
    */
}
