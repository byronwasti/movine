use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use crate::errors::Error;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: ConnectionParams,
}

#[derive(Debug, Deserialize)]
pub struct ConnectionParams {
    pub host: String,
    pub database: String,
    pub user: String,
    pub password: Option<String>,
    pub port: u16,
}

pub fn load() -> Result<Config, Error> {
    let mut file = File::open(&"movine.toml")?;
    let mut config_toml = String::new();
    file.read_to_string(&mut config_toml)?;
    let config: Config = toml::from_str(&config_toml)?;
    return Ok(config);
}
