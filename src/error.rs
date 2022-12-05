//! Error type
//!
//! This module defines the `Error` type.

/// Error type. See module level [documentation](self)
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The Command parsing error.
    #[error("Unsupported command")]
    UnsupportedCommand,

    /// Invalid database path error.
    #[error("Invalid database path.")]
    InvalidDatabasePathError,

    /// The error when for a lookup attempt without first loading the database.
    #[error("Unloaded database error.")]
    UnloadedDatabaseError,

    /// The record parsing error.
    #[error("Invalid record")]
    ParseError,

    /// The record lookup error.
    #[error("Record not found")]
    LookupError,

    /// The Sqlx related error.
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
