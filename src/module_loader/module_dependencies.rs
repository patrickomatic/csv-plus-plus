//! # ModuleDependencies
//!
//! The dependency-resolution logic for how csv++ modules (`use` statements) are loaded.
//!
//! The primary idea here is that when you `use` a dependency, you should only get the symbols
//! (functions and variables) that module defines.  Not the ones it depends on also.  To accomplish
//! this we use Tarjan's algorithm to give us a sorted ordering of module dependencies, then
//! recursively eval them leaving just the most direct dependencies with all of their components
//! evaled inline.  In effect erasing the names of the transitive dependencies (since they get
//! evaled into place)
use super::dependency::DependencyRelation;
use super::LoadedModules;
use crate::ast::{Functions, Variables};
use crate::{Scope, ModulePath};
use petgraph::graph;
use std::collections;

#[derive(Debug, Default)]
pub(crate) struct ModuleDependencies {
    pub(crate) functions: Functions,
    pub(crate) variables: Variables,
}

impl ModuleDependencies {
    /// Extract all direct dependencies on `scope`.  
    pub(super) fn direct_dependencies(
        module_path: &ModulePath,
        scope: &Scope,
        loaded: LoadedModules,
    ) -> Self {
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

        let mut evaled_deps = ModuleDependencies::default();
        let mut all_fns: Functions = Default::default();
        let mut all_vars: Variables = Default::default();

        for dep in resolution_order {
            match dep.relation {
                DependencyRelation::Direct => {
                    // we're on the last one -
                    for (var_name, ast) in dep.scope.variables.clone().into_iter() {
                        // XXX eval it
                        evaled_deps.variables.insert(var_name, ast);
                    }
                    for (fn_name, ast) in dep.scope.functions.clone().into_iter() {
                        // XXX eval it
                        evaled_deps.functions.insert(fn_name, ast);
                    }
                }

                DependencyRelation::Transitive => {
                    // TODO: we could optimize this by not evaling if required_modules.is_empty()
                    for (var_name, ast) in dep.scope.variables.clone().into_iter() {
                        // XXX eval it
                        all_vars.insert(var_name, ast);
                    }
                    for (fn_name, ast) in dep.scope.functions.clone().into_iter() {
                        // XXX eval it
                        all_fns.insert(fn_name, ast);
                    }
                }
            }
        }

        evaled_deps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{module_loader::dependency::Dependency, *};
    use std::collections;

    #[test]
    fn direct_dependencies_empty() {
        let scope = Scope::default();
        let module_path = ModulePath(vec!["main".to_string()]);
        let loaded = collections::HashMap::new();
        let deps = ModuleDependencies::direct_dependencies(&module_path, &scope, loaded);

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
        // ModuleDependencies::direct_dependencies(&module_path, &scope, loaded).is_ok()
        // );
        todo!()
    }

    #[test]
    fn direct_dependencies_cyclic() {
        // XXX
        let scope = Scope::default();
        let module_path = ModulePath(vec!["main".to_string()]);
        // let loaded = collections::HashMap::new();

        // assert!(
        // ModuleDependencies::direct_dependencies(&module_path, &scope, loaded).is_ok()
        // );
    }
}
