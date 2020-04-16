#[macro_use]
extern crate log;
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

use adaptor::DbAdaptor;
use errors::Result;
use file_handler::FileHandler;
use migration::MigrationBuilder;
use plan_builder::PlanBuilder;

pub struct Movine {
    adaptor: Box<dyn DbAdaptor>,
    file_handler: FileHandler,
}

impl Movine {
    pub fn new(adaptor: Box<dyn DbAdaptor>) -> Self {
        Self {
            adaptor,
            file_handler: FileHandler::new(&"./migrations"),
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.file_handler.create_migration_directory()?;
        let up_sql = self.adaptor.init_up_sql();
        let down_sql = self.adaptor.init_down_sql();

        let init_migration = MigrationBuilder::new()
            .name(&"movine_init")
            .date(Utc.timestamp(0, 0))
            .up_sql(&up_sql)
            .down_sql(&down_sql)
            .build()?;

        self.file_handler.write_migration(&init_migration)?;

        // Can't just call to `up` function since we are unable to get
        // database migrations until we run this migration.
        let local_migrations = self.file_handler.load_local_migrations()?;
        let db_migrations = Vec::new();
        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .up()?;
        self.adaptor.run_migration_plan(&plan)
    }

    pub fn generate(&mut self, name: &str) -> Result<()> {
        let new_migration = MigrationBuilder::new()
            .name(name)
            .date(Utc::now())
            .build()?;
        self.file_handler.write_migration(&new_migration)
    }

    pub fn status(&mut self) -> Result<()> {
        let local_migrations = self.file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let status = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .status()?;

        display::print_status(&status);
        Ok(())
    }

    pub fn up(&mut self, number: Option<usize>, show_plan: bool) -> Result<()> {
        let local_migrations = self.file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .count(number)
            .up()?;

        if show_plan {
            display::print_plan(&plan);
            Ok(())
        } else {
            self.adaptor.run_migration_plan(&plan)
        }
    }

    pub fn down(
        &mut self,
        number: Option<usize>,
        show_plan: bool,
        _ignore_divergent: bool,
    ) -> Result<()> {
        let local_migrations = self.file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .count(number)
            .down()?;

        if show_plan {
            display::print_plan(&plan);
            Ok(())
        } else {
            self.adaptor.run_migration_plan(&plan)
        }
    }

    pub fn fix(&mut self, show_plan: bool) -> Result<()> {
        let local_migrations = self.file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .fix()?;

        if show_plan {
            display::print_plan(&plan);
            Ok(())
        } else {
            self.adaptor.run_migration_plan(&plan)
        }
    }

    pub fn redo(&mut self, number: Option<usize>, show_plan: bool) -> Result<()> {
        let local_migrations = self.file_handler.load_local_migrations()?;
        let db_migrations = self.adaptor.load_migrations()?;

        let plan = PlanBuilder::new()
            .local_migrations(&local_migrations)
            .db_migrations(&db_migrations)
            .count(number)
            .redo()?;

        if show_plan {
            display::print_plan(&plan);
            Ok(())
        } else {
            self.adaptor.run_migration_plan(&plan)
        }
    }
}
