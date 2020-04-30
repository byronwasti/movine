use crate::errors::{Error, Result};
use serde::Deserialize;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use toml;

mod postgres;
mod sqlite;

pub use self::postgres::PostgresParams;
use self::postgres::RawPostgresParams;
use sqlite::RawSqliteParams;
pub use sqlite::SqliteParams;

#[derive(Debug)]
pub struct Config {
    pub postgres: Option<PostgresParams>,
    pub sqlite: Option<SqliteParams>,
    pub database_url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
         Config {
             postgres: None,
             sqlite: None,
             database_url: None,
         }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawConfig {
    pub postgres: Option<RawPostgresParams>,
    pub sqlite: Option<RawSqliteParams>,
}

impl RawConfig {
    pub fn load_file(file: &str) -> Result<RawConfig> {
        let mut file = File::open(file)?;
        let mut config = String::new();
        file.read_to_string(&mut config)?;
        let config = toml::from_str(&config)?;
        Ok(config)
    }
}

impl Config {
    pub fn load(file: &str) -> Result<Self> {
        let raw_config = RawConfig::load_file(file);
        let pg_env_params = RawPostgresParams::load_from_env();
        let sqlite_env_params = RawSqliteParams::load_from_env();

        let database_url = std::env::var("DATABASE_URL");
        if let Ok(database_url) = database_url {
            return Ok(Config {
                database_url: Some(database_url),
                ..Self::default()
            })
        }

        let raw_config = match raw_config {
            Ok(raw_config) => Some(raw_config),
            Err(Error::IoError(e)) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => {
                return Err(e);
            }
        };

        match raw_config {
            Some(RawConfig {
                postgres: Some(pg_params),
                ..
            }) => {
                let all_params = [Ok(pg_params), pg_env_params];
                let params: Vec<_> = all_params.iter().filter_map(|x| x.as_ref().ok()).collect();
                let params: PostgresParams = (&params[..]).try_into()?;
                Ok(Self {
                    postgres: Some(params),
                    ..Self::default()
                })
            }
            Some(RawConfig {
                sqlite: Some(sqlite_params),
                ..
            }) => {
                let all_params = [Ok(sqlite_params), sqlite_env_params];
                let params: Vec<_> = all_params.iter().filter_map(|x| x.as_ref().ok()).collect();
                let params = (&params[..]).try_into()?;
                Ok(Self {
                    sqlite: Some(params),
                    ..Self::default()
                })
            }
            _ => match (pg_env_params, sqlite_env_params) {
                (Ok(pg_env_params), _) => {
                    let params = [&pg_env_params];
                    let params = (&params[..]).try_into()?;
                    Ok(Self {
                        postgres: Some(params),
                    ..Self::default()
                    })
                }
                (_, Ok(sqlite_env_params)) => {
                    let params = [&sqlite_env_params];
                    let params = (&params[..]).try_into()?;
                    Ok(Self {
                        sqlite: Some(params),
                        ..Self::default()
                    })
                }
                _ => Err(Error::ConfigNotFound),
            },
        }
    }
}
