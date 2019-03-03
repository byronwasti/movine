use crate::migration::Migration;
use chrono::prelude::*;
use postgres::rows::Row;

#[derive(Debug)]
pub struct DbMigration(Migration);

impl DbMigration {
    pub fn from_row(row: &Row) -> Self {
        let name: String = row.get(0);
        let mut migration = Migration::new(&name);
        migration.up_hash = row.get(1);
        migration.down_hash = row.get(2);
        migration.down_sql = row.get(3);
        Self(migration)
    }

    pub fn get_name(&self) -> String {
        self.0.get_name()
    }

    pub fn get_date(&self) -> DateTime<Utc> {
        self.0.get_date()
    }

    pub fn get_up_hash(&self) -> Option<String> {
        self.0.up_hash.clone()
    }

    pub fn get_down_hash(&self) -> Option<String> {
        self.0.down_hash.clone()
    }
    pub fn destruct(self) -> Migration {
        self.0
    }
}
