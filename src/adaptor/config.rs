use crate::errors::{ConfigError, Result};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

mod pg_params;
mod sqlite_params;

pub use pg_params::PostgresParams;
pub use sqlite_params::SqliteParams;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub postgres: Option<PostgresParams>,
    pub sqlite: Option<SqliteParams>,
}

impl Config {
    pub fn load(file: &str) -> Result<Self> {
        let mut toml_config = String::new();
        let file = File::open(file);
        if let Ok(mut file) = file {
            file.read_to_string(&mut toml_config)?;
        }

        let pg = PostgresParams::load(&toml_config);
        let sqlite = SqliteParams::load(&toml_config);

        match (pg, sqlite) {
            (Err(pg_e), Err(_sq_e)) => Err(pg_e),
            (Ok(Err(pg_e)), Ok(Err(_sq_e))) => {
                if pg_e.is_partial() {
                    Err(pg_e.into())
                } else {
                    Err(ConfigError::NoConfigFound.into())
                }
            }
            (Ok(pg), Ok(sq)) => Ok(Self {
                postgres: pg.ok(),
                sqlite: sq.ok(),
            }),
            (Ok(pg), _) => Ok(Self {
                postgres: pg.ok(),
                sqlite: None,
            }),
            (_, Ok(sq)) => Ok(Self {
                postgres: None,
                sqlite: sq.ok(),
            }),
        }
    }
}
