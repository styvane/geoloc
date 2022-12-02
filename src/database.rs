//! Database types.
//!
//! This module defines the database data structures.

use std::path::Path;
use std::str;

use csv::{ByteRecord, ReaderBuilder, Trim};

use super::protocol::Protocol;
use crate::{command::Command, Error, Result};

/// Database type.
pub struct Database<P> {
    /// Path to the database file.
    path: P,

    /// IP address range table.
    iptable: Vec<(u32, u32, Vec<u8>)>,
}

impl<P> Database<P>
where
    P: AsRef<Path>,
{
    /// Creates new database.
    pub fn new(path: P) -> Result<Self> {
        if !path.as_ref().exists() {
            return Err(Error::InvalidDatabasePathError);
        }

        Ok(Self {
            path,
            iptable: vec![],
        })
    }

    /// Responds to requests
    pub fn respond(&mut self, cmd: &Command) -> Result<String> {
        let data = match cmd {
            Command::Load => self.load()?.into(),
            Command::Exit => self.exit().into(),
            Command::Lookup(ip) => self.lookup(*ip)?,
        };
        Ok(data)
    }
    /// Caches the subset of columns required.
    pub fn populate_cache(&mut self) -> Result<()> {
        let mut reader = ReaderBuilder::new()
            .trim(Trim::All)
            .has_headers(false)
            .from_path(self.path.as_ref())?;

        let mut record = ByteRecord::new();

        let mut table = vec![];

        while reader.read_byte_record(&mut record)? {
            let start: u32 = str::from_utf8(&record[0])
                .map_err(|_| Error::ParseError)?
                .parse()
                .map_err(|_| Error::ParseError)?;
            let end: u32 = str::from_utf8(&record[1])
                .map_err(|_| Error::ParseError)?
                .parse()
                .map_err(|_| Error::ParseError)?;
            let country_city = [&record[2], b",", &record[5]].concat();
            table.push((start, end, country_city));
        }

        self.iptable = table;
        Ok(())
    }
}

impl<P> Protocol for Database<P>
where
    P: AsRef<Path>,
{
    fn load(&mut self) -> Result<&str> {
        self.populate_cache()?;
        Ok("OK")
    }

    fn exit(&self) -> &str {
        "OK"
    }

    fn lookup(&mut self, ip: u32) -> Result<String> {
        for (start, end, v) in &self.iptable {
            if ip >= *start && ip <= *end {
                return str::from_utf8(v)
                    .map_err(|_| Error::ParseError)
                    .map(|s| s.to_string());
            }
        }
        Err(Error::LookupError)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn load_database_successfuly() {
        let file = NamedTempFile::new().expect("failed to create tempfile");
        let mut db = Database::new(file.path()).expect("failed to create db");
        assert!(db.load().is_ok(), "failed to load db");
    }

    #[test]
    fn test_ip_lookup() {
        let mut file = NamedTempFile::new().expect("failed to create tempfile");
        struct Test<'a> {
            name: &'a str,
            want: &'a str,
            ip: u32,
            ok: bool,
        }

        file.write_all(br#""0","16777215","-","-","-","-","0.000000","0.000000"
"16777216","16777471","US","United States of America","California","Los Angeles","34.052230","-118.243680"
"16777472","16778239","CN","China","Fujian","Fuzhou","26.061390","119.306110"
"16778240","16779263","AU","Australia","Victoria","Melbourne","-37.814000","144.963320"
"16779264","16781311","CN","China","Guangdong","Guangzhou","23.116670","113.250000"
"16781312","16785407","JP","Japan","Tokyo","Tokyo","35.689506","139.691700""#).expect("failed to write data");
        file.flush().expect("failed to flush");
        let mut db = Database::new(file.path()).expect("failed to create db");
        db.load().expect("failed to load db");

        let tests = &[
            Test {
                name: "ok",
                want: "CN,Fuzhou",
                ip: 16777472,
                ok: true,
            },
            Test {
                name: "nok",
                want: "",
                ip: 16778940,
                ok: false,
            },
        ];

        for tt in tests {
            let resp = db.lookup(tt.ip);

            match resp {
                Ok(value) if tt.ok => {
                    assert_eq!(value, tt.want, "{} test failed", tt.name);
                }
                _ => assert!(!tt.ok),
            }
        }
    }
}
