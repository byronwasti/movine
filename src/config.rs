use crate::adaptor::{PostgresAdaptor, SqliteAdaptor};
use crate::errors::{Error, Result};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;

mod postgres;
mod sqlite;

pub use sqlite::SqliteParams;
pub use self::postgres::PostgresParams;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub postgres: Option<PostgresParams>,
    pub sqlite: Option<SqliteParams>,
}

impl Config {
    pub fn from_file(file: &str) -> Result<Self> {
        let mut file = File::open(file).map_err(|_| Error::ConfigFileNotFound)?;
        let mut config = String::new();
        file.read_to_string(&mut config)?;
        let config = toml::from_str(&config)?;
        Ok(config)
    }

    pub fn into_adaptor(self) -> Result<Adaptor> {
        match self {
            Config {
                postgres: Some(params),
                ..
            } => Ok(Adaptor::Postgres(PostgresAdaptor::from_params(&params)?)),
            Config {
                sqlite: Some(params),
                ..
            } => Ok(Adaptor::Sqlite(SqliteAdaptor::from_params(&params)?)),
            _ => Err(Error::AdaptorNotFound),
        }
    }

    pub fn into_pg_adaptor(self) -> Result<PostgresAdaptor> {
        if let Some(params) = self.postgres {
            Ok(PostgresAdaptor::from_params(&params)?)
        } else {
            Err(Error::AdaptorNotFound)
        }
    }

    pub fn into_sqlite_adaptor(self) -> Result<SqliteAdaptor> {
        if let Some(params) = self.sqlite {
            Ok(SqliteAdaptor::from_params(&params)?)
        } else {
            Err(Error::AdaptorNotFound)
        }
    }
}

pub enum Adaptor {
    Postgres(PostgresAdaptor),
    Sqlite(SqliteAdaptor),
}
