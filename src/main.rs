#[macro_use]
extern crate log;

mod cli;
mod config;
mod db;
mod errors;
mod local;
mod logger;
mod migration;
mod plan_builder;
mod sql;
mod view;

use cli::Opt;
use db::DBExecutor;
use errors::Error;
use local::LocalMigrations;
use migration::Migration;
use plan_builder::{PlanBuilder, PlanType};
use structopt::StructOpt;

fn main() {
    logger::init().unwrap();
    let config = config::load();
    let mut db_exec = DBExecutor::new(config.connection);
    let local = LocalMigrations::new();

    match run(local, db_exec) {
        Ok(_) => (),
        Err(e) => {
            dbg!(e);
        }
    }
}

fn run(mut local: LocalMigrations, mut db_exec: DBExecutor) -> Result<(), Error> {
    match Opt::from_args() {
        Opt::Init {} => {
            local.init().unwrap();
            let local_migrations = local.load_migrations().unwrap();
            debug!("Local migrations: {:?}", local_migrations);
            let plan_type = PlanType::Up(None);
            let mut plan = PlanBuilder::new()
                .set_local_migrations(local_migrations)
                .with_no_db_migrations()
                .build(plan_type);
            db_exec.run_migration_plan(&plan).unwrap();

            Ok(())
        }

        Opt::Generate { name } => {
            local.create_new_migration(&name).unwrap();

            Ok(())
        }

        Opt::Status {} => {
            let mut local_migrations = local
                .load_migrations()
                .map_err(|_| Error::NoMigrationsFolder)?;
            let mut db_migrations = db_exec.load_migrations();
            let mut status = if let Ok(db_migrations) = db_migrations {
                PlanBuilder::new()
                    .set_local_migrations(local_migrations)
                    .set_db_migrations(db_migrations)
                    .get_status()
            } else {
                PlanBuilder::new()
                    .set_local_migrations(local_migrations)
                    .with_no_db_migrations()
                    .get_status()
            };

            view::display_status(&status);

            Ok(())
        }

        Opt::Up { number, show_plan } => {
            let mut local_migrations = local.load_migrations().unwrap();
            let mut db_migrations = db_exec.load_migrations().unwrap();
            let plan_type = PlanType::Up(number);
            let mut plan = PlanBuilder::new()
                .set_local_migrations(local_migrations)
                .set_db_migrations(db_migrations)
                .build(plan_type);

            if show_plan {
                view::display_plan(plan);
            } else {
                db_exec.run_migration_plan(&plan).unwrap();
            }

            Ok(())
        }

        Opt::Down {
            number,
            show_plan,
            ignore_divergent,
        } => {
            let mut local_migrations = local.load_migrations().unwrap();
            let mut db_migrations = db_exec.load_migrations().unwrap();
            let plan_type = PlanType::Down(number, ignore_divergent);
            let mut plan = PlanBuilder::new()
                .set_local_migrations(local_migrations)
                .set_db_migrations(db_migrations)
                .build(plan_type);

            if show_plan {
                view::display_plan(plan);
            } else {
                db_exec.run_migration_plan(&plan).unwrap();
            }

            Ok(())
        }

        Opt::Redo { number, show_plan } => {
            let mut local_migrations = local.load_migrations().unwrap();
            let mut db_migrations = db_exec.load_migrations().unwrap();
            let plan_type = PlanType::Redo(number);
            let mut plan = PlanBuilder::new()
                .set_local_migrations(local_migrations)
                .set_db_migrations(db_migrations)
                .build(plan_type);

            if show_plan {
                view::display_plan(plan);
            } else {
                db_exec.run_migration_plan(&plan).unwrap();
            }

            Ok(())
        }

        _ => unimplemented!(),
    }
}
