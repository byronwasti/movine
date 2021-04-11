use crate::adaptor::DbAdaptor;
use crate::errors::{Error, Result};
use crate::migration::{Migration, MigrationBuilder};
use rusqlite::{params, Connection};

impl DbAdaptor for Connection {
    fn init_up_sql(&self) -> &'static str {
        INIT_UP_SQL
    }

    fn init_down_sql(&self) -> &'static str {
        INIT_DOWN_SQL
    }

    fn load_migrations(&mut self) -> Result<Vec<Migration>> {
        let mut migrations = Vec::new();
        let sql = "
            SELECT name, hash, down_sql
            FROM movine_migrations
            ORDER BY created_at DESC;
        ";
        let mut stmt = self.prepare(&sql)?;
        let rows: std::result::Result<Vec<(String, String, String)>, _> = stmt
            .query_map(params![], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect();
        let rows: Vec<(String, String, String)> = rows.unwrap();

        for row in rows {
            let name: String = row.0;
            let hash: String = row.1;
            let down_sql: String = row.2;
            let migration = MigrationBuilder::new()
                .compound_name(&name)
                .hash(&hash)
                .down_sql(&down_sql)
                .build()?;
            migrations.push(migration);
        }
        Ok(migrations)
    }

    fn run_up_migration(&mut self, migration: &Migration) -> Result<()> {
        let name = &migration.name;
        let hash = migration.hash.as_ref().ok_or(Error::BadMigration)?;
        let up_sql = migration
            .up_sql
            .as_ref()
            .ok_or(Error::BadMigration)?;
        let empty_string = "".to_string();
        let down_sql = migration.down_sql.as_ref().unwrap_or(&empty_string);

        let transaction = self.transaction()?;
        transaction.execute_batch(&up_sql)?;
        transaction.execute(LOG_UP_MIGRATION, &[&name, &hash, &down_sql])?;
        transaction.commit()?;
        Ok(())
    }

    fn run_down_migration(&mut self, migration: &Migration) -> Result<()> {
        let name = &migration.name;
        let down_sql = migration
            .down_sql
            .as_ref()
            .ok_or(Error::BadMigration)?;

        let transaction = self.transaction()?;
        transaction.execute_batch(&down_sql)?;
        transaction.execute(LOG_DOWN_MIGRATION, &[&name])?;
        transaction.commit()?;
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
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    name TEXT NOT NULL,
    hash TEXT NOT NULL,
    down_sql TEXT
);
";

pub const INIT_DOWN_SQL: &str = "\
DROP TABLE movine_migrations;
";
