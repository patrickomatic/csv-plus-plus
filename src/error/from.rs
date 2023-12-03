use super::Error;
use std::sync;

impl<T> From<sync::PoisonError<T>> for Error {
    fn from(e: sync::PoisonError<T>) -> Self {
        Error::ModuleLoadError(format!("Runtime lock error while loading modules: {e}"))
    }
}
