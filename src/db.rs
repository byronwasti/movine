use postgres::{Connection, TlsMode, Result};
use crate::config::ConnectionParams;

pub struct DBExecutor {
    conn: Connection,
}

impl DBExecutor {
    pub fn new(conn_params: ConnectionParams) -> Self {
        let connection_params = format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}",
            user = conn_params.user, 
            password = conn_params.password.unwrap_or("".to_string()),
            host = conn_params.host,
            port = conn_params.port,
            database = conn_params.database,
            );
        let conn = Connection::connect(connection_params, TlsMode::None).unwrap();
        Self {
            conn,
        }
    }

    pub fn run_sql(&mut self, sql: &str) -> Result<()> {
        let transaction = self.conn.transaction()?;
        transaction.batch_execute(sql)?;
        transaction.commit()?;
        Ok(())
    }
}

