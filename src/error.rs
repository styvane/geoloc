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

    /// The CSV file related error.
    #[error(transparent)]
    CSVError(#[from] csv::Error),

    /// The record parsing error.
    #[error("Invalid record")]
    ParseError,

    /// The record lookup error.
    #[error("Record not found")]
    LookupError,

    /// The IO related error.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
