use crate::adaptor::DbAdaptor;
use crate::errors::{Error, Result};
use crate::migration::{Migration, MigrationBuilder};
use crate::plan_builder::Step;
use postgres;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostgresParams {
    pub username: String,
    pub password: String,
    pub host: String,
    pub database: String,
    pub port: i32,
}

pub struct PostgresAdaptor {
    conn: postgres::Connection,
}

impl PostgresAdaptor {
    pub fn new(params: &PostgresParams) -> Result<Self> {
        let connection_params = format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}",
            user = params.username,
            password = params.password,
            host = params.host,
            port = params.port,
            database = params.database,
        );
        let conn = postgres::Connection::connect(connection_params, postgres::TlsMode::None)?;
        Ok(Self { conn })
    }
}

impl DbAdaptor for PostgresAdaptor {
    fn init_up_sql(&self) -> &'static str {
        INIT_UP_SQL
    }

    fn init_down_sql(&self) -> &'static str {
        INIT_DOWN_SQL
    }

    fn load_migrations(&self) -> Result<Vec<Migration>> {
        let mut migrations = Vec::new();
        let sql = "
            SELECT name, hash, down_sql
            FROM movine_migrations
            ORDER BY created_at DESC;
        ";
        let rows = self.conn.query(sql, &[])?;
        for row in &rows {
            let name: String = row.get(0);
            let hash: String = row.get(1);
            let down_sql: String = row.get(2);
            let migration = MigrationBuilder::new()
                .compound_name(&name)
                .hash(&hash)
                .down_sql(&down_sql)
                .build()?;
            migrations.push(migration);
        }
        Ok(migrations)
    }

    fn run_migration_plan(&mut self, plan: &[(Step, &Migration)]) -> Result<()> {
        for (step, migration) in plan {
            match step {
                Step::Up => {
                    let name = &migration.name;
                    let hash = migration.hash.as_ref().ok_or_else(|| Error::BadMigration)?;
                    let up_sql = migration
                        .up_sql
                        .as_ref()
                        .ok_or_else(|| Error::BadMigration)?;
                    let empty_string = "".to_string();
                    let down_sql = migration.down_sql.as_ref().unwrap_or_else(|| &empty_string);

                    let transaction = self.conn.transaction()?;
                    transaction.batch_execute(&up_sql)?;
                    transaction.execute(LOG_UP_MIGRATION, &[&name, &hash, &down_sql])?;
                    transaction.commit()?;
                }
                Step::Down => {
                    let name = &migration.name;
                    let down_sql = migration
                        .down_sql
                        .as_ref()
                        .ok_or_else(|| Error::BadMigration)?;

                    let transaction = self.conn.transaction()?;
                    transaction.batch_execute(&down_sql)?;
                    transaction.execute(LOG_DOWN_MIGRATION, &[&name])?;
                    transaction.commit()?;
                }
            }
        }
        Ok(())
    }
}

pub const LOG_UP_MIGRATION: &str = "\
INSERT INTO movine_migrations (name, hash, down_sql)
VALUES ($1, $2, $3);
";

pub const LOG_DOWN_MIGRATION: &str = "\
DELETE FROM movine_migrations 
WHERE name = $1;
";

pub const INIT_UP_SQL: &str = "\
CREATE TABLE movine_migrations (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),
    name TEXT NOT NULL,
    hash TEXT NOT NULL,
    down_sql TEXT
);
";

pub const INIT_DOWN_SQL: &str = "\
DROP TABLE movine_migrations;
";
