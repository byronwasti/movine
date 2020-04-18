use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostgresParams {
    pub user: String,
    pub password: String,
    pub host: String,
    pub database: String,
    pub port: i32,
}
