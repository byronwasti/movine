use crate::errors::{Error, Result};
use log::debug;
use serde::Deserialize;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::fs;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use crate::DbAdaptor;

mod postgres_params;
mod sqlite_params;

pub use self::postgres_params::PostgresParams;
use self::postgres_params::RawPostgresParams;
use sqlite_params::RawSqliteParams;
pub use sqlite_params::SqliteParams;

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

impl Config {
    pub fn load(file: &str) -> Result<Self> {
        let raw_config = RawConfig::load_file(file);
        let pg_env_params = RawPostgresParams::load_from_env();
        let sqlite_env_params = RawSqliteParams::load_from_env();
        let database_url = std::env::var("DATABASE_URL");

        debug!(
            "Config information loaded:
            file: {:?}
            pg_env: {:?}
            sqlite_env: {:?}
            database_url: {:?}",
            &raw_config, &pg_env_params, &sqlite_env_params, &database_url
        );

        if let Ok(database_url) = database_url {
            debug!("Using database_url provided.");
            return Ok(Config {
                database_url: Some(database_url),
                ..Self::default()
            });
        }

        let raw_config = match raw_config {
            Ok(raw_config) => Some(raw_config),
            Err(Error::IoError(e)) if e.kind() == std::io::ErrorKind::NotFound => {
                debug!("Config file not found.");
                None
            }
            Err(e) => {
                return Err(e);
            }
        };

        match raw_config {
            Some(RawConfig {
                postgres: Some(pg_params),
                ..
            }) => {
                debug!("Using postgres config-file params provided.");
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
                debug!("Using sqlite config-file params provided.");
                let all_params = [Ok(sqlite_params), sqlite_env_params];
                let params: Vec<_> = all_params.iter().filter_map(|x| x.as_ref().ok()).collect();
                let params = (&params[..]).try_into()?;
                Ok(Self {
                    sqlite: Some(params),
                    ..Self::default()
                })
            }
            _ => match (pg_env_params, sqlite_env_params) {
                (Ok(pg_env_params), _) if pg_env_params.is_any() => {
                    debug!("Using postgres env vars provided.");
                    let params = [&pg_env_params];
                    let params = (&params[..]).try_into()?;
                    Ok(Self {
                        postgres: Some(params),
                        ..Self::default()
                    })
                }
                (_, Ok(sqlite_env_params)) if sqlite_env_params.is_any() => {
                    debug!("Using sqlite env vars provided.");
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

    pub fn into_db_adaptor(&self) -> Result<DbAdaptorKind> {
        match self {
            Config {
                database_url: Some(url),
                ..
            } => {
                if url.starts_with("postgres") {
                    let conn = postgres::Client::connect(&url, postgres::NoTls)?;
                    Ok(DbAdaptorKind::Postgres(conn))
                } else {
                    Err(Error::AdaptorNotFound)
                }
            }
            Config {
                postgres: Some(params),
                ..
            } => {
                let url = format!(
                    "postgresql://{user}:{password}@{host}:{port}/{database}",
                    user = params.user,
                    password = params.password,
                    host = params.host,
                    port = params.port,
                    database = params.database,
                );
                let conn = if let Some(cert) = &params.sslcert {
                    let cert = fs::read(cert)?;
                    let cert = Certificate::from_pem(&cert)?;
                    let connector = TlsConnector::builder().add_root_certificate(cert).build()?;
                    let tls = MakeTlsConnector::new(connector);
                    postgres::Client::connect(&url, tls)?
                } else {
                    postgres::Client::connect(&url, postgres::NoTls)?
                };

                Ok(DbAdaptorKind::Postgres(conn))
            },
            Config {
                sqlite: Some(params),
                ..
            } => {
                let conn = rusqlite::Connection::open(&params.file)?;
                Ok(DbAdaptorKind::Sqlite(conn))
            }
            _ => {
                Err(Error::AdaptorNotFound)
            }
        }
    }
}

pub enum DbAdaptorKind {
    Postgres(postgres::Client),
    Sqlite(rusqlite::Connection),
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

