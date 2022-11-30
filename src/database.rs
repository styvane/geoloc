//! Database types.
//!
//! This module defines the database data structures.

use std::fs::File;
use std::path::Path;
use std::str;

use csv::{ByteRecord, Reader, ReaderBuilder, Trim};

use super::protocol::Protocol;
use crate::{Error, Result};

/// Database type.
pub struct Database<P> {
    /// The path to the underline file.
    path: P,

    /// A reader
    reader: Option<Reader<File>>,
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
        Ok(Self { path, reader: None })
    }
}

impl<P> Protocol for Database<P>
where
    P: AsRef<Path>,
{
    fn load(&mut self) -> Result<&str> {
        let reader = ReaderBuilder::new()
            .trim(Trim::All)
            .has_headers(false)
            .from_path(self.path.as_ref())?;
        self.reader.replace(reader);

        Ok("OK")
    }

    fn exit(&self) -> &str {
        "OK"
    }

    fn lookup(&mut self, ip: &[u8]) -> Result<String> {
        let reader = self.reader.as_mut().ok_or(Error::UnloadedDatabaseError)?;
        let mut record = ByteRecord::new();
        while reader.read_byte_record(&mut record)? {
            let Some(start) = record.get(0) else { continue};
            let Some(end) =  record.get(1) else { continue};

            if ip >= start && ip <= end {
                return Ok(format!(
                    "{},{}",
                    str::from_utf8(&record[2]).map_err(|_| Error::ParseError)?,
                    str::from_utf8(&record[5]).map_err(|_| Error::ParseError)?,
                ));
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
        assert!(db.reader.is_some(), "db reader not is None");
    }

    #[test]
    fn test_ip_lookup() {
        let mut file = NamedTempFile::new().expect("failed to create tempfile");

        struct Test<'a> {
            name: &'a str,
            want: &'a str,
            ip: &'a [u8],
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
                ip: b"16777472",
                ok: true,
            },
            Test {
                name: "nok",
                want: "",
                ip: b"16778940",
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
