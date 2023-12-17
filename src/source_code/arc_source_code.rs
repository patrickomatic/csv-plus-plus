use super::SourceCode;
use std::ops;
use std::sync;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct ArcSourceCode(sync::Arc<SourceCode>);

impl ArcSourceCode {
    pub(crate) fn new(inner: SourceCode) -> Self {
        Self(sync::Arc::new(inner))
    }
}

impl ops::Deref for ArcSourceCode {
    type Target = SourceCode;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
