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
