use crate::errors::{PgParamError, Result};
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
    pub fn load(toml_config: &str) -> Result<std::result::Result<Self, PgParamError>> {
        let toml_config: RawConfig = toml::from_str(toml_config)?;
        let toml_config = toml_config.postgres;
        let env_config: Option<RawPostgresParams> = envy::prefixed("PG").from_env().ok();

        // Left-to-right right overrides left
        let configs: Vec<RawPostgresParams> = (&mut [toml_config, env_config])
            .into_iter()
            .filter_map(|x| x.take())
            .collect();

        Ok((&configs[..]).try_into())
    }
}

impl TryFrom<&[RawPostgresParams]> for PostgresParams {
    type Error = PgParamError;

    fn try_from(value: &[RawPostgresParams]) -> std::result::Result<PostgresParams, PgParamError> {
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

        match params {
            RawPostgresParams {
                user: Some(user),
                password: Some(password),
                database: Some(database),
                host: Some(host),
                port: Some(port),
            } => Ok(Self {
                user,
                password,
                host,
                database,
                port,
            }),
            p => Err(PgParamError {
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
