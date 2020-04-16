use movine::config::{Adaptor, Config};
use movine::errors::Result;
use movine::logger;
use movine::Movine;

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
    match adaptor {
        Adaptor::Postgres(adaptor) => {
            let mut movine = Movine::new(adaptor, &"./migrations");
            movine.run_from_args()
        }
        Adaptor::Sqlite(adaptor) => {
            let mut movine = Movine::new(adaptor, &"./migrations");
            movine.run_from_args()
        }
    }
}
