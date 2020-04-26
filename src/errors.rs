use postgres::error::Error as PostgresError;
use rusqlite::Error as SqliteError;
use std::fmt;
use std::io;
use toml::de::Error as TomlError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadPgConfig(String),
    BadSqliteConfig(String),
    ConfigEnvNotDefined,
    ConfigFileNotFound,
    ConfigNotDefined,
    BadMigration,
    Unknown,
    AdaptorNotFound,
    IoError(io::Error),
    TomlError(TomlError),
    PgError(PostgresError),
    SqliteError(SqliteError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            ConfigFileNotFound => write!(f, "`movine.toml` config file not found."),
            BadMigration => write!(f, "Error parsing migrations."),
            Unknown => write!(f, "Unknown error occurred"),
            AdaptorNotFound => write!(f, "Could not find adaptor"),
            IoError(e) => write!(f, "IO Error: {}", e),
            TomlError(e) => write!(f, "Unable to read config file: {}", e),
            PgError(e) => write!(f, "Error in Postgres: {}", e),
            SqliteError(e) => write!(f, "Error in Sqlite: {}", e),
            _ => todo!(),
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
