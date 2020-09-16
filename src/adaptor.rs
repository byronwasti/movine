use crate::display;
use crate::errors::Result;
use crate::migration::Migration;
use crate::plan_builder::Step;

mod postgres;
mod sqlite;

pub trait DbAdaptor {
    fn init_up_sql(&self) -> &'static str;
    fn init_down_sql(&self) -> &'static str;
    fn load_migrations(&mut self) -> Result<Vec<Migration>>;
    fn run_up_migration(&mut self, migration: &Migration) -> Result<()>;
    fn run_down_migration(&mut self, migration: &Migration) -> Result<()>;

    fn run_migration_plan(&mut self, plan: &[(Step, &Migration)]) -> Result<()> {
        for (step, migration) in plan {
            display::print_step(&(*step, migration));
            match step {
                Step::Up => {
                    self.run_up_migration(&migration)?;
                }
                Step::Down => {
                    if migration.is_reversable() {
                        self.run_down_migration(&migration)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: DbAdaptor + ?Sized> DbAdaptor for &'_ mut T {
    fn init_up_sql(&self) -> &'static str {
        (**self).init_up_sql()
    }

    fn init_down_sql(&self) -> &'static str {
        (**self).init_down_sql()
    }

    fn load_migrations(&mut self) -> Result<Vec<Migration>> {
        (**self).load_migrations()
    }

    fn run_up_migration(&mut self, migration: &Migration) -> Result<()> {
        (**self).run_up_migration(migration)
    }

    fn run_down_migration(&mut self, migration: &Migration) -> Result<()> {
        (**self).run_down_migration(migration)
    }

    fn run_migration_plan(&mut self, plan: &[(Step, &Migration)]) -> Result<()> {
        (**self).run_migration_plan(plan)
    }
}

impl<T: DbAdaptor + ?Sized> DbAdaptor for Box<T> {
    fn init_up_sql(&self) -> &'static str {
        (**self).init_up_sql()
    }

    fn init_down_sql(&self) -> &'static str {
        (**self).init_down_sql()
    }

    fn load_migrations(&mut self) -> Result<Vec<Migration>> {
        (**self).load_migrations()
    }

    fn run_up_migration(&mut self, migration: &Migration) -> Result<()> {
        (**self).run_up_migration(migration)
    }

    fn run_down_migration(&mut self, migration: &Migration) -> Result<()> {
        (**self).run_down_migration(migration)
    }

    fn run_migration_plan(&mut self, plan: &[(Step, &Migration)]) -> Result<()> {
        (**self).run_migration_plan(plan)
    }
}
