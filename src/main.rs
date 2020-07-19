use log::debug;
use movine::config::{Config, DbAdaptorKind};
use movine::errors::{Error, Result};
use movine::DbAdaptor;
use movine::Movine;
use structopt::StructOpt;

mod cli;
use cli::Opt;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let config = Config::load(&"movine.toml")?;
    let adaptor = config.into_db_adaptor()?;
    match adaptor {
        DbAdaptorKind::Postgres(mut conn) => run(&mut conn),
        DbAdaptorKind::Sqlite(mut conn) => run(&mut conn),
    }
}

fn run<T: DbAdaptor>(adaptor: T) -> Result<()> {
    let mut movine = Movine::new(adaptor);
    match Opt::from_args() {
        Opt::Init { debug } => {
            setup(debug);
            movine.initialize()
        }
        Opt::Generate { name, debug } => {
            setup(debug);
            movine.generate(&name)
        }
        Opt::Status { debug } => {
            setup(debug);
            movine.status()
        }
        Opt::Up {
            number,
            show_plan,
            debug,
            strict,
        } => {
            setup(debug);
            movine
                .set_number(number)
                .set_strict(strict)
                .set_show_plan(show_plan)
                .up()
        }
        Opt::Down {
            number,
            show_plan,
            ignore_divergent,
            debug,
        } => {
            setup(debug);
            movine
                .set_number(number)
                .set_show_plan(show_plan)
                .set_ignore_divergent(ignore_divergent)
                .down()
        }
        Opt::Redo {
            number,
            show_plan,
            ignore_divergent,
            debug,
        } => {
            setup(debug);
            movine
                .set_number(number)
                .set_ignore_divergent(ignore_divergent)
                .set_show_plan(show_plan)
                .redo()
        }
        Opt::Fix { show_plan, debug } => {
            setup(debug);
            movine.set_show_plan(show_plan).fix()
        }
        _ => unimplemented!(),
    }
}

fn setup(debug: bool) {
    env_logger::builder()
        .filter_level(if debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init()
}
