use crate::adaptor::{PostgresAdaptor, SqliteAdaptor};
use crate::errors::{Error, Result};
use serde::Deserialize;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use toml;

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
        let mut file = File::open(file).map_err(|_| Error::ConfigFileNotFound)?;
        let mut toml_config = String::new();
        file.read_to_string(&mut toml_config)?;

        let pg = PostgresParams::load(&toml_config);
        let sqlite = SqliteParams::load(&toml_config);

        if pg.is_err() && sqlite.is_err() {
            Err(Error::ConfigNotDefined)
        } else {
            Ok(Self {
                postgres: pg.ok(),
                sqlite: sqlite.ok(),
            })
        }
    }
}
