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
    pub meta: Meta,
    pub connection: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub database: String,
}
