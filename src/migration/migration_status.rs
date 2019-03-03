use crate::migration::{DbMigration, LocalMigration};
use chrono::prelude::*;
use std::fmt;

#[derive(Debug)]
pub enum MigrationStatus {
    Applied(LocalMigration),
    Pending(LocalMigration),
    Divergent(DbMigration),
    Variant(DbMigration, LocalMigration),
}

impl MigrationStatus {
    pub fn get_date(&self) -> DateTime<Utc> {
        use MigrationStatus::*;
        match self {
            Applied(m) => m.get_date(),
            Pending(m) => m.get_date(),
            Divergent(m) => m.get_date(),
            Variant(m, _) => m.get_date(),
        }
    }

    pub fn get_name(&self) -> String {
        use MigrationStatus::*;
        match self {
            Applied(m) => m.get_name(),
            Pending(m) => m.get_name(),
            Divergent(m) => m.get_name(),
            Variant(m, _) => m.get_name(),
        }
    }
}

impl fmt::Display for MigrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MigrationStatus::*;
        write!(
            f,
            "{}",
            match self {
                Applied(_) => "Applied  ",
                Pending(_) => "Pending  ",
                Divergent(_) => "Divergent",
                Variant(_, _) => "Variant  ",
            }
        )
    }
}
