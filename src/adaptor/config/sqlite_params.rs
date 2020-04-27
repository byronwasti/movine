use crate::errors::{Result, SqliteParamError};
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Deserialize)]
pub struct SqliteParams {
    pub file: String,
}

impl SqliteParams {
    pub fn load(toml_config: &str) -> Result<std::result::Result<Self, SqliteParamError>> {
        let toml_config: RawConfig = toml::from_str(toml_config)?;
        let toml_config = toml_config.sqlite;
        let env_config: Option<RawSqliteParams> = envy::prefixed("SQLITE_").from_env().ok();
        let configs: Vec<RawSqliteParams> = (&mut [toml_config, env_config])
            .into_iter()
            .filter_map(|x| x.take())
            .collect();

        Ok((&configs[..]).try_into())
    }
}

impl TryFrom<&[RawSqliteParams]> for SqliteParams {
    type Error = SqliteParamError;

    fn try_from(value: &[RawSqliteParams]) -> std::result::Result<SqliteParams, SqliteParamError> {
        let params = value.iter().fold(RawSqliteParams::default(), |mut acc, x| {
            acc.file = x.file.to_owned().or(acc.file);
            acc
        });

        match params {
            RawSqliteParams { file: Some(file) } => Ok(Self { file }),
            p => Err(SqliteParamError {
                file: p.file.is_some(),
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    sqlite: Option<RawSqliteParams>,
}

#[derive(Debug, Deserialize)]
struct RawSqliteParams {
    pub file: Option<String>,
}

impl Default for RawSqliteParams {
    fn default() -> Self {
        Self { file: None }
    }
}
