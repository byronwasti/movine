use crate::errors::{Result};
use crate::migration::Migration;
use crate::plan_builder::Step;

mod pg_adaptor;
mod sqlite_adaptor;

pub use pg_adaptor::PostgresAdaptor;
pub use sqlite_adaptor::SqliteAdaptor;

pub trait DbAdaptor {
    fn init_up_sql(&self) -> &'static str;
    fn init_down_sql(&self) -> &'static str;
    fn load_migrations(&self) -> Result<Vec<Migration>>;
    fn run_migration_plan(&mut self, plan: &[(Step, &Migration)]) -> Result<()>;
}
