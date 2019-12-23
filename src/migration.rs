use crate::errors::{Error, Result};
use chrono::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq)]
pub struct Migration {
    pub name: String,
    pub up_sql: Option<String>,
    pub down_sql: Option<String>,
    pub hash: Option<String>,
}

pub struct MigrationBuilder {
    compound_name: Option<String>,
    name: Option<String>,
    date: Option<DateTime<Utc>>,
    up_sql: Option<String>,
    down_sql: Option<String>,
    hash: Option<String>,
}

impl MigrationBuilder {
    pub fn new() -> Self {
        Self {
            compound_name: None,
            name: None,
            date: None,
            up_sql: None,
            down_sql: None,
            hash: None,
        }
    }

    pub fn compound_name<'a>(&'a mut self, compound_name: &str) -> &'a mut Self {
        self.compound_name = Some(compound_name.to_owned());
        self
    }

    pub fn name<'a>(&'a mut self, name: &str) -> &'a mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn date<'a>(&'a mut self, date: DateTime<Utc>) -> &'a mut Self {
        self.date = Some(date.to_owned());
        self
    }

    pub fn up_sql<'a>(&'a mut self, up_sql: &str) -> &'a mut Self {
        self.up_sql = Some(up_sql.to_owned());
        self
    }

    pub fn down_sql<'a>(&'a mut self, down_sql: &str) -> &'a mut Self {
        self.down_sql = Some(down_sql.to_owned());
        self
    }

    pub fn hash<'a>(&'a mut self, hash: &str) -> &'a mut Self {
        self.hash = Some(hash.to_owned());
        self
    }

    pub fn build(&self) -> Result<Migration> {
        // TODO: Clean up ownership a bit; we should be able to just take
        let name = if let Some(compound_name) = &self.compound_name {
            compound_name.to_owned()
        } else {
            let name = self.name.to_owned().ok_or(Error::BadMigration)?;
            let date = self.date.to_owned().ok_or(Error::BadMigration)?;
            let date = date.format("%Y-%m-%d-%H%M%S").to_string();
            format!("{}_{}", date, name)
        };

        let hash = match (&self.up_sql, &self.down_sql, &self.hash) {
            (_, _, Some(x)) => Some(x.to_owned()),
            (x, y, None) => {
                let mut hasher = DefaultHasher::new();
                x.hash(&mut hasher);
                y.hash(&mut hasher);
                Some(format!("{:x}", hasher.finish()))
            }
        };

        Ok(Migration {
            name,
            up_sql: self.up_sql.to_owned(),
            down_sql: self.down_sql.to_owned(),
            hash,
        })
    }
}
