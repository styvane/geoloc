//! # Geoloc
//!
//! This crate perform IP addresses lookup in a reader.

#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    clippy::missing_docs_in_private_items,
    clippy::missing_const_for_fn
)]

mod command;
mod error;

pub use self::error::Error;

/// Result Type.
///
/// It's an alias for the `std::result::Result` type with the error
/// type [`Error`](crate::Error).
pub type Result<T> = std::result::Result<T, crate::Error>;
