//! Protocol trait.
//!
//! This module defines the communication protocol with other processes.

use crate::Result;

/// Protocol trait. See module level [documentation](self)
pub trait Protocol {
    /// Loads the database in to memory.
    fn load(&mut self) -> Result<&str>;

    /// Exit the process.
    fn exit(&self) -> &str;

    /// Lookup the IP address in the database.
    fn lookup(&mut self, ip: u32) -> Result<String>;
}
