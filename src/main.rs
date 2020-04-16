#[macro_use]
extern crate log;
use chrono::prelude::*;
use structopt::StructOpt;

mod adaptor;
mod cli;
mod config;
mod display;
mod errors;
mod helpers;
mod logger;
mod match_maker;
mod migration;
mod plan_builder;

use cli::Opt;
use errors::Result;
use migration::MigrationBuilder;
use plan_builder::PlanBuilder;

fn main() {
    logger::init().expect("Could not initialize the logger.");
    match run() {
        Ok(()) => {}
        Err(e) => println!("Error: {}", e),
    }
}

fn run() -> Result<()> {
    match Opt::from_args() {
        Opt::Init {} => initialize(),
        Opt::Generate { name } => generate(&name),
        Opt::Status {} => status(),
        Opt::Up { number, show_plan } => up(number, show_plan),
        Opt::Down {
            number,
            show_plan,
            ignore_divergent,
        } => down(number, show_plan, ignore_divergent),
        Opt::Redo { number, show_plan } => redo(number, show_plan),
        Opt::Fix { show_plan } => fix(show_plan),
        _ => unimplemented!(),
    }
}

fn initialize() -> Result<()> {
    let config = config::load_config()?;
    helpers::create_migration_directory()?;
    let mut adaptor = adaptor::get_adaptor(&config)?;
    let up_sql = adaptor.init_up_sql();
    let down_sql = adaptor.init_down_sql();

    let init_migration = MigrationBuilder::new()
        .name(&"movine_init")
        .date(Utc.timestamp(0, 0))
        .up_sql(&up_sql)
        .down_sql(&down_sql)
        .build()?;

    helpers::write_migration(&init_migration)?;

    // Can't just call to `up` function since we are unable to get
    // database migrations until we run this migration.
    let local_migrations = helpers::load_local_migrations()?;
    let db_migrations = Vec::new();
    let plan = PlanBuilder::new()
        .local_migrations(&local_migrations)
        .db_migrations(&db_migrations)
        .up()?;
    adaptor.run_migration_plan(&plan)
}

fn generate(name: &str) -> Result<()> {
    let new_migration = MigrationBuilder::new()
        .name(name)
        .date(Utc::now())
        .build()?;
    helpers::write_migration(&new_migration)
}

fn status() -> Result<()> {
    let config = config::load_config()?;
    let adaptor = adaptor::get_adaptor(&config)?;
    let local_migrations = helpers::load_local_migrations()?;
    let db_migrations = adaptor.load_migrations()?;

    let status = PlanBuilder::new()
        .local_migrations(&local_migrations)
        .db_migrations(&db_migrations)
        .status()?;

    display::print_status(&status);
    Ok(())
}

fn up(number: Option<usize>, show_plan: bool) -> Result<()> {
    let config = config::load_config()?;
    let mut adaptor = adaptor::get_adaptor(&config)?;
    let local_migrations = helpers::load_local_migrations()?;
    let db_migrations = adaptor.load_migrations()?;

    let plan = PlanBuilder::new()
        .local_migrations(&local_migrations)
        .db_migrations(&db_migrations)
        .count(number)
        .up()?;

    if show_plan {
        display::print_plan(&plan);
        Ok(())
    } else {
        adaptor.run_migration_plan(&plan)
    }
}

fn down(number: Option<usize>, show_plan: bool, _ignore_divergent: bool) -> Result<()> {
    let config = config::load_config()?;
    let mut adaptor = adaptor::get_adaptor(&config)?;
    let local_migrations = helpers::load_local_migrations()?;
    let db_migrations = adaptor.load_migrations()?;

    let plan = PlanBuilder::new()
        .local_migrations(&local_migrations)
        .db_migrations(&db_migrations)
        .count(number)
        .down()?;

    if show_plan {
        display::print_plan(&plan);
        Ok(())
    } else {
        adaptor.run_migration_plan(&plan)
    }
}

fn fix(show_plan: bool) -> Result<()> {
    let config = config::load_config()?;
    let mut adaptor = adaptor::get_adaptor(&config)?;
    let local_migrations = helpers::load_local_migrations()?;
    let db_migrations = adaptor.load_migrations()?;

    let plan = PlanBuilder::new()
        .local_migrations(&local_migrations)
        .db_migrations(&db_migrations)
        .fix()?;

    if show_plan {
        display::print_plan(&plan);
        Ok(())
    } else {
        adaptor.run_migration_plan(&plan)
    }
}

fn redo(number: Option<usize>, show_plan: bool) -> Result<()> {
    let config = config::load_config()?;
    let mut adaptor = adaptor::get_adaptor(&config)?;
    let local_migrations = helpers::load_local_migrations()?;
    let db_migrations = adaptor.load_migrations()?;

    let plan = PlanBuilder::new()
        .local_migrations(&local_migrations)
        .db_migrations(&db_migrations)
        .count(number)
        .redo()?;

    if show_plan {
        display::print_plan(&plan);
        Ok(())
    } else {
        adaptor.run_migration_plan(&plan)
    }
}
