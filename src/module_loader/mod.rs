//! # ModuleLoader
//!
//! A multithreaded module loader that will resursively load the dependencies for a given
//! `Scope`.
//!
// TODO:
// * make it so that `---` is not required
use crate::{compiler_error, Error, Module, ModulePath, Result, Scope};
use log::{debug, info};
use petgraph::{algo, graph, stable_graph};
use std::collections;
use std::path;
use std::sync;
use std::thread;

type ArcRwLock<T> = sync::Arc<sync::RwLock<T>>;

type LoadedModules = collections::HashMap<ModulePath, Module>;

type Attempted = ArcRwLock<collections::HashSet<ModulePath>>;
type Loaded = ArcRwLock<LoadedModules>;
type Failed = ArcRwLock<collections::HashMap<ModulePath, Error>>;

#[derive(Debug)]
pub(super) struct ModuleLoader<'a> {
    attempted: Attempted,
    failed: Failed,
    loaded: Loaded,
    loader_root: path::PathBuf,
    main_module: &'a Module,
    is_dirty: bool,
    // TODO: this is going to need to respect the --use-cache flag
    // use_cache: bool,
}

// TODO: ideally this shouldn't take a $source_code and the calling part does the map_err
macro_rules! eval_fns_or_vars {
    ($scope:ident, $functions_or_variables:ident, $source_code:expr) => {{
        for (name, ast) in $scope.$functions_or_variables.clone().into_iter() {
            $scope.$functions_or_variables.insert(
                name,
                ast.eval(&$scope, None)
                    .map_err(|e| $source_code.eval_error(e, None))?,
            );
        }
    }};
}

macro_rules! eval_scope {
    ($scope:ident, $source_code:expr) => {
        eval_fns_or_vars!($scope, variables, $source_code);
        eval_fns_or_vars!($scope, functions, $source_code);
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
    pub(super) fn load_dependencies<P: Into<path::PathBuf>>(
        module: &'a Module,
        relative_to: P,
    ) -> Result<ModuleLoader<'a>> {
        let mut module_loader = Self {
            attempted: Default::default(),
            failed: Default::default(),
            loaded: Default::default(),
            main_module: module,
            loader_root: relative_to.into(),
            is_dirty: false,
        };

        // we need to do this in a loop because every time we reload dirty dependencies we're
        // pulling in changes to the source code, which could mean the author added a new `use ...`
        // in which case we need to load it
        loop {
            module_loader.load(module)?;
            module_loader = module_loader.propagate_dirty_flag()?;
            if module_loader.is_dirty {
                module_loader.reload_dirty_modules()?;
            } else {
                break;
            }
        }

        Ok(module_loader)
    }

    /// Returns only the direct dependencies for this module graph.  For example if our Module A
    /// requires Module B which in turn requires Module C, we will only get vars & functions from
    /// Module B, not from Module C (or any other indirect dependencies)
    pub(super) fn into_direct_dependencies(self) -> Result<Scope> {
        if self.has_failures() {
            Err(Error::ModuleLoadErrors(
                sync::Arc::try_unwrap(self.failed).unwrap().into_inner()?,
            ))
        } else {
            self.direct_dependencies()
        }
    }

    fn load_dependency_graph(&self) -> stable_graph::StableGraph<ModulePath, ()> {
        let loaded = self.loaded.read().unwrap();
        info!(
            "Creating dependency graph with {} dependencies",
            loaded.len()
        );

        let mut dep_graph: stable_graph::StableGraph<_, ()> = stable_graph::StableGraph::new();

        let main_node = dep_graph.add_node(self.main_module.module_path.clone());

        // load all of the direct dependencies
        for p in &self.main_module.required_modules {
            let direct_dep_node = dep_graph.add_node(p.clone());
            dep_graph.add_edge(main_node, direct_dep_node, ());
        }

        for (p, dep_mod) in loaded.iter() {
            let dep_node = dep_graph.add_node(p.clone());

            for required_mod in &dep_mod.required_modules {
                let dep_dep_node = dep_graph.add_node(required_mod.clone());
                dep_graph.add_edge(dep_node, dep_dep_node, ());
            }
        }

        debug!("Loaded dependency graph {dep_graph:?}");

        dep_graph
    }

    fn dirty_nodes(&self) -> collections::HashSet<ModulePath> {
        let loaded = self.loaded.read().unwrap();
        let dep_graph = self.load_dependency_graph();

        let mut dirty_nodes: collections::HashSet<ModulePath> = Default::default();
        for node in dep_graph.node_indices().collect::<Vec<_>>() {
            let Some(module) = loaded.get(&dep_graph[node]) else {
                continue;
            };

            if module.is_dirty {
                for graph_path in algo::simple_paths::all_simple_paths::<Vec<_>, _>(
                    &dep_graph,
                    graph::NodeIndex::new(0),
                    node,
                    1,
                    None,
                ) {
                    for n in graph_path {
                        dirty_nodes.insert(dep_graph[n].clone());
                    }
                }
            }
        }

        dirty_nodes
    }

    fn propagate_dirty_flag(self) -> Result<Self> {
        let dirty_nodes = self.dirty_nodes();
        let mut loaded = sync::Arc::try_unwrap(self.loaded).unwrap().into_inner()?;

        let is_dirty = !dirty_nodes.is_empty();

        for mp in dirty_nodes {
            if let Some(n) = loaded.get_mut(&mp) {
                n.is_dirty = true;
            }
        }

        Ok(Self {
            loaded: sync::Arc::new(sync::RwLock::new(loaded)),
            is_dirty,
            ..self
        })
    }

    fn reload_dirty_modules(&mut self) -> Result<()> {
        let loaded = self.loaded.read().unwrap();
        for (mp, module) in loaded.iter() {
            if module.is_dirty {
                self.loaded.write()?.insert(
                    mp.clone(),
                    Module::load_from_source(mp.clone(), module.source_code.filename.clone())?,
                );
            }
        }

        Ok(())
    }

    /// Extract all direct dependencies on `scope`.  
    fn direct_dependencies(self) -> Result<Scope> {
        let dep_graph = self.load_dependency_graph();
        let loaded = sync::Arc::try_unwrap(self.loaded).unwrap().into_inner()?;

        // now that we have a graph, use Tarjan's algo to give us a topological sort which will
        // represent the dependencies in the order they need to be resolved.
        let resolution_order = algo::tarjan_scc(&dep_graph)
            .into_iter()
            .flatten()
            .filter_map(|p| loaded.get(&dep_graph[p]));

        debug!("Resolving dependencies in order {resolution_order:?}");

        let mut evaled = collections::HashMap::<&ModulePath, Scope>::new();

        for to_resolve in resolution_order.into_iter() {
            let mut local_scope = to_resolve.scope.clone();
            for req_path in to_resolve.required_modules.iter().rev() {
                // look in `evaled` first, then fall back to `loaded`, and otherwise if it's not
                // found it doesn't make sense because we know the module loader loaded it
                let dep_scope = if let Some(s) = evaled.get(req_path) {
                    s.clone()
                } else if let Some(m) = loaded.get(req_path) {
                    m.scope.clone()
                } else {
                    compiler_error(format!("Expected module to have been loaded: {req_path}"))
                };

                // merge the scopes together, but let ours take precedent. because if you
                // define a variable that has the same name as an import, the assumption is
                // you'll be shadowing it
                local_scope = local_scope.merge(dep_scope);
            }

            eval_scope!(local_scope, to_resolve.source_code);

            evaled.insert(&to_resolve.module_path, local_scope);
        }

        let mut resolved_scope = self.main_module.scope.clone();
        for req_path in self.main_module.required_modules.iter() {
            resolved_scope = resolved_scope.merge_into_main(evaled.remove(req_path).unwrap());
        }

        Ok(resolved_scope)
    }

    fn has_failures(&self) -> bool {
        !self.failed.try_read().unwrap().is_empty()
    }

    fn load(&self, module: &Module) -> Result<()> {
        let mut to_attempt = vec![];
        // hold a lock while we reserve all of the dependencies we're going to resolve (by
        // preemptively marking them in `attempted`)
        {
            let mut attempted = self.attempted.write()?;
            for module_path in &module.required_modules {
                if attempted.contains(module_path) {
                    // another module has already loaded it
                    continue;
                } else {
                    attempted.insert(module_path.clone());
                    to_attempt.push(module_path.clone());
                }
            }
        }

        // now a thread for each module to load and they'll recurse back to this function if they
        // in turn have modules to load
        thread::scope(|s| {
            for module_path in to_attempt {
                s.spawn(|| self.load_module(module_path, &module.module_path));
            }
        });

        Ok(())
    }

    fn load_module(&self, module_path: ModulePath, relative_to: &ModulePath) -> Result<()> {
        match Module::load_from_cache_or_source(module_path.clone(), relative_to, &self.loader_root)
        {
            Ok(loaded_module) => {
                // recursively load the newly loaded code section's dependencies (which are
                // transitive at this point)
                self.load(&loaded_module)?;
                self.loaded.write()?.insert(module_path, loaded_module);
            }
            Err(e) => {
                self.failed.write()?.insert(module_path, e);
            }
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
    use std::collections::*;
    use std::sync::*;

    #[test]
    fn load_dependencies_empty() {
        assert!(ModuleLoader::load_dependencies(&build_module(), "").is_ok());
    }

    #[test]
    fn load_dependencies_require_does_not_exist() {
        let mut module = build_module();
        module.required_modules.push(ModulePath::new("bar"));
        let module_loader = ModuleLoader::load_dependencies(&module, "").unwrap();

        assert_eq!(module_loader.failed.read().unwrap().len(), 1);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 1);
        assert_eq!(module_loader.loaded.read().unwrap().len(), 0);
    }

    #[test]
    fn load_dependencies_valid_files() {
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
        let module = Module {
            module_path: ModulePath::new("main"),
            required_modules: vec![mod1_path.clone(), mod2_path.clone()],
            ..build_module()
        };
        let module_loader = ModuleLoader::load_dependencies(&module, "").unwrap();
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
        let dep_mod = TestFile::new_in_dir(
            "csvpp",
            "
a := 42
---
        ",
        );
        let module = Module {
            module_path: ModulePath::new("main"),
            required_modules: vec![(&dep_mod).into()],
            ..build_module()
        };
        let module_loader = ModuleLoader::load_dependencies(&module, "").unwrap();

        assert_eq!(module_loader.loaded.read().unwrap().len(), 1);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 1);
        assert_eq!(module_loader.failed.read().unwrap().len(), 0);
    }

    #[test]
    fn load_dependencies_double_load() {
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

        let module = Module {
            module_path: ModulePath::new("main"),
            required_modules: vec![(&mod1).into(), (&mod2).into()],
            ..build_module()
        };
        let module_loader = ModuleLoader::load_dependencies(&module, "").unwrap();

        assert_eq!(module_loader.loaded.read().unwrap().len(), 2);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 2);
        assert_eq!(module_loader.failed.read().unwrap().len(), 0);
    }

    #[test]
    fn into_direct_dependencies_empty() {
        let module_loader = ModuleLoader {
            main_module: &build_module(),
            attempted: Default::default(),
            loaded: Default::default(),
            failed: Default::default(),
            loader_root: path::Path::new("").to_path_buf(),
            is_dirty: false,
        };

        assert!(module_loader.into_direct_dependencies().is_ok());
    }

    #[test]
    fn into_direct_dependencies_error() {
        let module_loader = ModuleLoader {
            main_module: &build_module(),
            attempted: Default::default(),
            loaded: Default::default(),
            failed: Default::default(),
            loader_root: path::Path::new("").to_path_buf(),
            is_dirty: false,
        };
        module_loader.failed.write().unwrap().insert(
            ModulePath::new("foo"),
            Error::InitError("failed".to_string()),
        );

        assert!(module_loader.into_direct_dependencies().is_err());
    }

    #[test]
    fn into_direct_dependencies_variable() {
        // var_from_a depends on var_from_b
        let mut mod_a = Module {
            module_path: ModulePath::new("a"),
            required_modules: vec![ModulePath::new("b")],
            ..build_module()
        };
        mod_a
            .scope
            .define_variable("var_from_a", Node::reference("var_from_b"));

        // var_from_b depends on var_from_c
        let mut mod_b = Module {
            module_path: ModulePath::new("b"),
            required_modules: vec![ModulePath::new("c")],
            ..build_module()
        };
        mod_b
            .scope
            .define_variable("var_from_b", Node::reference("var_from_c"));

        // var_from_c resolves to 420
        let mut mod_c = Module {
            module_path: ModulePath::new("c"),
            required_modules: vec![],
            ..build_module()
        };
        mod_c
            .scope
            .define_variable("var_from_c", Node::Integer(420));

        let main_module = Module {
            module_path: ModulePath::new("foo"),
            required_modules: vec![ModulePath::new("a")],
            ..build_module()
        };
        let module_loader = ModuleLoader {
            main_module: &main_module,
            attempted: Default::default(),
            loaded: Arc::new(RwLock::new(HashMap::from([
                (ModulePath::new("a"), mod_a),
                (ModulePath::new("b"), mod_b),
                (ModulePath::new("c"), mod_c),
            ]))),
            failed: Default::default(),
            loader_root: path::Path::new("").to_path_buf(),
            is_dirty: false,
        };

        let dependencies = module_loader.into_direct_dependencies().unwrap();

        assert_eq!(
            dependencies.variables.get("var_from_a").unwrap(),
            &Ast::new(420.into())
        );
        assert!(!dependencies.variables.contains_key("var_from_b"));
        assert!(!dependencies.variables.contains_key("var_from_c"));
    }

    #[test]
    fn into_direct_dependencies_function() {
        let mut mod_a = Module {
            module_path: ModulePath::new("a"),
            required_modules: vec![ModulePath::new("b")],
            ..build_module()
        };
        mod_a.scope.define_function(
            "fn_from_a",
            Node::fn_def("fn_from_a", &[], Node::reference("var_from_b")),
        );

        let mut mod_b = Module {
            module_path: ModulePath::new("b"),
            ..build_module()
        };
        mod_b
            .scope
            .define_variable("var_from_b", Node::Integer(420));

        let main_module = Module {
            module_path: ModulePath::new("foo"),
            required_modules: vec![ModulePath::new("a")],
            ..build_module()
        };
        let module_loader = ModuleLoader {
            main_module: &main_module,
            attempted: Default::default(),
            loaded: Arc::new(RwLock::new(HashMap::from([
                (ModulePath::new("a"), mod_a),
                (ModulePath::new("b"), mod_b),
            ]))),
            failed: Default::default(),
            loader_root: path::Path::new("").to_path_buf(),
            is_dirty: false,
        };

        let dependencies = module_loader.into_direct_dependencies().unwrap();

        assert_eq!(
            dependencies.functions.get("fn_from_a").unwrap(),
            &Ast::new(Node::fn_def("fn_from_a", &[], Node::Integer(420)))
        );
    }

    #[test]
    fn into_direct_dependencies_shadowing() {
        // var_from_a depends on var_from_b
        let mut mod_a = Module {
            module_path: ModulePath::new("a"),
            required_modules: vec![ModulePath::new("b")],
            ..build_module()
        };
        mod_a
            .scope
            .define_variable("var_from_a", Node::reference("var_from_b"));

        // var_from_b depends on var_from_c
        let mut mod_b = Module {
            module_path: ModulePath::new("b"),
            required_modules: vec![ModulePath::new("c")],
            ..build_module()
        };
        mod_b
            .scope
            .define_variable("var_from_b", Node::reference("var_from_c"));

        // var_from_c resolves to 420
        let mut mod_c = Module {
            module_path: ModulePath::new("c"),
            required_modules: vec![],
            ..build_module()
        };
        mod_c
            .scope
            .define_variable("var_from_c", Ast::new(420.into()));

        let mut main_module = Module {
            module_path: ModulePath::new("foo"),
            required_modules: vec![ModulePath::new("a")],
            ..build_module()
        };
        main_module
            .scope
            .define_variable("var_from_c", Ast::new(1.into()));
        let module_loader = ModuleLoader {
            main_module: &main_module,
            attempted: Default::default(),
            loaded: Arc::new(RwLock::new(HashMap::from([
                (ModulePath::new("a"), mod_a),
                (ModulePath::new("b"), mod_b),
                (ModulePath::new("c"), mod_c),
            ]))),
            failed: Default::default(),
            loader_root: path::Path::new("").to_path_buf(),
            is_dirty: false,
        };

        let dependencies = module_loader.into_direct_dependencies().unwrap();

        // var_from_c should be what main_module declared it as, not what any dependencies declared
        // it is.  i.e., main shadowed the imports
        assert_eq!(
            dependencies.variables.get("var_from_c").unwrap(),
            &Ast::new(1.into())
        );
    }

    #[test]
    fn into_direct_dependencies_cyclic() {
        // TODO
    }
}
