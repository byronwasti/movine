use movine::adaptor::DbAdaptor;
use movine::cli::Opt;
use movine::config::{Adaptor, Config};
use movine::errors::Result;
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
    let config = Config::from_file(&"movine.toml").or_else(|error| match Config::from_env() {
        Some(config) => Ok(config),
        None => Err(error),
    })?;
    let adaptor = config.into_adaptor()?;
    match adaptor {
        Adaptor::Postgres(adaptor) => {
            let mut movine = Movine::new(adaptor);
            run_from_args(&mut movine)
        }
        Adaptor::Sqlite(adaptor) => {
            let mut movine = Movine::new(adaptor);
            run_from_args(&mut movine)
        }
    }
}

fn run_from_args<T: DbAdaptor>(movine: &mut Movine<T>) -> Result<()> {
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
