use chrono;
use fern;
use log::debug;
use std;

pub fn init() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("postgres", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    debug!("Logger initialized");
    Ok(())
}
