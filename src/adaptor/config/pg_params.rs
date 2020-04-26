use crate::errors::{Error, Result};
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Deserialize)]
pub struct PostgresParams {
    pub user: String,
    pub password: String,
    pub host: String,
    pub database: String,
    pub port: i32,
}

impl PostgresParams {
    pub fn load(toml_config: &str) -> Result<Self> {
        let toml_config: RawConfig = toml::from_str(toml_config)?;
        let toml_config = toml_config.postgres;
        let env_config: Option<RawPostgresParams> = envy::prefixed("PG").from_env().ok();

        // Left-to-right right overrides left
        let configs: Vec<RawPostgresParams> = (&mut [toml_config, env_config])
            .into_iter()
            .filter_map(|x| x.take())
            .collect();

        (&configs[..]).try_into()
    }
}

impl TryFrom<&[RawPostgresParams]> for PostgresParams {
    type Error = Error;

    fn try_from(value: &[RawPostgresParams]) -> Result<PostgresParams> {
        let params = value
            .iter()
            .fold(RawPostgresParams::default(), |mut acc, x| {
                acc.user = x.user.to_owned().or(acc.user);
                acc.password = x.password.to_owned().or(acc.password);
                acc.host = x.host.to_owned().or(acc.host);
                acc.database = x.database.to_owned().or(acc.database);
                acc.port = x.port.to_owned().or(acc.port);
                acc
            });

        Ok(Self {
            user: params
                .user
                .ok_or(Error::BadPgConfig("No user".to_string()))?,
            password: params
                .password
                .ok_or(Error::BadPgConfig("No password".to_string()))?,
            host: params
                .host
                .ok_or(Error::BadPgConfig("No host".to_string()))?,
            database: params
                .database
                .ok_or(Error::BadPgConfig("No database".to_string()))?,
            port: params
                .port
                .ok_or(Error::BadPgConfig("No port".to_string()))?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    postgres: Option<RawPostgresParams>,
}

#[derive(Debug, Deserialize)]
struct RawPostgresParams {
    pub user: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub database: Option<String>,
    pub port: Option<i32>,
}

impl Default for RawPostgresParams {
    fn default() -> Self {
        Self {
            user: None,
            password: None,
            host: None,
            database: None,
            port: Some(5432),
        }
    }
}
