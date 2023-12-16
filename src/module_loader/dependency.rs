use crate::{ArcSourceCode, Scope};

#[derive(Copy, Clone, Debug)]
pub(super) enum DependencyRelation {
    Direct,
    Transitive,
}

#[derive(Debug)]
pub(super) struct Dependency {
    pub(super) relation: DependencyRelation,
    pub(super) scope: Scope,
    pub(super) source_code: ArcSourceCode,
}

#[cfg(test)]
impl Dependency {
    pub(crate) fn direct(scope: Scope, source_code: ArcSourceCode) -> Self {
        Self {
            relation: DependencyRelation::Direct,
            scope,
            source_code,
        }
    }

    pub(crate) fn transitive(scope: Scope, source_code: ArcSourceCode) -> Self {
        Self {
            relation: DependencyRelation::Transitive,
            scope,
            source_code,
        }
    }
}
