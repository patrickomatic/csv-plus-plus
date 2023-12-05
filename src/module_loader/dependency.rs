use crate::Scope;

#[derive(Copy, Clone, Debug)]
pub(super) enum DependencyRelation {
    Direct,
    Transitive,
}

#[derive(Debug)]
pub(super) struct Dependency {
    pub(super) relation: DependencyRelation,
    pub(super) scope: Scope,
}

impl Dependency {
    #[cfg(test)]
    pub(super) fn direct(scope: Scope) -> Self {
        Self {
            relation: DependencyRelation::Direct,
            scope,
        }
    }

    #[cfg(test)]
    pub(super) fn transitive(scope: Scope) -> Self {
        Self {
            relation: DependencyRelation::Transitive,
            scope,
        }
    }
}
