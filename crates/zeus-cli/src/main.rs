mod application;
mod cli;
mod config;
mod error;

fn setup_logging() {
    let debug = std::env::var("DEBUG").and(Ok(true)).unwrap_or(false);

    let level = std::env::var("ZEUS_LOG_LEVEL")
        .map(|lvl| lvl.parse().unwrap())
        .unwrap_or(if debug {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Error
        });

    // Separate file config so we can include year, month and day in file logs
    let mut log_builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} - {} - {:<30} \t{}",
                chrono::Local::now().format("%H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(fern::log_file("/tmp/zeus.log").unwrap());

    if debug {
        log_builder = log_builder.chain(std::io::stdout());
    };

    log_builder.apply().unwrap();
}

fn main() {
    setup_logging();
    let config = config::get_config();

    let mut zeus = application::Zeus::init();

    let res = match config.entrypoint {
        Some(path) => zeus.run_file(path),
        _ => zeus.run_prompt(),
    };

    match res {
        Ok(_) => (),
        Err(e) => println!("Error: {e}"),
    }
}
