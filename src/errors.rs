use postgres::error::Error as PostgresError;
use toml::de::Error as TomlError;
use std::io;

#[derive(Debug)]
pub enum Error {
    NoMigrationsFolder,
    IoError(io::Error),
    PgError(PostgresError),
    TomlError(TomlError),
}

impl From<TomlError> for Error {
    fn from(error: TomlError) -> Self {
        Error::TomlError(error)
    }
}


impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<PostgresError> for Error {
    fn from(error: PostgresError) -> Self {
        Error::PgError(error)
    }
}
