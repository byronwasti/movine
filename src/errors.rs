use postgres::error::Error as PostgresError;
use rusqlite::Error as SqliteError;
use std::fmt;
use std::io;
use toml::de::Error as TomlError;

mod pg_errors;
mod sqlite_errors;

pub use pg_errors::PgParamError;
pub use sqlite_errors::SqliteParamError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadConfiguration(ConfigError),
    AdaptorNotFound,
    BadMigration,
    Unknown,
    IoError(io::Error),
    TomlError(TomlError),
    PgError(PostgresError),
    SqliteError(SqliteError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            //ConfigFileNotFound => write!(f, "`movine.toml` config file not found."),
            BadMigration => write!(f, "Error parsing migrations."),
            Unknown => write!(f, "Unknown error occurred"),
            AdaptorNotFound => write!(f, "Could not find adaptor"),
            IoError(e) => write!(f, "IO Error: {}", e),
            TomlError(e) => write!(f, "Unable to read config file: {}", e),
            PgError(e) => write!(f, "Error in Postgres: {}", e),
            SqliteError(e) => write!(f, "Error in Sqlite: {}", e),
            BadConfiguration(conf_err) => write!(f, "Error evaluating config: {}", conf_err),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Postgres(PgParamError),
    Sqlite(SqliteParamError),
    NoConfigFound,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Postgres(err) => {
                write!(f, "Error in PostgreSQL configuration: {:?}", err)
            }
            ConfigError::Sqlite(err) => {
                write!(f, "Error in SQLite configuration: {:?}", err)
            }
            ConfigError::NoConfigFound => {
                write!(f, "Could not load any config. Make sure you supply a `movine.toml` file or environment variables.")
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<TomlError> for Error {
    fn from(error: TomlError) -> Self {
        Error::TomlError(error)
    }
}

impl From<PostgresError> for Error {
    fn from(error: PostgresError) -> Self {
        Error::PgError(error)
    }
}

impl From<SqliteError> for Error {
    fn from(error: SqliteError) -> Self {
        Error::SqliteError(error)
    }
}

impl From<SqliteParamError> for Error {
    fn from(error: SqliteParamError) -> Self {
        Error::BadConfiguration(ConfigError::Sqlite(error))
    }
}

impl From<PgParamError> for Error {
    fn from(error: PgParamError) -> Self {
        Error::BadConfiguration(ConfigError::Postgres(error))
    }
}

impl From<ConfigError> for Error {
    fn from(error: ConfigError) -> Self {
        Error::BadConfiguration(error)
    }
}
