use log::debug;
use movine::adaptor::{PostgresAdaptor, SqliteAdaptor};
use movine::config::Config;
use movine::errors::{Error, Result};
use movine::Movine;
use structopt::StructOpt;

mod cli;
use cli::Opt;

fn main() {
    dotenv::dotenv().ok();
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        },
    }
}

fn run() -> Result<()> {
    match Opt::from_args() {
        Opt::Init { debug } => {
            setup(debug);
            let mut movine = initialize()?;
            movine.initialize()
        }
        Opt::Generate { name, debug } => {
            setup(debug);
            let mut movine = initialize()?;
            movine.generate(&name)
        }
        Opt::Status { debug } => {
            setup(debug);
            let mut movine = initialize()?;
            movine.status()
        }
        Opt::Up {
            number,
            show_plan,
            debug,
            strict,
        } => {
            setup(debug);
            let mut movine = initialize()?;
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
            let mut movine = initialize()?;
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
            let mut movine = initialize()?;
            movine
                .set_number(number)
                .set_ignore_divergent(ignore_divergent)
                .set_show_plan(show_plan)
                .redo()
        }
        Opt::Fix { show_plan, debug } => {
            setup(debug);
            let mut movine = initialize()?;
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

fn initialize() -> Result<Movine> {
    let config = Config::load(&"movine.toml")?;
    debug!("Loaded config");
    let adaptor = match config {
        Config {
            database_url: Some(url),
            ..
        } => {
            if url.starts_with("postgres") {
                PostgresAdaptor::from_url(&url)?
            } else {
                return Err(Error::AdaptorNotFound);
            }
        }
        Config {
            postgres: Some(params),
            ..
        } => PostgresAdaptor::from_params(&params)?,
        Config {
            sqlite: Some(params),
            ..
        } => SqliteAdaptor::from_params(&params)?,
        _ => {
            return Err(Error::AdaptorNotFound);
        }
    };

    Ok(Movine::new(adaptor))
}
