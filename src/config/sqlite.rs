use serde::Deserialize; //::{params, Connection, Result};

#[derive(Debug, Deserialize)]
pub struct SqliteParams {
    pub file: String,
}
