use crate::errors::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use toml;

pub fn load_config() -> Result<Config> {
    let mut file = File::open(&"movine.toml").map_err(|_| Error::ConfigFileNotFound)?;
    let mut config = String::new();
    file.read_to_string(&mut config)?;
    let config: Config = toml::from_str(&config)?;
    Ok(config)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub postgres: Option<PostgresParams>,
    pub sqlite: Option<SqliteParams>,
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
