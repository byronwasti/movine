use movine::adaptor::Adaptor;
use movine::cli::Opt;
use movine::errors::Result;
use movine::Movine;
use structopt::StructOpt;

fn main() {
    dotenv::dotenv().ok();
    match run_from_args() {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run_from_args() -> Result<()> {
    match Opt::from_args() {
        Opt::Init {debug} => {
            init_logging(debug);
            init_movine()?
                .initialize()
        },
        Opt::Generate { name, debug } => {
            init_logging(debug);
            init_movine()?
                .generate(&name)
        },
        Opt::Status {debug} => {
            init_logging(debug);
            init_movine()?
                .status()
        },
        Opt::Up { number, show_plan, debug } => {
            init_logging(debug);
            init_movine()?
                .set_number(number)
                .set_show_plan(show_plan)
                .up()
        },
        Opt::Down {
            number,
            show_plan,
            debug,
            ignore_divergent,
        } => {
            init_logging(debug);
            init_movine()?
                .set_number(number)
                .set_show_plan(show_plan)
                .set_ignore_divergent(ignore_divergent)
                .down()
        }
        Opt::Redo { number, show_plan, debug } => {
            init_logging(debug);
            init_movine()?
                .set_number(number)
                .set_show_plan(show_plan)
                .redo()
        }
        Opt::Fix { show_plan, debug } => {
            init_logging(debug);
            init_movine()?
                .set_show_plan(show_plan)
                .fix()
        }
        _ => unimplemented!(),
    }
}

fn init_movine() -> Result<Movine> {
    let adaptor = Adaptor::load()?;
    Ok(Movine::new(adaptor))
}

fn init_logging(verbose: bool) {
    if verbose {
        env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    } else {
        env_logger::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }
}
