//! Error handling functions
mod error;
mod inner_error;

pub use error::Error;
pub use inner_error::InnerError;

pub type Result<T> = std::result::Result<T, Error>;
pub type InnerResult<T> = std::result::Result<T, InnerError>;
