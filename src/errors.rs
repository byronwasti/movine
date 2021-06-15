use libsqlite3_sys::Error as SqliteLibError;
use libsqlite3_sys::ErrorCode as SqliteLibErrorCode;
use postgres::error::Error as PostgresError;
use rusqlite::Error as SqliteError;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use toml::de::Error as TomlError;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    ConfigNotFound,
    PgParamError {
        user: bool,
        password: bool,
        database: bool,
        host: bool,
        port: bool,
    },
    SqliteParamError {
        file: bool,
    },
    BadMigration,
    Unknown,
    AdaptorNotFound,
    MigrationDirNotFound,
    DirtyMigrations,
    DivergentMigration,
    UnrollbackableMigration,
    IoError(io::Error),
    TomlError(TomlError),
    PgError(PostgresError),
    SqliteError(SqliteError),
    Envy(envy::Error),
    #[cfg(feature = "with-native-tls")]
    NativeTlsError(native_tls::Error),
    #[cfg(feature = "with-rustls")]
    RustlsError(rustls::TLSError),
    #[cfg(feature = "with-rustls")]
    RustlsPemfileError,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            ConfigNotFound => write!(f, "`movine.toml` config file not found and no environment variables were found."),
            BadMigration => write!(f, "Error parsing migrations."),
            Unknown => write!(f, "Unknown error occurred"),
            AdaptorNotFound => write!(f, "Could not find adaptor"),
            MigrationDirNotFound => write!(f, "Could not find migration directory"),
            DirtyMigrations => write!(f, "More recent migrations exist in the database than the pending migrations. This is an error when run with --strict"),
            DivergentMigration => write!(f, "Divergent migration found. Run with --ignore-divergent to ignore divergent migrations."),
            UnrollbackableMigration => write!(f, "Can't rollback one of the migrations in the list. Consider changing your parameters or adding a `down.sql` migration."),
            IoError(e) => write!(f, "IO Error: {}", e),
            TomlError(e) => write!(f, "Unable to read config file: {}", e),
            PgError(e) => write!(f, "Error in Postgres: {}", e),
            SqliteError(e) => {
                match e {
                    rusqlite::Error::SqliteFailure(SqliteLibError { code: SqliteLibErrorCode::APIMisuse, .. }, _) => {
                        write!(f, "Error in Sqlite: {}.", e)?;
                        write!(f, "\nThis is likely due to an invalid SQL statement, such as an empty UP or DOWN migration.")
                    }
                    _ => write!(f, "Error in Sqlite: {}", e),
                }
            }
            Envy(e) => write!(f, "Error in loading environment variables: {}", e),
            #[cfg(feature = "with-native-tls")]
            NativeTlsError(e) => write!(f, "Error in TLS: {}", e),
            #[cfg(feature = "with-rustls")]
            RustlsError(e) => write!(f, "Error in TLS: {}", e),
            #[cfg(feature = "with-rustls")]
            RustlsPemfileError => write!(f, "Error in TLS: could not add PEM file to store"),
            SqliteParamError { .. } => write!(f, "Unable to load Sqlite params. Make sure you have `file` defined in your `movine.toml` or SQLITE_FILE defined as an environment variable"),
            PgParamError {
                user, password, database, host, port
            } => {
                write!(f, "Unable to load Postgres params. Please ensure you have the following defined:\nUser: {}\nPassword: {}\nDatabase: {}\nHost: {}\nPort: {}",
                    user, password, database, host, port)
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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

impl From<envy::Error> for Error {
    fn from(error: envy::Error) -> Self {
        Error::Envy(error)
    }
}

#[cfg(feature = "with-native-tls")]
impl From<native_tls::Error> for Error {
    fn from(error: native_tls::Error) -> Self {
        Error::NativeTlsError(error)
    }
}

#[cfg(feature = "with-rustls")]
impl From<rustls::TLSError> for Error {
    fn from(error: rustls::TLSError) -> Self {
        Error::RustlsError(error)
    }
}

// Implements std::Error for ease of use outside of Movine
impl StdError for Error {}
