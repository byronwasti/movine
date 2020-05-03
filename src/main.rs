use movine::adaptor::{PostgresAdaptor, SqliteAdaptor};
use movine::cli::Opt;
use movine::config::Config;
use movine::errors::{Error, Result};
use movine::Movine;
use structopt::StructOpt;
use log::debug;

mod logger;

fn main() {
    logger::init().expect("Could not initialize the logger.");
    dotenv::dotenv().ok();
    match run() {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run() -> Result<()> {
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
    let mut movine = Movine::new(adaptor);
    run_from_args(&mut movine)
}

fn run_from_args(movine: &mut Movine) -> Result<()> {
    match Opt::from_args() {
        Opt::Init {} => movine.initialize(),
        Opt::Generate { name } => movine.generate(&name),
        Opt::Status {} => movine.status(),
        Opt::Up { number, show_plan } => movine.set_number(number).set_show_plan(show_plan).up(),
        Opt::Down {
            number,
            show_plan,
            ignore_divergent,
        } => movine
            .set_number(number)
            .set_show_plan(show_plan)
            .set_ignore_divergent(ignore_divergent)
            .down(),
        Opt::Redo { number, show_plan } => {
            movine.set_number(number).set_show_plan(show_plan).redo()
        }
        Opt::Fix { show_plan } => movine.set_show_plan(show_plan).fix(),
        _ => unimplemented!(),
    }
}
