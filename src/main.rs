use movine::cli::Opt;
use movine::config::Config;
use movine::errors::Result;
use movine::logger;
use movine::Movine;
use structopt::StructOpt;

fn main() {
    logger::init().expect("Could not initialize the logger.");
    match run() {
        Ok(()) => {}
        Err(e) => println!("Error: {}", e),
    }
}

fn run() -> Result<()> {
    let config = Config::from_file(&"movine.toml")?;
    let adaptor = config.into_adaptor()?;
    let mut movine = Movine::new(adaptor, &"./migrations");
    match Opt::from_args() {
        Opt::Init {} => movine.initialize(),
        Opt::Generate { name } => movine.generate(&name),
        Opt::Status {} => movine.status(),
        Opt::Up { number, show_plan } => movine.up(number, show_plan),
        Opt::Down {
            number,
            show_plan,
            ignore_divergent,
        } => movine.down(number, show_plan, ignore_divergent),
        Opt::Redo { number, show_plan } => movine.redo(number, show_plan),
        Opt::Fix { show_plan } => movine.fix(show_plan),
        _ => unimplemented!(),
    }
}
