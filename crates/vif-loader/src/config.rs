use crate::cli;
use lazy_static::lazy_static;
use std::fs::canonicalize;
use std::path::PathBuf;

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

pub struct Config {
    pub debug: bool,
    pub log_level: log::LevelFilter,
    pub entrypoint: Option<PathBuf>,
    pub assembly: bool,
}

pub fn get_config() -> Config {
    let cli = cli::get_cli();
    let matches = cli.get_matches();

    let entrypoint = matches
        .get_one::<String>("entrypoint")
        .and_then(|path| Some(canonicalize(path).expect("Could not parse the provided path")));

    let assembly = matches.get_flag("assembly");
    let debug = std::env::var("DEBUG").and(Ok(true)).unwrap_or(false);
    let log_level = std::env::var("VIF_LOG_LEVEL")
        .map(|lvl| lvl.parse().unwrap())
        .unwrap_or(if debug {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Error
        });

    return Config {
        entrypoint,
        debug,
        log_level,
        assembly,
    };
}
