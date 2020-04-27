#[derive(Debug)]
pub struct PgParamError {
    pub user: bool,
    pub database: bool,
    pub password: bool,
    pub host: bool,
    pub port: bool,
}

impl PgParamError {
    pub fn is_partial(&self) -> bool {
        self.user || self.database || self.password || self.host || self.port
    }
}
