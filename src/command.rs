//! Command type.
//!
//! This module defines the `Command` type which represents the
//! different command supported.

use std::{net::Ipv4Addr, str::FromStr};

use crate::Error;

/// Command type.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Command {
    /// Load database into memory.
    Load,

    /// Lookup IP address
    Lookup(u32),

    /// Exit process
    Exit,
}

impl TryFrom<String> for Command {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let cmd = match value.as_str() {
            "LOAD" => Self::Load,
            "EXIT" => Self::Exit,
            _ => {
                let ip = value
                    .strip_prefix("LOOKUP")
                    .iter()
                    .filter_map(|ip| Ipv4Addr::from_str(ip.trim()).ok())
                    .next()
                    .ok_or(Error::UnsupportedCommand)?;

                Self::Lookup(ip.into())
            }
        };

        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {}
