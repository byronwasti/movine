use chrono::prelude::*;

pub mod adaptor;
pub mod cli;
pub mod config;
mod display;
pub mod errors;
mod file_handler;
pub mod logger;
mod match_maker;
mod migration;
mod plan_builder;

pub use config::Config;
use adaptor::DbAdaptor;
use errors::Result;
use file_handler::FileHandler;
use migration::MigrationBuilder;
use plan_builder::PlanBuilder;

pub struct Movine<T: DbAdaptor> {
    adaptor: T,
    migration_dir: String,
    number: Option<usize>,
    show_plan: bool,
    ignore_divergent: bool,
}

impl<T: DbAdaptor> Movine<T> {
    pub fn new(adaptor: T) -> Self {
        Self {
            adaptor,
            migration_dir: "./migrations".into(),
            number: None,
            show_plan: false,
            ignore_divergent: false,
        }
    }

    pub fn set_migration_dir<'a>(&'a mut self, migration_dir: &str) -> &'a mut Self {
        self.migration_dir = migration_dir.into();
        self
    }

    pub fn set_number<'a>(&'a mut self, number: Option<usize>) -> &'a mut Self {
        self.number = number;
        self
    }

    pub fn set_show_plan<'a>(&'a mut self, show_plan: bool) -> &'a mut Self {
        self.show_plan = show_plan;
        self
    }

    pub fn set_ignore_divergent<'a>(&'a mut self, ignore_divergent: bool) -> &'a mut Self {
        self.ignore_divergent = ignore_divergent;
        self
    }

    pub fn initialize(&mut self) -> Result<()> {
        let file_handler = FileHandler::new(&self.migration_dir);
        file_handler.create_migration_directory()?;
        let up_sql = self.adaptor.init_up_sql();
        let down_sql = self.adaptor.init_down_sql();

        let init_migration = MigrationBuilder::new()
            .name(&"movine_init")
            .date(Utc.timestamp(0, 0))
            .up_sql(&up_sql)
            .down_sql(&down_sql)
            .build()?;

        file_handler.write_migration(&init_migration)?;

        // Can't just call to `up` function since we are unable to get
        // database migrations until we run this migration.
        let local_migrations = file_handler.load_local_migrations()?;
        let db_migrations = Vec::new();
        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .up()?;
        self.adaptor.run_migration_plan(&plan)
    }

    pub fn generate(&mut self, name: &str) -> Result<()> {
        let file_handler = FileHandler::new(&self.migration_dir);
        let new_migration = MigrationBuilder::new()
            .name(name)
            .date(Utc::now())
            .build()?;
        file_handler.write_migration(&new_migration)
    }

    pub fn status(&mut self) -> Result<()> {
        let file_handler = FileHandler::new(&self.migration_dir);
        let local_migrations = file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let status = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .status()?;

        display::print_status(&status);
        Ok(())
    }

    pub fn up(&mut self) -> Result<()> {
        let file_handler = FileHandler::new(&self.migration_dir);
        let local_migrations = file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .count(self.number)
            .up()?;

        if self.show_plan {
            display::print_plan(&plan);
            Ok(())
        } else {
            self.adaptor.run_migration_plan(&plan)
        }
    }

    pub fn down(&mut self) -> Result<()> {
        let file_handler = FileHandler::new(&self.migration_dir);
        let local_migrations = file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .count(self.number)
            .down()?;

        if self.show_plan {
            display::print_plan(&plan);
            Ok(())
        } else {
            self.adaptor.run_migration_plan(&plan)
        }
    }

    pub fn fix(&mut self) -> Result<()> {
        let file_handler = FileHandler::new(&self.migration_dir);
        let local_migrations = file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .fix()?;

        if self.show_plan {
            display::print_plan(&plan);
            Ok(())
        } else {
            self.adaptor.run_migration_plan(&plan)
        }
    }

    pub fn redo(&mut self) -> Result<()> {
        let file_handler = FileHandler::new(&self.migration_dir);
        let local_migrations = file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .count(self.number)
            .redo()?;

        if self.show_plan {
            display::print_plan(&plan);
            Ok(())
        } else {
            self.adaptor.run_migration_plan(&plan)
        }
    }
}
