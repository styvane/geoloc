//! Command type.
//!
//! This module defines the `Command` type which represents the
//! different command supported.

use std::{net::Ipv4Addr, str::FromStr};

use crate::Error;

/// Command type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
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
        value.parse()
    }
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd = match s {
            "LOAD" => Self::Load,
            "EXIT" => Self::Exit,
            _ => {
                let ip = s
                    .strip_prefix("LOOKUP")
                    .iter()
                    .filter_map(|ip| Ipv4Addr::from_str(ip.trim()).ok())
                    .next()
                    .ok_or(Error::UnsupportedCommand)?;

                let ip: u32 = ip.into();
                Self::Lookup(ip)
            }
        };

        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn create_lookup_command_from_valid_input_successfully(ips: Vec<Ipv4Addr>) -> bool {
        let count = ips.len();
        ips.into_iter()
            .filter_map(|ip| Command::try_from(format!("LOOKUP {ip}")).ok())
            .count()
            == count
    }

    #[quickcheck]
    fn cannot_create_lookup_command_from_invalid_input(ips: Vec<String>) -> bool {
        ips.into_iter()
            .filter_map(|ip| Command::try_from(format!("LOOKUP {ip}")).ok())
            .count()
            == 0
    }
}
