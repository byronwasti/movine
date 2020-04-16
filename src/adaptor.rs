use crate::errors::{Error, Result};
use crate::migration::{Migration, MigrationBuilder};
use crate::plan_builder::Step;
use std::collections::HashMap;

mod pg_adaptor;
mod sqlite_adaptor;

use pg_adaptor::PostgresAdaptor;
use sqlite_adaptor::SqliteAdaptor;

pub fn get_adaptor(database: &str, connection: &HashMap<String, String>) -> Result<Box<dyn DbAdaptor>> {
    match database {
        "postgres" => {
            let pg = PostgresAdaptor::new(connection)?;
            Ok(Box::new(pg))
        }
        "sqlite" => {
            let sqlite = SqliteAdaptor::new(connection)?;
            Ok(Box::new(sqlite))
        }
        x => Err(Error::AdaptorNotFound(x.to_string())),
    }
}

pub trait DbAdaptor {
    fn init_up_sql(&self) -> &'static str;
    fn init_down_sql(&self) -> &'static str;
    fn load_migrations(&self) -> Result<Vec<Migration>>;
    fn run_migration_plan(&mut self, plan: &[(Step, &Migration)]) -> Result<()>;
}

