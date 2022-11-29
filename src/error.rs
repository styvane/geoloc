//! Error type
//!
//! This module defines the `Error` type.

/// Error type. See module level [documentation](self)
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Command parsing error.
    #[error("Unsupported command")]
    UnsupportedCommand,
}
