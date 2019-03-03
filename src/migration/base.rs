use crate::migration::{DbMigration, LocalMigration};
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Migration {
    name: String,
    date: DateTime<Utc>,
    pub up_sql: Option<String>,
    pub down_sql: Option<String>,
    pub up_hash: Option<String>,
    pub down_hash: Option<String>,
}

impl Migration {
    pub fn new(name: &str) -> Self {
        let (date, _) = name.split_at(name.find('_').unwrap());
        let date = Utc.datetime_from_str(&date, "%Y-%m-%d-%H%M%S").unwrap();
        Self {
            name: name.to_string(),
            date,
            up_sql: None,
            up_hash: None,
            down_sql: None,
            down_hash: None,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_date(&self) -> DateTime<Utc> {
        self.date
    }
}

impl From<DbMigration> for Migration {
    fn from(db_migration: DbMigration) -> Self {
        db_migration.destruct()
    }
}

impl From<LocalMigration> for Migration {
    fn from(local_migration: LocalMigration) -> Self {
        local_migration.destruct()
    }
}
