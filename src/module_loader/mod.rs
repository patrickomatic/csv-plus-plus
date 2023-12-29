//! # `ModuleLoader`
//!
//! A multithreaded module loader that will resursively load the dependencies for a given
//! `Scope`.
//!
// TODO:
// * make it so that `---` is not required
use crate::{compiler_error, Error, Module, ModulePath, Result, Scope};
use log::{debug, info};
use petgraph::{algo, graph};
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
pub(super) struct ModuleLoader {
    attempted: Attempted,
    failed: Failed,
    loaded: Loaded,
    loader_root: path::PathBuf,
    main_module: Module,
    is_dirty: bool,
    use_cache: bool,
}

// TODO: ideally this shouldn't take a $source_code and the calling part does the map_err
macro_rules! eval_fns_or_vars {
    ($scope:expr, $functions_or_variables:ident, $source_code:expr) => {{
        for (name, ast) in $scope.$functions_or_variables.clone().into_iter() {
            $scope.$functions_or_variables.insert(
                name,
                ast.eval(&$scope, None)
                    .map_err(|e| $source_code.eval_error(e, None))?,
            );
        }
    }};
}

// TODO:
// this probably doesn't need to be a macro
macro_rules! eval_scope {
    ($scope:expr, $source_code:expr) => {
        eval_fns_or_vars!($scope, variables, $source_code);
        eval_fns_or_vars!($scope, functions, $source_code);
    };
}

// TODO:
// * get rid of unwrap()s
// * see if I can reduce the clone()s
impl ModuleLoader {
    /// Recursively load the dependencies from the given `scope` while collecting any errors into
    /// `failed` and sucesses into `loaded`. The idea being that we want to show as many errors as
    /// possible to the user (otherwise it's annoying to have them fix and re-compile one-by-one),
    /// so we accumulate and keep going.  But in the end fail if there are any errors at all.
    pub(super) fn load_main<P: Into<path::PathBuf>>(
        main_module: Module,
        relative_to: P,
        use_cache: bool,
    ) -> Result<Module> {
        let mut module_loader = Self {
            attempted: sync::Arc::default(),
            failed: sync::Arc::default(),
            loaded: sync::Arc::default(),
            main_module,
            loader_root: relative_to.into(),
            is_dirty: false,
            use_cache,
        };

        // we need to do this in a loop because every time we reload dirty dependencies we're
        // pulling in changes to the source code, which could mean the author added a new `use ...`
        // in which case we need to load it
        loop {
            module_loader.load(&module_loader.main_module)?;
            module_loader = module_loader.propagate_dirty_flag()?;
            if module_loader.is_dirty {
                module_loader.reload_dirty_modules()?;
            } else {
                break;
            }
        }

        module_loader.eval_dependencies()?;
        module_loader.merge_direct_dependencies()
    }

    /// Returns only the direct dependencies for this module graph.  For example if our Module A
    /// requires Module B which in turn requires Module C, we will only get vars & functions from
    /// Module B, not from Module C (or any other indirect dependencies)
    fn merge_direct_dependencies(self) -> Result<Module> {
        if self.has_failures() {
            Err(Error::ModuleLoadErrors(
                sync::Arc::try_unwrap(self.failed).unwrap().into_inner()?,
            ))
        } else {
            let mut loaded = self.loaded.write()?;
            let mut main_module = self.main_module;

            for req_path in &main_module.required_modules {
                main_module
                    .scope
                    .merge(&loaded.remove(req_path).unwrap().scope);
            }

            Ok(main_module)
        }
    }

    fn load_dependency_graph(&self) -> graph::Graph<ModulePath, ()> {
        let loaded = self.loaded.read().unwrap();
        info!(
            "Creating dependency graph with {} dependencies",
            loaded.len()
        );

        let mut dep_graph = graph::Graph::new();

        let main_node = dep_graph.add_node(self.main_module.module_path.clone());

        let mut loaded_nodes = collections::HashMap::new();
        loaded_nodes.insert(&self.main_module.module_path, main_node);

        // load all of the direct dependencies
        for p in &self.main_module.required_modules {
            let direct_dep_node = dep_graph.add_node(p.clone());
            loaded_nodes.insert(p, direct_dep_node);
            dep_graph.add_edge(main_node, direct_dep_node, ());
        }

        for (p, dep_mod) in loaded.iter() {
            // TODO: clean this up with a macro or something. or maybe wrap the petgraph into a
            // `UniqueGraph` of my own making
            let dep_node = loaded_nodes
                .get(p)
                .copied()
                .unwrap_or_else(|| dep_graph.add_node(p.clone()));
            loaded_nodes.insert(p, dep_node);

            for required_mod in &dep_mod.required_modules {
                let dep_dep_node = loaded_nodes
                    .get(required_mod)
                    .copied()
                    .unwrap_or_else(|| dep_graph.add_node(required_mod.clone()));
                loaded_nodes.insert(required_mod, dep_dep_node);

                dep_graph.add_edge(dep_node, dep_dep_node, ());
            }
        }

        debug!("Loaded dependency graph {dep_graph:?}");

        dep_graph
    }

    fn dirty_nodes(&self) -> collections::HashSet<ModulePath> {
        let loaded = self.loaded.read().unwrap();
        let dep_graph = self.load_dependency_graph();

        let mut dirty_nodes: collections::HashSet<ModulePath> = collections::HashSet::default();
        for node in dep_graph.node_indices().collect::<Vec<_>>() {
            let Some(module) = loaded.get(&dep_graph[node]) else {
                continue;
            };

            if module.is_dirty {
                for graph_path in algo::simple_paths::all_simple_paths::<Vec<_>, _>(
                    &dep_graph,
                    // just assume that the main module is at index 0
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
                    Module::load_from_source_from_filename(
                        mp.clone(),
                        module.source_code.filename.clone(),
                    )?,
                );
            }
        }

        Ok(())
    }

    fn eval_dependencies(&mut self) -> Result<()> {
        let dep_graph = self.load_dependency_graph();
        let mut loaded = self.loaded.write()?;

        // now that we have a graph, use Tarjan's algo to give us a topological sort which will
        // represent the dependencies in the order they need to be resolved.
        let resolution_order = algo::tarjan_scc(&dep_graph)
            .into_iter()
            .flatten()
            .map(|p| dep_graph[p].clone());

        debug!("Resolving dependencies in order {resolution_order:?}");

        // we'll need a local copy of all the scopes as we modify `loaded`
        let mut scopes = collections::HashMap::<ModulePath, Scope>::new();
        for (mp, m) in loaded.iter() {
            scopes.insert(mp.clone(), m.scope.clone());
        }

        for mp_to_resolve in resolution_order {
            let Some(to_resolve) = loaded.get_mut(&mp_to_resolve) else {
                continue;
            };

            if !to_resolve.needs_eval {
                continue;
            }

            // let mut local_scope = to_resolve.scope.clone();
            for req_path in to_resolve.required_modules.iter().rev() {
                let Some(dep_scope) = scopes.get(req_path) else {
                    compiler_error(format!(
                        "Expected dependent module to have been loaded: {req_path}"
                    ))
                };

                to_resolve.scope.merge(dep_scope);
            }

            eval_scope!(to_resolve.scope, to_resolve.source_code);

            // now that we've evaled it that's the last step, write the csvpo file
            if self.use_cache {
                to_resolve.write_object_file()?;
            }

            scopes.insert(to_resolve.module_path.clone(), to_resolve.scope.clone());
        }

        Ok(())
    }

    fn has_failures(&self) -> bool {
        !self.failed.try_read().unwrap().is_empty()
    }

    fn load(&self, module: &Module) -> Result<()> {
        let mut to_attempt = collections::HashSet::new();
        // hold a lock while we reserve all of the dependencies we're going to resolve (by
        // preemptively marking them in `attempted`)
        {
            let mut attempted = self.attempted.write()?;
            for module_path in &module.required_modules {
                if attempted.contains(module_path) {
                    // another module has already loaded it
                    continue;
                }

                attempted.insert(module_path.clone());
                to_attempt.insert(module_path.clone());
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
        let load_result = if self.use_cache {
            Module::load_from_cache(module_path.clone(), relative_to, &self.loader_root)
        } else {
            Module::load_from_source_relative(module_path.clone(), relative_to, &self.loader_root)
        };

        match load_result {
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
#[allow(clippy::similar_names)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::test_utils::*;
    use crate::*;

    #[test]
    fn load_main_empty() {
        assert!(ModuleLoader::load_main(build_module(), "", true).is_ok());
    }

    #[test]
    fn load_main_require_error() {
        let mut module = build_module();
        module.required_modules.push(ModulePath::new("bar"));

        assert!(ModuleLoader::load_main(module, "", true).is_err());
    }

    #[test]
    fn load_main_valid_files() {
        let mod1 = TestSourceCode::new(
            "csv",
            "
    a := 42
    ---
            ",
        );
        let mod2 = TestSourceCode::new(
            "csv",
            "
    b := 24
    ---
            ",
        );
        let main_module = Module {
            module_path: ModulePath::new("main"),
            required_modules: vec![(&mod1).into(), (&mod2).into()],
            ..build_module()
        };
        let main_module = ModuleLoader::load_main(main_module, "", true).unwrap();

        assert_eq!(
            main_module.scope.variables.get("a").unwrap(),
            &Ast::new(Node::var("a", VariableValue::Ast(42.into()))),
        );
        assert_eq!(
            main_module.scope.variables.get("b").unwrap(),
            &Ast::new(Node::var("b", VariableValue::Ast(24.into()))),
        );
    }

    #[test]
    fn load_main_in_directory() {
        let dep_mod = TestSourceCode::new_in_dir(
            "csv",
            "
    a := 42
    ---
            ",
        );
        let main_module = Module {
            module_path: ModulePath::new("main"),
            required_modules: vec![(&dep_mod).into()],
            ..build_module()
        };
        let main_module = ModuleLoader::load_main(main_module, "", true).unwrap();

        assert!(main_module.scope.variables.contains_key("a"));
    }

    #[test]
    fn load_main_double_load() {
        let mod1 = TestSourceCode::new(
            "csv",
            "
    a := 42
    ---
            ",
        );
        let mod1_path = ModulePath::from(&mod1);
        let mod2 = TestSourceCode::new(
            "csv",
            &format!(
                "
    use {mod1_path}
    b := 24
    ---
            "
            ),
        );

        let main_module = Module {
            module_path: ModulePath::new("main"),
            required_modules: vec![(&mod1).into(), (&mod2).into()],
            ..build_module()
        };
        assert!(ModuleLoader::load_main(main_module, "", true).is_ok());
    }

    #[test]
    fn load_main_variable_dependencies() {
        let mod_c_file = TestSourceCode::new(
            "csv",
            "
var_from_c := 420
---
    ",
        );
        let mod_c_path = ModulePath::from(&mod_c_file);

        let mod_b_file = TestSourceCode::new(
            "csv",
            &format!(
                "
use {mod_c_path}
var_from_b := var_from_c
---
"
            ),
        );
        let mod_b_path = ModulePath::from(&mod_b_file);

        let mod_a_file = TestSourceCode::new(
            "csv",
            &format!(
                "
use {mod_b_path}
var_from_a := var_from_b
---
"
            ),
        );
        let mod_a_path = ModulePath::from(&mod_a_file);

        let main_module = Module {
            module_path: ModulePath::new("foo"),
            required_modules: vec![mod_a_path],
            ..build_module()
        };
        let main_module = ModuleLoader::load_main(main_module, "", true).unwrap();

        assert_eq!(
            main_module.scope.variables.get("var_from_a").unwrap(),
            &Node::var("var_from_a", VariableValue::Ast(420.into())).into()
        );
        assert!(!main_module.scope.variables.contains_key("var_from_b"));
        assert!(!main_module.scope.variables.contains_key("var_from_c"));
    }

    #[test]
    fn load_main_function_dependencies() {
        let mod_b_file = TestSourceCode::new(
            "csv",
            "
var_from_b := 420
---
",
        );
        let mod_b_path = ModulePath::from(&mod_b_file);

        let mod_a_file = TestSourceCode::new(
            "csv",
            &format!(
                "
use {mod_b_path}

fn fn_from_a() var_from_b
---
",
            ),
        );
        let mod_a_path = ModulePath::from(&mod_a_file);

        let main_module = Module {
            module_path: ModulePath::new("foo"),
            required_modules: vec![mod_a_path],
            ..build_module()
        };
        let main_module = ModuleLoader::load_main(main_module, "", true).unwrap();

        assert_eq!(
            main_module.scope.functions.get("fn_from_a").unwrap(),
            &Ast::new(Node::fn_def("fn_from_a", &[], Node::Integer(420)))
        );
    }

    #[test]
    fn into_direct_dependencies_shadowing() {
        let mod_c_source_code = TestSourceCode::new(
            "csv",
            "
var_from_c := 420
---
    ",
        );
        let mod_c_path = ModulePath::from(&mod_c_source_code);

        let mod_b_source_code = TestSourceCode::new(
            "csv",
            &format!(
                "
use {mod_c_path}
var_from_b := var_from_c
---
"
            ),
        );
        let mod_b_path = ModulePath::from(&mod_b_source_code);

        let mod_a_source_code = TestSourceCode::new(
            "csv",
            &format!(
                "
use {mod_b_path}
var_from_a := var_from_b
---
"
            ),
        );
        let mod_a_path = ModulePath::from(&mod_a_source_code);

        let mut main_module = Module {
            module_path: ModulePath::new("foo"),
            required_modules: vec![mod_a_path],
            ..build_module()
        };
        main_module.scope.define_variable(
            "var_from_a",
            Ast::new(Node::var("var_from_a", VariableValue::Ast(1.into()))),
        );
        let main_module = ModuleLoader::load_main(main_module, "", true).unwrap();

        assert_eq!(
            main_module.scope.variables.get("var_from_a").unwrap(),
            &Node::var("var_from_a", VariableValue::Ast(1.into())).into()
        );
    }

    #[test]
    fn load_main_dependencies_cyclic() {
        // TODO
    }
}
