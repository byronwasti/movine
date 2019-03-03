use crate::migration::Migration;
use chrono::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct LocalMigration(Migration);

impl LocalMigration {
    pub fn new(name: &str) -> Self {
        Self(Migration::new(name))
    }

    pub fn set_up_sql(&mut self, sql: &str) {
        self.0.up_sql = Some(sql.to_string());

        let mut hasher = DefaultHasher::new();
        sql.hash(&mut hasher);
        self.0.up_hash = Some(format!("{:x}", hasher.finish()));
    }

    pub fn get_up_hash(&self) -> Option<String> {
        self.0.up_hash.clone()
    }

    pub fn get_down_hash(&self) -> Option<String> {
        self.0.down_hash.clone()
    }

    pub fn set_down_sql(&mut self, sql: &str) {
        self.0.down_sql = Some(sql.to_string());
        let mut hasher = DefaultHasher::new();
        sql.hash(&mut hasher);
        self.0.down_hash = Some(format!("{:x}", hasher.finish()));
    }

    pub fn get_name(&self) -> String {
        self.0.get_name()
    }

    pub fn get_date(&self) -> DateTime<Utc> {
        self.0.get_date()
    }

    pub fn destruct(self) -> Migration {
        self.0
    }
}
