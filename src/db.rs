use crate::config::ConnectionParams;
use crate::migration::{DbMigration, Migration, MigrationOp, MigrationPlan};
use crate::errors::Error;
use crate::sql::{LOG_DOWN_MIGRATION, LOG_UP_MIGRATION};
use postgres::{Connection, Result as PgResult, TlsMode};

pub struct DBExecutor {
    conn: Connection,
}

impl DBExecutor {
    pub fn new(conn_params: ConnectionParams) -> Result<Self, Error> {
        let connection_params = format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}",
            user = conn_params.user,
            password = conn_params.password.unwrap_or("".to_string()),
            host = conn_params.host,
            port = conn_params.port,
            database = conn_params.database,
        );
        let conn = Connection::connect(connection_params, TlsMode::None)?;
        Ok(Self { conn })
    }

    pub fn run_migration_plan(&mut self, migration_plan: &MigrationPlan) -> PgResult<()> {
        for migration_op in migration_plan {
            debug!("{:?}", migration_op);
            match migration_op {
                (MigrationOp::Up, migration) => {
                    let sql = &migration.up_sql;
                    if let Some(sql) = sql {
                        let transaction = self.conn.transaction()?;
                        transaction.batch_execute(&sql)?;
                        transaction.commit()?;
                        self.conn.execute(
                            LOG_UP_MIGRATION,
                            &[
                                &migration.get_name(),
                                &migration.up_hash,
                                &migration.down_hash,
                                &migration.down_sql,
                            ],
                        )?;
                    } else {
                        error!("Something went wrong");
                    }
                    println!("Up {}", &migration.get_name());
                }
                (MigrationOp::Down, migration) => {
                    let sql = &migration.down_sql;
                    if let Some(sql) = sql {
                        let transaction = self.conn.transaction()?;
                        transaction.batch_execute(&sql)?;
                        transaction.commit()?;
                        self.conn
                            .execute(LOG_DOWN_MIGRATION, &[&migration.get_name()])?;
                    } else {
                        error!("Something went wrong 2");
                    }
                    println!("Down {}", &migration.get_name());
                }
            }
        }
        Ok(())
    }

    pub fn load_migrations(&self) -> PgResult<Vec<DbMigration>> {
        let mut migrations = Vec::new();
        let sql = "
            SELECT name, up_hash, down_hash, down_sql
            FROM movine_migrations
            ORDER BY created_at DESC;
        ";
        let rows = self.conn.query(sql, &[])?;
        for row in &rows {
            let migration = DbMigration::from_row(&row);
            migrations.push(migration);
        }

        Ok(migrations)
    }
}
