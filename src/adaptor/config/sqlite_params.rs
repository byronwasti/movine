use crate::errors::{Error, Result};
use serde::Deserialize;
use std::convert::{TryInto, TryFrom};

#[derive(Debug, Deserialize)]
pub struct SqliteParams {
    pub file: String,
}

impl SqliteParams {
    pub fn load(toml_config: &str) -> Result<Self> {
        let toml_config: RawConfig = toml::from_str(toml_config)?;
        let toml_config = toml_config.sqlite;
        let env_config: Option<RawSqliteParams> = envy::prefixed("SQLITE_").from_env().ok();

        // Left-to-right right overrides left
        let configs: Vec<RawSqliteParams> = (&mut [toml_config, env_config])
            .into_iter()
            .filter_map(|x| x.take())
            .collect();

        (&configs[..]).try_into()
    }
}

impl TryFrom<&[RawSqliteParams]> for SqliteParams {
    type Error = Error;

    fn try_from(value: &[RawSqliteParams]) -> Result<SqliteParams> {
        let params = value.iter().fold(RawSqliteParams::default(), |mut acc, x| {
            acc.file = x.file.to_owned().or(acc.file);
            acc
        });

        Ok(Self {
            file: params
                .file
                .ok_or(Error::BadSqliteConfig("No file".to_string()))?,
        })
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
