use crate::errors::{Error, Result};
use crate::migration::{Migration, MigrationBuilder};
use crate::plan_builder::Step;
use crate::config::Config;
use std::collections::HashMap;

mod pg_adaptor;
mod sqlite_adaptor;

use pg_adaptor::PostgresAdaptor;
use sqlite_adaptor::SqliteAdaptor;

pub fn get_adaptor(config: &Config) -> Result<Box<dyn DbAdaptor>> {
    match config {
        Config { postgres: Some(params), ..} => {
            let pg = PostgresAdaptor::new(&params)?;
            Ok(Box::new(pg))
        }
        Config { sqlite: Some(params), ..} => {
            let sqlite = SqliteAdaptor::new(&params)?;
            Ok(Box::new(sqlite))
        }
        _ => Err(Error::AdaptorNotFound),
    }
}

pub trait DbAdaptor {
    fn init_up_sql(&self) -> &'static str;
    fn init_down_sql(&self) -> &'static str;
    fn load_migrations(&self) -> Result<Vec<Migration>>;
    fn run_migration_plan(&mut self, plan: &[(Step, &Migration)]) -> Result<()>;
}

