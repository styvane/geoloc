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

pub mod command;
pub mod database;
mod error;
pub mod protocol;

pub use self::error::Error;

/// Result Type.
///
/// It's an alias for the `std::result::Result` type with the error
/// type [`Error`](crate::Error).
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The Query trait specifies the query operations.
#[async_trait::async_trait]
pub trait QueryDriver {
    /// Find ip address info.
    async fn find_ip(&self, ip: u32) -> Result<String>;
}
