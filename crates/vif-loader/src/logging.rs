use crate::CONFIG;

pub fn setup_logging() {
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
        .level(CONFIG.log_level)
        .chain(fern::log_file("/tmp/vif.log").unwrap());

    if CONFIG.debug {
        log_builder = log_builder.chain(std::io::stdout());
    };

    log_builder.apply().unwrap();
}
