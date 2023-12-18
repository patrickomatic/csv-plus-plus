use crate::Module;

#[derive(Copy, Clone, Debug)]
pub(super) enum DependencyRelation {
    Direct,
    Transitive,
}

#[derive(Debug)]
pub(super) struct Dependency {
    pub(super) relation: DependencyRelation,
    pub(super) module: Module,
}

#[cfg(test)]
impl Dependency {
    pub(crate) fn direct(module: Module) -> Self {
        Self {
            relation: DependencyRelation::Direct,
            module,
        }
    }

    pub(crate) fn transitive(module: Module) -> Self {
        Self {
            relation: DependencyRelation::Transitive,
            module,
        }
    }
}
