use movine::config::Config;
use movine::errors::Result;
use movine::DbAdaptor;
use movine::Movine;
use structopt::StructOpt;

mod cli;
use cli::Opt;

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Init { debug } => {
            let mut movine = setup(debug)?;
            movine.initialize()
        }
        Opt::Generate { name, debug } => {
            let mut movine = setup(debug)?;
            movine.generate(&name)
        }
        Opt::Status { debug } => {
            let mut movine = setup(debug)?;
            movine.status()
        }
        Opt::Up {
            number,
            show_plan,
            debug,
            strict,
        } => {
            let mut movine = setup(debug)?;
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
            ignore_unreversable,
            debug,
        } => {
            let mut movine = setup(debug)?;
            movine
                .set_number(number)
                .set_show_plan(show_plan)
                .set_ignore_divergent(ignore_divergent)
                .set_ignore_unreversable(ignore_unreversable)
                .down()
        }
        Opt::Redo {
            number,
            show_plan,
            ignore_divergent,
            ignore_unreversable,
            debug,
        } => {
            let mut movine = setup(debug)?;
            movine
                .set_number(number)
                .set_ignore_divergent(ignore_divergent)
                .set_ignore_unreversable(ignore_unreversable)
                .set_show_plan(show_plan)
                .redo()
        }
        Opt::Fix { show_plan, debug } => {
            let mut movine = setup(debug)?;
            movine.set_show_plan(show_plan).fix()
        }
        _ => unimplemented!(),
    }
}

fn setup(debug: bool) -> Result<Movine<Box<dyn DbAdaptor>>> {
    dotenv::dotenv().ok();
    env_logger::builder()
        .filter_level(if debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    let config = Config::load(&"movine.toml")?;
    let adaptor = config.into_db_adaptor()?;
    let movine = Movine::new(adaptor);
    Ok(movine)
}
