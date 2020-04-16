use crate::adaptor::{DbAdaptor, PostgresAdaptor, SqliteAdaptor};
use crate::errors::{Error, Result};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;

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

    pub fn from_postgres_params(
        username: &str,
        password: &str,
        host: &str,
        database: &str,
        port: i32,
    ) -> Self {
        let pg_params = PostgresParams {
            username: username.into(),
            password: password.into(),
            host: host.into(),
            database: database.into(),
            port,
        };
        Self {
            postgres: Some(pg_params),
            sqlite: None,
        }
    }

    pub fn from_sqlite_params(file: &str) -> Self {
        let sqlite = SqliteParams { file: file.into() };
        Self {
            postgres: None,
            sqlite: Some(sqlite),
        }
    }

    pub fn into_adaptor(&self) -> Result<Box<dyn DbAdaptor>> {
        match self {
            Config {
                postgres: Some(params),
                ..
            } => {
                let pg = PostgresAdaptor::new(&params)?;
                Ok(Box::new(pg))
            }
            Config {
                sqlite: Some(params),
                ..
            } => {
                let sqlite = SqliteAdaptor::new(&params)?;
                Ok(Box::new(sqlite))
            }
            _ => Err(Error::AdaptorNotFound),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostgresParams {
    pub username: String,
    pub password: String,
    pub host: String,
    pub database: String,
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct SqliteParams {
    pub file: String,
}
