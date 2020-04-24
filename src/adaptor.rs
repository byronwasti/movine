use crate::errors::Result;
use crate::migration::Migration;
use crate::plan_builder::Step;

mod postgres;
mod sqlite;

pub use self::postgres::PostgresAdaptor;
pub use sqlite::SqliteAdaptor;

pub trait DbAdaptor {
    fn init_up_sql(&self) -> &'static str;
    fn init_down_sql(&self) -> &'static str;
    fn load_migrations(&self) -> Result<Vec<Migration>>;
    fn run_up_migration(&mut self, migration: &Migration) -> Result<()>;
    fn run_down_migration(&mut self, migration: &Migration) -> Result<()>;

    fn run_migration_plan(&mut self, plan: &[(Step, &Migration)]) -> Result<()> {
        for (step, migration) in plan {
            match step {
                Step::Up => {
                    println!("Running up.sql for {}", &migration.name);
                    self.run_up_migration(&migration)?;
                }
                Step::Down => {
                    println!("Running down.sql for {}", &migration.name);
                    self.run_down_migration(&migration)?;
                }
            }
        }
        Ok(())
    }
}
