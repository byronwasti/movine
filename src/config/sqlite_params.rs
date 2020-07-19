use crate::errors::{Error, Result};
use serde::Deserialize; //::{params, Connection, Result};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct SqliteParams {
    pub file: String,
}

impl TryFrom<&[&RawSqliteParams]> for SqliteParams {
    type Error = Error;

    fn try_from(value: &[&RawSqliteParams]) -> Result<SqliteParams> {
        let params = value.iter().fold(RawSqliteParams::default(), |mut acc, x| {
            acc.file = x.file.to_owned().or(acc.file);
            acc
        });

        match params {
            RawSqliteParams { file: Some(file) } => Ok(Self { file }),
            p => Err(Error::SqliteParamError {
                file: p.file.is_some(),
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawSqliteParams {
    pub file: Option<String>,
}

impl RawSqliteParams {
    pub fn load_from_env() -> Result<Self> {
        let params = envy::prefixed("SQLITE_").from_env()?;
        Ok(params)
    }

    pub fn is_any(&self) -> bool {
        self.file.is_some()
    }
}

impl Default for RawSqliteParams {
    fn default() -> Self {
        Self { file: None }
    }
}
