//! Protocol trait.
//!
//! This module defines the communication protocol with other processes.

use crate::{QueryDriver, Result};

/// Protocol trait. See module level [documentation](self)
#[async_trait::async_trait]
pub trait Protocol<D: QueryDriver> {
    /// Loads the database in to memory.
    fn load(&mut self, driver: D) -> Result<&str>;

    /// Exit the process.
    fn exit(&self) -> &str;

    /// Lookup the IP address in the database.
    async fn lookup(&self, ip: u32) -> Result<String>;
}
