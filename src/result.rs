//! This module defines defines an alias for the `Result` type with the error
//! type [`Error`](crate::Error).

/// Result Type. See module level [documentation](self).
pub type Result<T> = std::result::Result<T, Error>;
pub type Result<T> = Result<T, crate::Error>;
