use movine::adaptor::{DbAdaptor, PostgresAdaptor, SqliteAdaptor};
use movine::cli::Opt;
use movine::config::Config;
use movine::errors::{Error, Result};
use movine::Movine;
use structopt::StructOpt;

mod logger;

fn main() {
    logger::init().expect("Could not initialize the logger.");
    dotenv::dotenv().ok();
    match run() {
        Ok(()) => {}
        Err(e) => println!("Error: {}", e),
    }
}

fn run() -> Result<()> {
    let config = Config::load(&"movine.toml")?;
    let adaptor = match config {
        Config {
            database_url: Some(url),
            ..
        } => {
            if url.starts_with("postgres") {
                Box::new(PostgresAdaptor::from_url(&url)?) as Box<dyn DbAdaptor>
            } else {
                return Err(Error::AdaptorNotFound);
            }
        }
        Config {
            postgres: Some(params),
            ..
        } => Box::new(PostgresAdaptor::from_params(&params)?) as Box<dyn DbAdaptor>,
        Config {
            sqlite: Some(params),
            ..
        } => Box::new(SqliteAdaptor::from_params(&params)?) as Box<dyn DbAdaptor>,
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
