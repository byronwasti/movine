use serde::Deserialize;
use std::fs::File;
use std::io::Read;
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

pub fn load() -> Config {
    let mut file = File::open(&"movine.toml").unwrap();
    let mut config_toml = String::new();
    file.read_to_string(&mut config_toml).unwrap();
    let config: Config = toml::from_str(&config_toml).unwrap();
    return config;
}
