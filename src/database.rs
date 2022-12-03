//! Database types.
//!
//! This module defines the database data structures.

use std::path::Path;
use std::str;

use async_trait::async_trait;
use sqlx::sqlite::{SqliteConnectOptions, SqliteSynchronous};
use sqlx::SqlitePool;

use super::protocol::Protocol;
use crate::{command::Command, Error, QueryDriver, Result};

/// Database type.
#[allow(dead_code)]
pub struct Database<P, D> {
    /// Path to the database file.
    path: P,

    /// Query driver
    driver: Option<D>,
}

impl<P> Database<P, SqlitePool>
where
    P: AsRef<Path> + Sync,
{
    /// Creates new database.
    pub const fn new(path: P) -> Self {
        Self { path, driver: None }
    }

    /// Responds to requests
    pub async fn respond(&mut self, cmd: &Command) -> Result<String> {
        let data = match cmd {
            Command::Load => {
                let options = SqliteConnectOptions::new()
                    .filename(&self.path)
                    .synchronous(SqliteSynchronous::Normal);
                let driver = SqlitePool::connect_with(options).await?;
                self.load(driver)?.into()
            }
            Command::Exit => self.exit().into(),
            Command::Lookup(ip) => self.lookup(*ip).await?,
        };
        Ok(data)
    }
}

#[async_trait]
impl QueryDriver for SqlitePool {
    async fn find_ip(&self, ip: u32) -> Result<String> {
        let row = sqlx::query!(
            r#"SELECT country_code, city
                FROM iptable WHERE $1 BETWEEN start AND end
            "#,
            ip
        )
        .fetch_one(self)
        .await
        .map_err(|_| Error::LookupError)?;

        Ok(format!(
            "{},{}",
            str::from_utf8(&row.country_code).map_err(|_| Error::ParseError)?,
            str::from_utf8(&row.city).map_err(|_| Error::ParseError)?
        ))
    }
}

#[async_trait]
impl<P, D> Protocol<D> for Database<P, D>
where
    P: AsRef<Path> + Sync,
    D: QueryDriver + Send + Sync,
{
    fn load(&mut self, driver: D) -> Result<&str> {
        self.driver.replace(driver);
        Ok("OK")
    }

    fn exit(&self) -> &str {
        "OK"
    }

    async fn lookup(&self, ip: u32) -> Result<String> {
        let driver = self.driver.as_ref().ok_or(Error::UnloadedDatabaseError)?;
        driver.find_ip(ip).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_ip_lookup() {
        struct Test<'a> {
            name: &'a str,
            want: &'a str,
            ip: u32,
            ok: bool,
        }

        let path = std::env::var("CARGO_MANIFEST_DIR").expect("failed to get manifest dir");
        let path = format!("sqlite://{path}/testdata/test.db");

        let mut db: Database<_, SqlitePool> = Database::new(&path);

        let pool = SqlitePool::connect(&path)
            .await
            .expect("failed to connect to db");

        db.load(pool).expect("failed to load db");

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
            let resp = db.lookup(tt.ip).await;

            match resp {
                Ok(value) if tt.ok => {
                    assert_eq!(value, tt.want, "{} test failed", tt.name);
                }
                _ => assert!(!tt.ok),
            }
        }
    }
}
