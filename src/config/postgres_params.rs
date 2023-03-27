use crate::errors::{Error, Result};
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct PostgresParams {
    pub user: String,
    pub password: Option<String>,
    pub host: String,
    pub database: String,
    pub port: i32,
    pub sslrootcert: Option<String>,
}

impl TryFrom<&[&RawPostgresParams]> for PostgresParams {
    type Error = Error;

    fn try_from(value: &[&RawPostgresParams]) -> Result<PostgresParams> {
        let params = value
            .iter()
            .fold(RawPostgresParams::default(), |mut acc, x| {
                acc.user = x.user.to_owned().or(acc.user);
                acc.password = x.password.to_owned().or(acc.password);
                acc.host = x.host.to_owned().or(acc.host);
                acc.database = x.database.to_owned().or(acc.database);
                acc.port = x.port.to_owned().or(acc.port);
                acc.sslrootcert = x.sslrootcert.to_owned().or(acc.sslrootcert);
                acc
            });

        match params {
            RawPostgresParams {
                user: Some(user),
                password,
                database: Some(database),
                host: Some(host),
                port: Some(port),
                sslrootcert,
            } => Ok(Self {
                user,
                password,
                host,
                database,
                port,
                sslrootcert,
            }),
            p => Err(Error::PgParamError {
                user: p.user.is_some(),
                password: p.password.is_some(),
                database: p.database.is_some(),
                host: p.host.is_some(),
                port: p.port.is_some(),
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawPostgresParams {
    pub user: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub database: Option<String>,
    pub port: Option<i32>,
    pub sslrootcert: Option<String>,
}

impl RawPostgresParams {
    pub fn load_from_env() -> Result<Self> {
        let params = envy::prefixed("PG").from_env()?;
        Ok(params)
    }

    pub fn is_any(&self) -> bool {
        self.user.is_some()
            || self.password.is_some()
            || self.host.is_some()
            || self.database.is_some()
            || self.port.is_some()
    }
}

impl Default for RawPostgresParams {
    fn default() -> Self {
        Self {
            user: None,
            password: None,
            host: None,
            database: None,
            port: Some(5432),
            sslrootcert: None,
        }
    }
}
